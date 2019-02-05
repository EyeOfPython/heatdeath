use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign};
use std::fmt;

use ordered_float::{NotNan, FloatIsNaN};

pub type ScalarPrimitive = f64;
pub type Scalar = NotNan<ScalarPrimitive>;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
pub struct Vector2 {
    x: Scalar,
    y: Scalar,
}

impl Vector2 {
    pub fn zero() -> Vector2 {
        Vector2 {
            x: Scalar::new(0.0).unwrap(),
            y: Scalar::new(0.0).unwrap(),
        }
    }

    pub fn one() -> Vector2 {
        Vector2 {
            x: Scalar::new(1.0).unwrap(),
            y: Scalar::new(1.0).unwrap(),
        }
    }

    pub fn new(x: Scalar, y: Scalar) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn new_prim(x_prim: ScalarPrimitive, y_prim: ScalarPrimitive) -> Result<Vector2, FloatIsNaN> {
        Ok(Vector2 {
            x: Scalar::new(x_prim)?,
            y: Scalar::new(y_prim)?,
        })
    }

    pub fn x(self) -> Scalar { self.x }
    pub fn y(self) -> Scalar { self.y }

    pub fn distance_sq(self, other: Vector2) -> Scalar {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx*dx + dy*dy
    }

    pub fn distance(self, other: Vector2) -> Scalar {
        Scalar::new(self.distance_sq(other).sqrt()).unwrap()
    }

    pub fn dot(self, other: Vector2) -> Scalar {
        self.x * other.x + self.y * other.y
    }

    pub fn norm(self) -> Scalar {
        self.distance(Vector2::zero())
    }

    pub fn normalized(self) -> Vector2 {
        self / self.norm()
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<Vector2> for Scalar {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Mul<Scalar> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Scalar) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<Scalar> for Vector2 {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Div<Scalar> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Scalar) -> Vector2 {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<Scalar> for Vector2 {
    fn div_assign(&mut self, rhs: NotNan<f64>) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
