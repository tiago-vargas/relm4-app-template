use relm4::prelude::*;

mod app;

fn main() {
    let app = RelmApp::new("local.sandbox.tiago-vargas.Template");
    app.run::<app::AppModel>(());
}
