use gtk::prelude::*;
use relm4::prelude::*;

mod content;
mod settings;

use settings::Settings;

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
        adw::ApplicationWindow {
            set_title: Some("Template"),  // TODO: Set window title
            set_default_width: settings.int(Settings::WindowWidth.as_str()),
            set_default_height: settings.int(Settings::WindowHeight.as_str()),
            set_maximized: settings.boolean(Settings::WindowMaximized.as_str()),
            add_css_class: "devel",

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar,

                model.content.widget(),
            }
        }
    }

    /// Initialize the UI and model.
    fn init(
        _init: Self::Init,
        window: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = gtk::gio::Settings::new(APP_ID);

        let content = content::ContentModel::builder()
            .launch(content::ContentInit)
            .detach();
        let model = AppModel { content };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {}
    }
}
