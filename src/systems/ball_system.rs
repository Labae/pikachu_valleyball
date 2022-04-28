// move
// hyper
// set status.. or store the previous pos (Vector3)
use crate::game_play_state::{Ball, Velocity};
use crate::game_play_state::{GRAVITY, GROUND_Y};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};

#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut balls, mut velocities, mut transforms, time): Self::SystemData) {
        for (ball, velocity, transform) in (&mut balls, &mut velocities, &mut transforms).join() {
            if transform.translation().y <= GROUND_Y {
                transform.set_translation_y(GROUND_Y);
            } else {
                velocity.y = GRAVITY * time.delta_seconds();
            }
            transform.prepend_translation_y(velocity.y * ball.move_speed);
            ball.position.x = transform.translation()[0];
            ball.position.y = transform.translation()[1];

            ball.prev1_position.x = ball.position.x;
            ball.prev1_position.y = 40.0 + ball.position.y;

            ball.prev2_position.x = ball.position.x;
            ball.prev2_position.y = 80.0 + ball.position.y;
        }
    }
}
