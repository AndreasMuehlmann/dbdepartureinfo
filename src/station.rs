use druid::{Data, Lens};
use druid::im::Vector;

use crate::departure::Departure;


#[derive(Clone, Data, Lens)]
pub struct Station {
    pub displayed_name: String,
    pub api_name: String,
    pub departures: Vector<Departure>,
}


impl Station {
    pub fn new(displayed_name: String, api_name: String, departures: Vec<Departure>) -> Self {
        return Self {
            displayed_name,
            api_name,
            departures: Vector::from(departures),
        }
    }

    pub fn set_departures(&mut self, departures: Vec<Departure>) {
        self.departures = Vector::from(departures);
    }
}
