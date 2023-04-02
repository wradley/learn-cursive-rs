use crate::main_menu::get_main_menu;

mod main_menu;
mod material_menu;
mod model_menu;
mod scene_menu;
mod spec_menu;
mod texture_menu;

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(get_main_menu());
    siv.run();
}
