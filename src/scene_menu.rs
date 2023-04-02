use crate::main_menu::get_main_menu;
use cursive::views::{Button, Dialog, DummyView, Layer, LinearLayout, SelectView};
use cursive::Cursive;

const EXISTING_SCENES: [&str; 3] = ["init", "Load Screen", "Open World"];

pub fn get_scene_menu() -> Dialog {
    let scene_selection = SelectView::<String>::new().with_all_str(EXISTING_SCENES);

    let buttons = LinearLayout::vertical()
        .child(Button::new("Create new", create_scene))
        .child(Button::new("Delete", delete_scene))
        .child(DummyView)
        .child(Button::new("Back", back));

    Dialog::around(
        LinearLayout::horizontal()
            .child(scene_selection)
            .child(DummyView)
            .child(buttons),
    )
    .title("Scene Editor")
}

fn create_scene(s: &mut Cursive) {
    todo!()
}

fn delete_scene(s: &mut Cursive) {
    todo!()
}

fn back(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(get_main_menu());
}
