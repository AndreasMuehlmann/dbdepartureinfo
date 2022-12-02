use druid::{Data, Lens};
use druid::im::Vector;

use crate::station::Station;


#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub stations: Vector<Station>,
}


impl AppState {
    pub fn new(stations: Vec<Station>) -> Self {
        return Self {
            stations: Vector::from(stations),
        }
    }
}
