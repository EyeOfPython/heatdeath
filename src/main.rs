mod world;
mod vector2;

use std::f64::consts::PI;

use piston_window::*;
use ordered_float::NotNan;
use vector2::Vector2;

use crate::world::{Circle, Constraint, World};


fn main() -> Result<(), Box<std::error::Error>> {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true).build().unwrap();
    let mut world = World::new();

//    let c1 = world.add_circle(
//        Circle::new(NotNan::new(1.0)?, Vector2::new_prim(100.0, 70.0)?),
//    );
    for i in 0..(640 / (Circle::RADIUS as i32 * 2) + 1) {
        world.add_circle(
            Circle::new(
                NotNan::new(0.0)?,
                Vector2::new_prim(i as f64 * Circle::RADIUS * 2.0, 480.0)?,
            ),
        );
    }
    //world.add_constraint(
    //    Constraint::CircleDistance(c1, c2, NotNan::new(30.0)?),
    //);
    let mut cursor = Vector2::zero();
    let mut grab_circle = None;
    let mut is_holding_ctrl = false;
    let mut is_holding_p = false;
    let mut collisions = Vec::new();
    while let Some(event) = window.next() {
        if is_holding_p {
            world.run_physics_pre(NotNan::new(0.5)?);
            collisions = world.find_collisions();
            world.run_physics(NotNan::new(0.5)?, 10, &collisions);
        }
        let event: Event = event;
        event.mouse_cursor(|x, y| {
            cursor = Vector2::new_prim(x, y).unwrap();
        });
        if let Some(Button::Keyboard(Key::LCtrl)) = event.press_args() {
            is_holding_ctrl = true;
        }
        if let Some(Button::Keyboard(Key::P)) = event.press_args() {
            is_holding_p = !is_holding_p;
        }
        if let Some(Button::Keyboard(Key::S)) = event.press_args() {
            world.run_physics_pre(NotNan::new(0.1)?);
            collisions = world.find_collisions();
            world.run_physics(NotNan::new(0.1)?, 10, &collisions);
        }
        if let Some(Button::Keyboard(Key::LCtrl)) = event.release_args() {
            is_holding_ctrl = false;
        }
        if let Some(Button::Mouse(button)) = event.press_args() {
            if let Some((key, _)) = world.circle_at(cursor) {
                grab_circle = Some(key);
            } else {
                world.add_circle(Circle::new(NotNan::new(1.0)?, cursor));
            }
        }
        if let Some(Button::Mouse(button)) = event.release_args() {
            if is_holding_ctrl && grab_circle.is_some() {
                if let Some((key, _)) = world.circle_at(cursor) {
                    let circle1 = world.circle(grab_circle.unwrap()).unwrap();
                    let circle2 = world.circle(key).unwrap();
                    let distance = circle1.p().distance(circle2.p());
                    world.add_constraint(
                        Constraint::CircleDistance(grab_circle.unwrap(), key, distance),
                    );
                }
            }
            grab_circle = None;
        }
        if let Some(key) = grab_circle {
            if !is_holding_ctrl {
                world.move_circle(key, cursor).unwrap();
            }
        }
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            for (_, v) in world.circles() {
                circle_arc(
                    [0.5, 0.6, 1.0, 1.0],
                    1.0,
                    0.0, PI * 1.99999,
                    [
                        v.x_prim() - Circle::RADIUS, v.y_prim() - Circle::RADIUS,
                        Circle::RADIUS * 2.0, Circle::RADIUS * 2.0,
                    ],
                    context.transform,
                    graphics,
                );
            }
            for (_, c) in world.constraints() {
                match c {
                    Constraint::CircleDistance(key1, key2, _) => {
                        let c1 = world.circle(*key1).unwrap();
                        let c2 = world.circle(*key2).unwrap();
                        line([0.3, 0.7, 0.4, 1.0],
                             1.0,
                             [c1.x_prim(), c1.y_prim(), c2.x_prim(), c2.y_prim()],
                            context.transform,
                            graphics,
                        );
                    }
                }
            }
            for collision in collisions.iter() {
                let c1 = world.circle(collision.circle_key1).unwrap();
                let c2 = world.circle(collision.circle_key2).unwrap();
                let p: Vector2 = collision.entry_p;
                let n = collision.entry_p + NotNan::new(10.0).unwrap() * collision.normal;
                let r = 3.0;
                circle_arc(
                    [0.9, 0.6, 1.0, 1.0],
                    0.6,
                    0.0, PI * 1.99999,
                    [
                        p.x().into_inner() - r, p.y().into_inner() - r,
                        r * 2.0, r * 2.0,
                    ],
                    context.transform,
                    graphics,
                );
                line([0.9, 0.0, 0.4, 1.0],
                     0.6,
                     [p.x().into_inner(), p.y().into_inner(),
                         n.x().into_inner(), n.y().into_inner()],
                     context.transform,
                     graphics,
                );
            }
        });
    }

    Ok(())
}
