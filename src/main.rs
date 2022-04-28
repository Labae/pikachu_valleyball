use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

pub mod animation;
pub mod game_play_state;
pub mod spritesheet;
pub mod systems;

use crate::game_play_state::GamePlayState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let bindings_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        // .with(
        //     SpriteVisibilitySortingSystem::new(),
        //     "sprite_visibility_system",
        //     &["transform_system"],
        // )
        .with_bundle(input_bundle)?
        .with(systems::PikachuAnimationSystem, "pikachu_anim_system", &[])
        .with(
            systems::PikachuMoveSystem,
            "pikachu_system",
            &["input_system"],
        )
        .with(systems::BallSystem, "ball_system", &[])
        .with(systems::BallAnimationSystem, "ball_anim_system", &[])
        .with(
            systems::BallGhostSystem,
            "ball_ghost_system",
            &["ball_system"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )
        .unwrap();

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, GamePlayState::default(), game_data).unwrap();
    game.run();

    Ok(())
}
