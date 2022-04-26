use crate::game_play_state::{Pikachu, PikachuStatus};
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct PikachuAnimationSystem;

impl<'s> System<'s> for PikachuAnimationSystem {
    type SystemData = (
        ReadStorage<'s, PikachuStatus>,
        WriteStorage<'s, Pikachu>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (pikachu_statuses, mut pikachus, mut sprite_renders, time): Self::SystemData,
    ) {
        for (pikachu_status, pikachu, sprite) in
            (&pikachu_statuses, &mut pikachus, &mut sprite_renders).join()
        {
            let animation = pikachu.get_animation(pikachu_status).unwrap();
            let elapsed_time = time.frame_number();
            let frame = (elapsed_time / animation.frame_duration) as i32 % animation.frames;
            sprite.sprite_number = animation.first_sprite_index + frame as usize;
        }
    }
}
