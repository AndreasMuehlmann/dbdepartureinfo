use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target};

use crate::data::AppState;

pub const API_CALL: Selector = Selector::new("API_CALL");

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        _data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if cmd.is(API_CALL) {
            println!("it woorked!");
            Handled::Yes
        } else {
            println!("cmd forwarded: {:?}", cmd);
            Handled::No
        }
    }
}
