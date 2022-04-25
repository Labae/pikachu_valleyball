use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use amethyst::renderer::SpriteRender;

use crate::game_play_state::Pikachu;

#[derive(SystemDesc)]
pub struct PikachuSystem;

impl<'s> System<'s> for PikachuSystem {
    type SystemData = (
        WriteStorage<'s, Pikachu>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut pikachus, mut sprite_renders, time): Self::SystemData) {
        for (pikachu, sprite_render) in (&mut pikachus, &mut sprite_renders).join() {
            pikachu.current_frame += time.delta_seconds() * pikachu.animation_speed;
            let cur_frame = pikachu.current_frame as usize;
            if cur_frame >= pikachu.frame_size {
                pikachu.current_frame -= pikachu.frame_size as f32;
            }
            sprite_render.sprite_number = pikachu.current_frame as usize;
        }
    }
}
