use druid::{AppLauncher, WindowDesc};

mod data;
use data::AppState;

mod departure;

mod station;
use station::Station;

mod delegate;
use delegate::Delegate;

mod dbf_api;

mod app_state_controller;

mod view;
use view::build_ui;


#[tokio::main]
pub async fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("DBDepartureInfo")
        .window_size((400.0, 400.0));

    let initial_state = AppState::new(
        vec![
        Station::new("MVS".to_string(), Vec::new()),
        Station::new("MHR".to_string(), Vec::new()),
        Station::new("MGDF".to_string(), Vec::new()),
        Station::new("MTR".to_string(), Vec::new()),
        ]);

    AppLauncher::with_window(main_window)
        .delegate(Delegate::new())
        .launch(initial_state)
        .expect("Failed to launch application");
}
