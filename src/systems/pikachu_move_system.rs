use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::game_play_state::{
    Pikachu, PikachuAction, PikachuStatus, Side, Velocity, GRAVITY, GROUND_Y, PIKACHU_JUMP_FORCE,
};

const JUMP_BUFFER_TIME: f32 = 0.15;

#[derive(SystemDesc)]
pub struct PikachuMoveSystem;

impl<'s> System<'s> for PikachuMoveSystem {
    type SystemData = (
        WriteStorage<'s, Pikachu>,
        WriteStorage<'s, PikachuStatus>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut pikachus, mut statuses, mut transforms, mut velocities, input, time): Self::SystemData,
    ) {
        for (pikachu, status, transform, velocity) in (
            &mut pikachus,
            &mut statuses,
            &mut transforms,
            &mut velocities,
        )
            .join()
        {
            velocity.x = 0.0;

            let movement = match pikachu.side {
                Side::Left => input.axis_value("left_pikachu_horizontal"),
                Side::Right => input.axis_value("right_pikachu_horizontal"),
            };

            if let Some(mv_amount) = movement {
                let scaled_amount = pikachu.move_speed * time.delta_seconds() * mv_amount as f32;
                velocity.x = scaled_amount;
            }

            if transform.translation().y <= GROUND_Y {
                pikachu.is_grounded = true;
                status.set_action_type(PikachuAction::Idle);
            }

            if pikachu.is_grounded {
                pikachu.jump_buffer += time.delta_seconds();
                let jump = match pikachu.side {
                    Side::Left => input.action_is_down("left_pikachu_jump"),
                    Side::Right => input.action_is_down("right_pikachu_jump"),
                };
                if let Some(true) = jump {
                    if pikachu.jump_buffer >= JUMP_BUFFER_TIME {
                        pikachu.is_grounded = false;
                        velocity.y = PIKACHU_JUMP_FORCE;
                        status.set_action_type(PikachuAction::Jump);
                        pikachu.jump_buffer = 0.0;
                    }
                }
            } else {
                velocity.y += GRAVITY * time.delta_seconds();
            }

            transform.prepend_translation_x(velocity.x);
            transform.prepend_translation_y(velocity.y);

            if pikachu.is_grounded {
                transform.set_translation_y(GROUND_Y);
                velocity.y = 0.0;
            }
        }
    }
}
