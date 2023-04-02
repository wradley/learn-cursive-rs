use crate::material_menu::get_material_menu;
use crate::model_menu::get_model_menu;
use crate::scene_menu::get_scene_menu;
use crate::spec_menu::get_spec_menu;
use crate::texture_menu::get_texture_menu;
use cursive::views::{Button, Dialog, DummyView, LinearLayout, SelectView};
use cursive::Cursive;
use strum_macros::Display;

#[derive(Clone, Copy, Display)]
enum MainMenuOption {
    Scenes,
    Specs,
    Models,
    Materials,
    Textures,
}

pub fn get_main_menu() -> Dialog {
    let select = SelectView::<MainMenuOption>::new()
        .on_submit(select_option)
        .with_all(
            vec![
                MainMenuOption::Scenes,
                MainMenuOption::Specs,
                MainMenuOption::Models,
                MainMenuOption::Materials,
                MainMenuOption::Textures,
            ]
            .iter()
            .map(|option| (option.to_string(), *option)),
        );

    Dialog::around(select)
        .title("Freight Editor")
        .button("Quit", |s| s.quit())
}

fn select_option(s: &mut Cursive, option: &MainMenuOption) {
    s.pop_layer();
    match option {
        MainMenuOption::Scenes => s.add_layer(get_scene_menu()),
        MainMenuOption::Specs => s.add_layer(get_spec_menu()),
        MainMenuOption::Models => s.add_layer(get_model_menu()),
        MainMenuOption::Materials => s.add_layer(get_material_menu()),
        MainMenuOption::Textures => s.add_layer(get_texture_menu()),
    }
}
