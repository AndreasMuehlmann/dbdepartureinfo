use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json::Value;

use druid::{AppLauncher, WindowDesc, WindowState};

mod state;
use state::State;

mod station;
use station::Station;

mod departure;

mod message;

mod delegate;
use delegate::Delegate;

mod dbf_api;

mod state_controller;

mod view;
use view::build_ui;

mod utils;
use utils::text_value_to_string;

#[cfg(test)]
mod tests;


// TODO: handle error when api call returns error
// TODO: write README.md
// TODO: look what LICENSE is required by dbf


fn read_stations_from_file() -> Vec<Station> {
    let path = Path::new("./stations.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let parsed_json: Value = serde_json::from_reader(reader).unwrap();
    let value_stations = parsed_json["stations"].as_array().unwrap();

    let mut stations = Vec::new();
    for station in value_stations {
        let station = Station::new(
            text_value_to_string(&station["displayed_name"]),
            text_value_to_string(&station["api_name"]),
            Vec::new()
            );
        stations.push(station)
    } 
    return stations;
}


fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("DBDepartureInfo")
        .set_window_state(WindowState::MAXIMIZED);

    let initial_state = State::new(read_stations_from_file());

    AppLauncher::with_window(main_window)
        .delegate(Delegate::new())
        .launch(initial_state)
        .expect("Failed to launch application");
}
