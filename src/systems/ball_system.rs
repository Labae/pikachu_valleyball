// move
// hyper
// set status.. or store the previous pos (Vector3)
use crate::game_play_state::{Ball, Velocity};
use crate::game_play_state::{GRAVITY, GROUND_Y};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};

const GHOST1_UPDATE_TIME: f32 = 0.04;
const GHOST2_UPDATE_TIME: f32 = 0.08;

#[derive(SystemDesc)]
pub struct BallSystem {
    ghost1_update_timer: f32,
    ghost2_update_timer: f32,
}

impl BallSystem {
    pub fn default() -> BallSystem {
        BallSystem {
            ghost1_update_timer: GHOST1_UPDATE_TIME,
            ghost2_update_timer: GHOST2_UPDATE_TIME,
        }
    }
}

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    // have to impl the rigid_body(physic).
    fn run(&mut self, (mut balls, mut velocities, mut transforms, time): Self::SystemData) {
        for (ball, velocity, transform) in (&mut balls, &mut velocities, &mut transforms).join() {
            // gravity
            if transform.translation().y > GROUND_Y {
                velocity.y = GRAVITY;
            }

            // clamp the wall and ground
            if transform.translation().y <= GROUND_Y && velocity.y < 0.0 {
                velocity.y *= -1.0;
            }
            if transform.translation().y >= 600.0 - (20.0 * transform.scale().y) && velocity.y > 0.0
            {
                velocity.y *= -1.0;
            }

            if transform.translation().x <= (20.0 * transform.scale().x) && velocity.x < 0.0 {
                velocity.x *= -1.0;
            }
            if transform.translation().x >= 800.0 - (20.0 * transform.scale().x) && velocity.x > 0.0
            {
                velocity.x *= -1.0;
            }

            let acc = if ball.is_hyper {
                ball.hyper_move_speed
            } else {
                ball.move_speed
            };
            transform.append_translation_xyz(
                velocity.x * time.delta_seconds() * acc,
                velocity.y * time.delta_seconds() * acc,
                0.0,
            );

            if ball.is_hyper {
                ball.position.x = transform.translation().x;
                ball.position.y = transform.translation().y;
                self.ghost1_update_timer -= time.delta_seconds();
                if self.ghost1_update_timer <= 0.0 {
                    ball.prev1_position.x = ball.position.x;
                    ball.prev1_position.y = ball.position.y;
                    self.ghost1_update_timer = GHOST1_UPDATE_TIME;
                }
                self.ghost2_update_timer -= time.delta_seconds();
                if self.ghost2_update_timer <= 0.0 {
                    ball.prev2_position.x = ball.position.x;
                    ball.prev2_position.y = ball.position.y;
                    self.ghost2_update_timer = GHOST2_UPDATE_TIME;
                }
            }
        }
    }
}
