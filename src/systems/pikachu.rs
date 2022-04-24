use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{ReadStorage, System, SystemData, WriteStorage};

use crate::game_play_state::Pikachu;

#[derive(SystemDesc)]
pub struct PikachuSystem;

impl<'s> System<'s> for PikachuSystem {
    type SystemData = (WriteStorage<'s, Transform>, WriteStorage<'s, Pikachu>);

    fn run(&mut self, (mut transforms, mut pikachu): Self::SystemData) {}
}
