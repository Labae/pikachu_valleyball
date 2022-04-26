use amethyst::{
    assets::ProgressCounter,
    core::math::Vector3,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, World},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, SpriteRender},
};

use crate::animation::{self, Animation};
use crate::spritesheet;

pub const GROUND_Y: f32 = 100.0;
pub const GRAVITY: f32 = -35.0;

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

#[derive(Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

pub const PIKACHU_WIDTH: f32 = 8.0;
pub const PIKACHU_HEIGHT: f32 = 16.0;
pub const PIKACHU_MOVE_SPEED: f32 = 300.0;
pub const PIKACHU_JUMP_FORCE: f32 = 20.0;

#[derive(Copy, Clone)]
pub enum PikachuAction {
    Idle,
    Jump,
}

#[derive(Clone, Copy)]
pub struct PikachuStatus {
    pub action: PikachuAction,
}

impl PikachuStatus {
    pub fn set_action_type(&mut self, action: PikachuAction) {
        self.action = action;
    }
}

impl Component for PikachuStatus {
    type Storage = DenseVecStorage<Self>;
}

pub struct Pikachu {
    pub side: Side,
    pub move_speed: f32,
    pub is_grounded: bool,
    pub jump_buffer: f32,
    idle_anim: Animation,
    jump_anim: Animation,
}

impl Pikachu {
    fn new(side: Side, idle_anim: Animation, jump_anim: Animation) -> Pikachu {
        Pikachu {
            side,
            move_speed: PIKACHU_MOVE_SPEED,
            is_grounded: false,
            jump_buffer: 0.0,
            idle_anim: idle_anim,
            jump_anim: jump_anim,
        }
    }

    pub fn get_animation(&self, status: &PikachuStatus) -> Result<Animation, amethyst::Error> {
        let anim = match status.action {
            PikachuAction::Idle => self.idle_anim,
            PikachuAction::Jump => self.jump_anim,
        };

        Ok(anim)
    }
}

impl Component for Pikachu {
    type Storage = DenseVecStorage<Self>;
}

fn init_pikachu(world: &mut World) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();
    left_transform.set_translation_xyz(212.0, GROUND_Y, 0.0);
    left_transform.set_scale(Vector3::new(2.0, 2.0, 1.0));

    right_transform.set_translation_xyz(812.0, GROUND_Y, 0.0);
    right_transform.set_rotation_y_axis(std::f32::consts::PI);
    right_transform.set_scale(Vector3::new(2.0, 2.0, 1.0));

    let pikachu_sheet = spritesheet::load_sprite_sheet(
        world,
        "texture/sprite_sheet.png",
        "texture/pikachu_anim.ron",
    );
    let sprite_render = SpriteRender::new(pikachu_sheet, 0);

    let velocity = Velocity { x: 0.0, y: 0.0 };
    let status = PikachuStatus {
        action: PikachuAction::Idle,
    };

    let idle_anim = animation::Animation {
        frames: 8,
        frame_duration: 10,
        first_sprite_index: 0,
    };
    let jump_anim = animation::Animation {
        frames: 8,
        frame_duration: 2,
        first_sprite_index: 8,
    };
    let left_pikachu = Pikachu::new(Side::Left, idle_anim.clone(), jump_anim.clone());
    let right_pikachu = Pikachu::new(Side::Right, idle_anim, jump_anim);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(status.clone())
        .with(velocity.clone())
        .with(left_pikachu)
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(sprite_render)
        .with(status)
        .with(velocity)
        .with(right_pikachu)
        .with(right_transform)
        .build();
}
