use crate::game_play_state::{Ball, BallGhost};
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};

#[derive(SystemDesc)]
pub struct BallGhostSystem;

impl<'s> System<'s> for BallGhostSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, BallGhost>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (balls, ball_ghosts, mut transforms): Self::SystemData) {
        (&balls).join().for_each(|ball| {
            for (ball_ghost, transform) in (&ball_ghosts, &mut transforms).join() {
                if !ball.is_hyper {
                    if !ball_ghost.is_trail {
                        transform.set_translation_x(ball.prev1_position.x);
                        transform.set_translation_y(ball.prev1_position.y);
                    } else {
                        transform.set_translation_x(ball.prev2_position.x);
                        transform.set_translation_y(ball.prev2_position.y);
                    }
                }
            }
        });
    }
}
