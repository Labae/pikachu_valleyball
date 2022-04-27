use crate::game_play_state::Ball;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct BallAnimationSystem;

impl<'s> System<'s> for BallAnimationSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut balls, mut sprite_renders, time): Self::SystemData) {
        for (ball, sprite) in (&mut balls, &mut sprite_renders).join() {
            let elapsed_time = time.frame_number();
            let frame =
                (elapsed_time / ball.idle_anim.frame_duration) as i32 % ball.idle_anim.frames;
            sprite.sprite_number = ball.idle_anim.first_sprite_index + frame as usize;
        }
    }
}
