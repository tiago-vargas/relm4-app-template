use relm4::prelude::*;

mod app;

fn main() {
    let app = RelmApp::new(todo!("Set app ID"));
    app.run::<app::AppModel>(());
}
