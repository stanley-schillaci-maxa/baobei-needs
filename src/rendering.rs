//! Component and systems permitting to render an entity to the screen.

#![allow(clippy::wildcard_enum_match_arm)]

use crate::{collisions::BoxCollider, physics::Position};
use ggez::{graphics, nalgebra::Point2, Context, GameResult};
use graphics::{Color, DrawParam, Image, Mesh, Rect};
use legion::prelude::*;

/// Component for rendering an entity
#[derive(Clone, Debug, PartialEq)]
pub struct Rendering {
    /// Sprite to render
    pub sprite: Image,
    /// Drawing info e.g. position or scale
    pub param: DrawParam,
    /// Order of the sprite, low = background and high = foreground.
    pub order: i32,
}

/// Update the position of the sprite depending on the entity position
pub fn update(world: &mut World) {
    <(Read<Position>, Write<Rendering>)>::query()
        .filter(changed::<Position>()) // only changed positions
        .iter_mut(world)
        .for_each(|(pos, mut render)| {
            render.param = render.param.dest(to_point(*pos));
        })
}

/// Converts a position to a point.
///
/// Note: This is a convenience to manage version conflicts between
/// `ggez::nalgebra` (0.18.1) and `ncollide2d::nalgebra` (0.21.1)
fn to_point(pos: Position) -> Point2<f32> {
    Point2::new(pos.x, pos.y)
}

/// Draws all entities with a rendering component.
/// It renders them following the `order` field.
pub fn draw(ctx: &mut Context, world: &World) -> GameResult {
    let query = Read::<Rendering>::query();

    let mut renders: Vec<_> = query.iter(world).collect();
    renders.sort_by_key(|r| r.order);

    for render in renders {
        graphics::draw(ctx, &render.sprite, render.param)?;
    }
    Ok(())
}

/// The green RGB color.
const GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};

/// Draws all box colliders with a green rectangle.
pub fn draw_colliders(ctx: &mut Context, world: &World) -> GameResult {
    for (pos, col) in <(Read<Position>, Read<BoxCollider>)>::query().iter(world) {
        let rect = Rect::new(pos.x, pos.y, col.width, col.height);

        let mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(2.0), rect, GREEN)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;
    }
    Ok(())
}
