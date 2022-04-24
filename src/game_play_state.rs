use amethyst::{
    assets::ProgressCounter,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, Join, World},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
};
use serde::{Deserialize, Serialize};

use crate::spritesheet;

#[derive(Default)]
pub struct GamePlayState {
    pub progress_counter: Option<ProgressCounter>,
}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        self.progress_counter = Some(Default::default());

        let pikachu_sheet = spritesheet::load_sprite_sheet(world, "texture/pikachu_idle.ron");
        init_camera(world);
        init_pikachu(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref progress_counter) = self.progress_counter {
            if progress_counter.is_complete() {
                let entites = _data.world.entities_mut();
                for entity in entites.join() {}
            }
        }
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                return Trans::Quit;
            }
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(512.0, 384.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(1024.0, 768.0))
        .with(transform)
        .build();
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

pub const PIKACHU_WIDTH: f32 = 8.0;
pub const PIKACHU_HEIGHT: f32 = 16.0;

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
enum PikachuAnimations {
    Idle,
}

pub struct Pikachu {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Pikachu {
    fn new(side: Side) -> Pikachu {
        Pikachu {
            side,
            width: PIKACHU_WIDTH,
            height: PIKACHU_HEIGHT,
        }
    }
}

impl Component for Pikachu {
    type Storage = DenseVecStorage<Self>;
}

fn init_pikachu(world: &mut World) {
    use std::f32::consts::PI;

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();
    left_transform.set_translation_xyz(212.0, 384.0, 0.0);
    right_transform.set_translation_xyz(812.0, 384.0, 0.0);
    right_transform.set_rotation_y_axis(PI);

    world
        .create_entity()
        .with(Pikachu::new(Side::Left))
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(Pikachu::new(Side::Right))
        .with(right_transform)
        .build();
}
