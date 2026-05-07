mod app;
mod app_views;
mod calculator;
mod converter;
mod programmer;
mod settings;
mod history;

use app::CosmicCalculator;

fn main() -> cosmic::iced::Result {
    cosmic::app::run::<CosmicCalculator>(
        cosmic::app::Settings::default()
            .antialiasing(true)
            .client_decorations(true)
            .debug(false),
        (),
    )
}
