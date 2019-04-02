use amethyst::{
    prelude::*,
    assets::Loader,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    ecs::prelude::Entity
};

use crate::components::MenuItem;

pub const MENU_COUNT: i32 = 2;

pub fn initialise_menu(world: &mut World, is_start: bool) -> (Entity, Entity) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    let mut start_button_text = "START GAME";

    if !is_start {
        start_button_text = "CONTINUE GAME"
    }

    let p1_transform = UiTransform::new(
        start_button_text.to_string(), Anchor::Middle,
        0., -50., 1., 400., 50., 0,
    );
    let p2_transform = UiTransform::new(
        "EXIT".to_string(), Anchor::Middle,
        0., -150., 1., 200., 50., 0,
    );

    let start_btn = world
        .create_entity()
        .with(p1_transform)
        .with(MenuItem { is_active: true, order: 0 })
        .with(UiText::new(
            font.clone(),
            start_button_text.to_string(),
            [1., 1., 1., 1.],
            50.,
        )).build();

    let exit_btn = world
        .create_entity()
        .with(p2_transform)
        .with(MenuItem { is_active: false, order: 1 })
        .with(UiText::new(
            font.clone(),
            "EXIT".to_string(),
            [1., 1., 1., 1.],
            50.,
        )).build();

    return (start_btn, exit_btn);
}