use druid::{
    widget::{Align, Button, Flex, Label, TextBox},
    AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc,
};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Locksmith");

#[derive(Clone, Data, Lens)]
struct AppState {
    name: String,
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0))
        .resizable(false);
    let initial_state = AppState {
        name: "Arshil".to_string(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    let label = Label::new(|data: &AppState, _env: &Env| format!("Hello, {}", data.name));
    let textbox = TextBox::new()
        .with_placeholder("One piece is real")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AppState::name);

    let button = Button::new("Save")
        .fix_width(150.0)
        .on_click(|_ctx, name: &mut String, _env: &Env| {
            println!("Button clicked with string {}", name);
        })
        .lens(AppState::name);

    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .with_spacer(10.2)
        .with_child(button);

    Align::centered(layout)
}
