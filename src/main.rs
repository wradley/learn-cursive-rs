use cursive::traits::Resizable;
use cursive::views::{Button, Dialog, DummyView, LinearLayout, SelectView, TextView};
use cursive::Cursive;
use std::fmt::{Display, Formatter};
use std::mem::swap;

#[derive(Clone, Copy)]
enum MainMenuOption {
    Scenes,
    Specs,
    Models,
    Materials,
    Textures,
}

impl Into<String> for MainMenuOption {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Display for MainMenuOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MainMenuOption::Scenes => "Scenes".to_string(),
            MainMenuOption::Specs => "Specs".to_string(),
            MainMenuOption::Models => "Models".to_string(),
            MainMenuOption::Materials => "Materials".to_string(),
            MainMenuOption::Textures => "Textures".to_string(),
        };
        write!(f, "{}", s)
    }
}

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let select = SelectView::<MainMenuOption>::new()
        .on_submit(select_scene)
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

    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons),
        )
        .title("title"),
    );

    siv.run();
}

fn add_name(s: &mut Cursive) {}
fn delete_name(s: &mut Cursive) {}

fn select_scene(s: &mut Cursive, option: &MainMenuOption) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(format!("Name: {}\nAwesome: yes", option))
            .title(format!("{}'s info", option))
            .button("Quit", Cursive::quit),
    );
}
