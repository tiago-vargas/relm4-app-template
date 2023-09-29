use gtk::prelude::*;
use relm4::prelude::*;

mod content;

pub(crate) struct AppModel {
    content: Controller<content::ContentModel>,
}

#[derive(Debug)]
pub(crate) enum AppInput {}

#[relm4::component(pub(crate))]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppInput;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some(todo!("Set window title")),
            set_default_width: 600,
            set_default_height: 300,

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