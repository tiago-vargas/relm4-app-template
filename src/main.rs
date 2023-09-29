use gtk::prelude::*;
use relm4::prelude::*;

fn main() {
    let app = RelmApp::new("local.sandbox.tiago-vargas.Template");
    app.run::<AppModel>(());
}

struct AppModel {}

#[derive(Debug)]
enum AppInput {}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppInput;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Template"),
            set_default_width: 600,
            set_default_height: 300,

            gtk::Label {
                set_label: "Hello, World!",
                set_margin_all: 4,
                set_css_classes: &["title-1"],
            }
        }
    }

    /// Initialize the UI and model.
    fn init(
        _init: Self::Init,
        window: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {}
    }
}
