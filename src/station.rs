use druid::{Data, Lens};
use druid::im::Vector;

use crate::departure::Departure;


#[derive(Clone, Data, Lens)]
pub struct Station {
    pub name: String,
    pub departures: Vector<Departure>,
}


impl Station {
    pub fn new(name: String, departures: Vec<Departure>) -> Self {
        return Self {
            name,
            departures: Vector::from(departures),
        }
    }

    pub fn set_departures(&mut self, departures: Vec<Departure>) {
        self.departures = Vector::from(departures);
    }
}
