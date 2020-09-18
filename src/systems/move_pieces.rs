use amethyst::{
    core::{timing::Time, transform::Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::tetris::{Piece, BLOCK_SIZE};
#[derive(SystemDesc)]
pub struct MovePiecesSystem;

impl<'s> System<'s> for MovePiecesSystem {
    type SystemData = (
        WriteStorage<'s, Piece>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut pieces, mut locals, time): Self::SystemData) {
        for (piece, local) in (&mut pieces, &mut locals).join() {
            // check if piece can move
            piece.time_since_move += time.delta_seconds();
            if piece.time_since_move >= 1.0 {
                local.prepend_translation_y(-BLOCK_SIZE);
                piece.time_since_move -= 1.0;
            }
        }
    }
}
