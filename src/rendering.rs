//! Component and systems permitting to render an entity to the screen.

#![allow(clippy::wildcard_enum_match_arm)]

use crate::physics::Position;
use ggez::{graphics, Context, GameResult};
use graphics::{DrawParam, Image};
use legion::prelude::*;

/// Component for rendering an entity
#[derive(Clone, Debug, PartialEq)]
pub struct Rendering {
    /// Sprite to render
    pub sprite: Image,
    /// Drawing info e.g. position or scale
    pub param: DrawParam,
}

/// Update the position of the sprite depending on the entity position
pub fn update(world: &mut World) {
    let query = <(Read<Position>, Write<Rendering>)>::query().filter(changed::<Position>());
    //                                                               ^ only changed positions
    for (pos, mut render) in query.iter_mut(world) {
        render.param = render.param.dest(*pos);
    }
}

/// Draw all entities with a rendering component
pub fn draw(ctx: &mut Context, world: &World) -> GameResult {
    let query = Read::<Rendering>::query();

    for render in query.iter(world) {
        graphics::draw(ctx, &render.sprite, render.param)?;
    }
    Ok(())
}
