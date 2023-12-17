use gtk::prelude::*;
use relm4::prelude::*;

mod content;
mod settings;

pub(crate) const APP_ID: &str = "your.app_id";  // TODO: Set app ID

pub(crate) struct AppModel {
    content: Controller<content::ContentModel>,
}

#[derive(Debug)]
pub(crate) enum AppInput {}

#[derive(Debug)]
pub(crate) enum AppOutput {}

#[relm4::component(pub(crate))]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppInput;
    type Output = AppOutput;

    view! {
        main_window = adw::ApplicationWindow {
            set_title: Some("Template"),  // TODO: Set window title
            add_css_class: "devel",

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar,

                model.content.widget(),
            }
        }
    }

    fn init(
        _init: Self::Init,
        window: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let content = content::ContentModel::builder()
            .launch(content::ContentInit)
            .detach();
        let model = AppModel { content };

        let widgets = view_output!();

        Self::load_window_state(&widgets);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {}
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        Self::save_window_state(&widgets);
    }
}
