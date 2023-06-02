use app::model::AppModel;
use constants::*;
use relm4::RelmApp;
mod app;
mod constants;
mod logparse;
fn main() {
    let app = RelmApp::new(APP_ID);
    app.run_async::<AppModel>(None);
}
