mod driver_db;
mod detection;
mod installer;
mod gui;
mod utils;

use gui::App;

fn main() -> iced::Result {
    App::run(iced::Settings::default())
}