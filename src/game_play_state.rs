use amethyst::{
    assets::ProgressCounter,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, World},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, SpriteRender},
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

        init_camera(world);
        init_pikachu(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
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
pub const PIKACHU_ANIMATION_SPEED: f32 = 20.0;

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
enum PikachuAnimations {
    Idle,
}

pub struct Pikachu {
    pub side: Side,
    pub current_frame: f32,
    pub animation_speed: f32,
    pub frame_size: usize,
    pub width: f32,
    pub height: f32,
}

impl Pikachu {
    fn new(side: Side, frame_size: usize) -> Pikachu {
        Pikachu {
            side,
            animation_speed: PIKACHU_ANIMATION_SPEED,
            current_frame: 0.0,
            frame_size: frame_size,
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

    // https://mtigley.dev/posts/sprite-animations-with-amethyst/
    let pikachu_sheet = spritesheet::load_sprite_sheet(world, "texture/pikachu_idle.ron");
    let sprite_render = SpriteRender::new(pikachu_sheet, 0);
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Pikachu::new(Side::Left, 8))
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(sprite_render)
        .with(Pikachu::new(Side::Right, 8))
        .with(right_transform)
        .build();
}
