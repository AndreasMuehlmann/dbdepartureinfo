use druid::{AppLauncher, WindowDesc, WindowState};

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


pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("DBDepartureInfo")
        .set_window_state(WindowState::MAXIMIZED);

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
