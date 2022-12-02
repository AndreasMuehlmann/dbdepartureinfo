use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target};
use futures::executor::block_on;

use crate::data::AppState;
use crate::dbf_api::DBFAPI;
use crate::departure::Departure;


pub const API_CALL: Selector = Selector::new("API_CALL");
const DEPARTURE_LIMIT: u32 = 3;


pub struct Delegate {
    dbf_api: DBFAPI,
}


impl Delegate {
    pub fn new() -> Self {
        return Self {
            dbf_api: DBFAPI::new(DEPARTURE_LIMIT),
        }
    }

    fn update_departures(&mut self, data: &mut AppState) {
        for i in 0..data.stations.len() {
            let departures: Vec<Departure> = block_on(self.dbf_api.get_departures(data.stations[i].name.clone()));
            data.stations[i].set_departures(departures);
        }
    }
}


impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx<'_>,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if cmd.is(API_CALL) {
            self.update_departures(data);
            Handled::Yes
        } else {
            println!("cmd forwarded: {:?}", cmd);
            Handled::No
        }
    }
}
