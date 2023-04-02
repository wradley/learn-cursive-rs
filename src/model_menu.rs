use crate::main_menu::get_main_menu;
use cursive::views::{Dialog, TextView};

pub fn get_model_menu() -> Dialog {
    Dialog::around(TextView::new("** models **"))
        .title("Model Editor")
        .button("Back", |s| {
            s.pop_layer();
            s.add_layer(get_main_menu());
        })
}
