use gtk::prelude::*;
use relm4::gtk::*;
use relm4::prelude::*;

fn main() {
    let app = RelmApp::new("local.sandbox.tiago-vargas.Template");
    app.run::<AppModel>(());
}

struct AppModel {}

#[derive(Debug)]
enum AppInput {}

struct AppWidgets {}

impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppInput;
    type Output = ();

    /// The root GTK widget that this component will create.
    type Root = gtk::Window;

    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        Window::builder()
            .title("Template")
            .default_width(600)
            .default_height(300)
            .build()
    }

    /// Initialize the UI and model.
    fn init(
        _init: Self::Init,
        window: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {};

        let label = Label::builder()
            .label("Hello, World!")
            .css_classes(["title-1"])
            .build();

        label.set_margin_all(4);

        window.set_child(Some(&label));

        let widgets = AppWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {}
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
