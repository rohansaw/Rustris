use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::{
        Entities, Join, LazyUpdate, Read, ReadExpect, System, SystemData, WriteExpect, WriteStorage,
    },
    renderer::SpriteRender,
};

use crate::{
    resources::SpriteResource,
    tetris::{Piece, BLOCK_SIZE, ARENA_HEIGHT, ARENA_WIDTH},
};
#[derive(SystemDesc)]
pub struct MovePiecesSystem {
    pub counter: u32,
}

impl<'s> System<'s> for MovePiecesSystem {
    type SystemData = (
        WriteStorage<'s, Piece>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (mut pieces, mut transforms, entities, time, sprite_resource, updater): Self::SystemData,
    ) {
        for (entity, piece, transform) in (&*entities, &mut pieces, &mut transforms).join() {
            if !piece.falling {
                entities.delete(entity).unwrap();
                continue;
            }
            piece.time_since_move += time.delta_seconds();

            if piece.time_since_move >= 1.0 {
                transform.prepend_translation_y(-BLOCK_SIZE);
                piece.time_since_move -= 1.0;
                println!("moved");
            }

            if transform.translation().y < 20.0 {
                piece.falling = false;
                println!("stopped");
                // if it cannot move further leave falling_piece.piece = None
            }
        }

        self.counter += 1;
        if self.counter > 200 {
            let piece = entities.create();
            let mut transform = Transform::default();
            transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT, 0.0);

            updater.insert(piece, transform);
            updater.insert(piece, Piece::new());
            updater.insert(piece, SpriteRender::new(sprite_resource.sprite_sheet.clone(), 0).clone());

            self.counter = 0;
            println!("spawned");
        }
    }
}
