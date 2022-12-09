use druid::{Data, Lens};
use druid::im::Vector;

use crate::station::Station;


#[derive(Clone, Data, Lens)]
pub struct State {
    pub stations: Vector<Station>,
}


impl State {
    pub fn new(stations: Vec<Station>) -> Self {
        return Self {
            stations: Vector::from(stations),
        }
    }
}
