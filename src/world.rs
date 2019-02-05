use slotmap::{HopSlotMap, new_key_type};
use crate::vector2::{Scalar, Vector2};


pub struct World {
    circles: HopSlotMap<CircleKey, Circle>,
    constraints: HopSlotMap<ConstraintKey, Constraint>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Circle {
    inv_mass: Scalar,
    p: Vector2,
    future_p: Vector2,
    v: Vector2,
}

new_key_type! {
    pub struct CircleKey;
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Constraint {
    CircleDistance(CircleKey, CircleKey, Scalar),
}

new_key_type! {
    pub struct ConstraintKey;
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Collision {
    pub circle_key1: CircleKey,
    pub circle_key2: CircleKey,
    pub entry_p: Vector2,
    pub normal: Vector2,
    pub correction: Vector2,
}

impl World {
    pub fn new() -> World {
        World {
            circles: HopSlotMap::with_key(),
            constraints: HopSlotMap::with_key(),
        }
    }

    pub fn add_circle(&mut self, circle: Circle) -> CircleKey {
        self.circles.insert(circle)
    }

    pub fn add_constraint(&mut self, constraint: Constraint) -> ConstraintKey {
        self.constraints.insert(constraint)
    }

    pub fn circles(&self) -> impl Iterator<Item=(CircleKey, &Circle)> {
        self.circles.iter()
    }

    pub fn circle_at(&self, p: Vector2) -> Option<(CircleKey, &Circle)> {
        self.circles().find(|(_, circle)| {
            circle.contains(p)
        })
    }

    pub fn circle(&self, key: CircleKey) -> Option<&Circle> {
        self.circles.get(key)
    }

    pub fn move_circle(&mut self, key: CircleKey, p: Vector2) -> Result<(), ()> {
        let circle = self.circles.get_mut(key).ok_or(())?;
        circle.p = p;
        Ok(())
    }

    pub fn constraints(&self) -> impl Iterator<Item=(ConstraintKey, &Constraint)> {
        self.constraints.iter()
    }

    pub fn find_collisions(&self) -> Vec<Collision> {
        use std::collections::HashSet;
        let mut collisions = Vec::new();
        let mut visited = HashSet::new();
        for (c1, circle1) in self.circles.iter() {
            for (c2, circle2) in self.circles.iter() {
                if c1 == c2 || (visited.contains(&(c2, c1)) && false) {
                    continue;
                }
                let normal = (circle2.p - circle1.p).normalized();
                let correction: Vector2 = circle1.radius() * normal;
                    // ^ start ray from closest point on circle1
                let p1 = circle1.p + correction;
                if circle2.future_p.distance_sq(p1) < circle2.radius() * circle2.radius() {
                    let closest_surface_point = circle2.p - normal * circle2.radius();
                    collisions.push(Collision {
                        circle_key1: c1,
                        circle_key2: c2,
                        entry_p: closest_surface_point,
                        normal: -normal,
                        correction,
                    });
                    visited.insert((c1, c2));
                    continue;
                }
                let p2 = circle1.future_p + correction;
                let d = p2 - p1;
                let f = p1 - circle2.p;
                let a = d.dot(d);
                let b = Scalar::new(2.0).unwrap() * f.dot(d);
                let c = f.dot(f) - circle2.radius() * circle2.radius();
                let discriminant = b*b - Scalar::new(4.0).unwrap() * a*c;
                visited.insert((c1, c2));
                if discriminant <= Scalar::new(0.0).unwrap() {
                    continue;
                }
                let discriminant = Scalar::new(discriminant.sqrt()).unwrap();
                let t1 = (-b - discriminant) / (Scalar::new(2.0).unwrap()*a);
                //let t2 = (-b + discriminant) / (2*a);
                if t1 >= Scalar::new(0.0).unwrap() && t1 <= Scalar::new(1.0).unwrap() {
                    let entry_p = p1 + t1 * d;
                    collisions.push(Collision { circle_key1: c1, circle_key2: c2, entry_p, normal: -normal, correction });
                }
            }
        }
        collisions
    }

    pub fn run_physics_pre(&mut self, dt: Scalar) {
        self.circles.iter_mut().for_each(|(_, circle)| {
            if circle.inv_mass > Scalar::new(0.0).unwrap() {
                circle.v += Vector2::new(
                    Scalar::new(0.0).unwrap(),
                    Scalar::new(0.4).unwrap() * dt,
                );
            }
        });
        self.circles.iter_mut().for_each(|(_, circle)| {
            circle.v *= Scalar::new(0.99).unwrap();
        });
        self.circles.iter_mut().for_each(|(_, circle)| {
            circle.future_p = circle.p + circle.v * dt;
        });
    }

    pub fn run_physics(&mut self, dt: Scalar, n_solver_loops: usize, collisions: &[Collision]) {
        //let collisions = self.find_collisions();
        for i in 0..n_solver_loops {
            for (_, constraint) in self.constraints.iter() {
                match constraint {
                    Constraint::CircleDistance(c1, c2, d) => {
                        let d = *d;
                        let circle1 = self.circles.get(*c1).unwrap();
                        let circle2 = self.circles.get(*c2).unwrap();
                        let dist = circle1.future_p.distance(circle2.future_p);
                        if (dist - d).into_inner() >= 0.0 { continue }
                        let weight_factor1 = -circle1.inv_mass / (circle1.inv_mass + circle2.inv_mass);
                        let weight_factor2 = circle2.inv_mass / (circle1.inv_mass + circle2.inv_mass);

                        let delta1 = weight_factor1 * (dist - d) * (circle1.future_p - circle2.future_p) / dist;
                        let delta2 = weight_factor2 * (dist - d) * (circle1.future_p - circle2.future_p) / dist;

                        let circle1 = self.circles.get_mut(*c1).unwrap();
                        circle1.future_p += delta1;
                        let circle2 = self.circles.get_mut(*c2).unwrap();
                        circle2.future_p += delta2;
                    }
                }
            }
            for collision in collisions.iter() {
                let circle2_inv_mass = self.circles
                    .get(collision.circle_key2).unwrap().inv_mass;
                let circle1 = self.circles.get_mut(collision.circle_key1).unwrap();
                let delta = -(circle1.future_p + collision.correction - collision.entry_p).dot(collision.normal) * collision.normal;
                circle1.future_p += circle1.inv_mass / (circle1.inv_mass + circle2_inv_mass) * delta;
            }
        }
        self.circles.iter_mut().for_each(|(_, circle)| {
            circle.v = (circle.future_p - circle.p) / dt;
            circle.p = circle.future_p;
        });
    }
}

impl Circle {
    pub const RADIUS: f64 = 10.0;

    pub fn new(inv_mass: Scalar, p: Vector2) -> Circle {
        Circle {
            inv_mass,
            p,
            future_p: p,
            v: Vector2::zero(),
        }
    }

    pub fn p(&self) -> Vector2 {
        self.p
    }

    pub fn x_prim(&self) -> f64 {
        self.p.x().into_inner()
    }

    pub fn y_prim(&self) -> f64 {
        self.p.y().into_inner()
    }

    pub fn radius(&self) -> Scalar {
        Scalar::new(Self::RADIUS).unwrap()
    }

    pub fn contains(&self, p: Vector2) -> bool {
        self.p.distance_sq(p) < self.radius() * self.radius()
    }
}
