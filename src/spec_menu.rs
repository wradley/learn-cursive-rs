use crate::main_menu::get_main_menu;
use cursive::views::{Dialog, TextView};

pub fn get_spec_menu() -> Dialog {
    Dialog::around(TextView::new("** specs **"))
        .title("Spec Editor")
        .button("Back", |s| {
            s.pop_layer();
            s.add_layer(get_main_menu());
        })
}
