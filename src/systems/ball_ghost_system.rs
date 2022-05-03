use crate::game_play_state::{Ball, BallGhost};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};

#[derive(SystemDesc)]
pub struct BallGhostSystem;

impl<'s> System<'s> for BallGhostSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, BallGhost>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, ball_ghosts, mut transforms, time): Self::SystemData) {
        (&balls).join().for_each(|ball| {
            if ball.is_hyper {
                for (ball_ghost, transform) in (&ball_ghosts, &mut transforms).join() {
                    if !ball.is_hyper {
                        if !ball_ghost.is_trail {
                            transform.set_translation_x(ball.prev1_position.x);
                            transform.set_translation_y(ball.prev1_position.y);
                        } else {
                            transform.set_translation_x(ball.prev2_position.x);
                            transform.set_translation_y(ball.prev2_position.y);
                        }

                        transform.prepend_rotation_z_axis(
                            time.delta_seconds() * std::f32::consts::PI * 5.0,
                        );
                    }
                }
            }
        });
    }
}
