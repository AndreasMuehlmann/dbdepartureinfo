use std::time::Duration;

use druid::{Event, Env, EventCtx, Widget, TimerToken};
use druid::widget::Controller;

use crate::data::*;

use crate::delegate::API_CALL;


const API_CALL_INTERVAL: Duration = Duration::from_secs(60);

pub struct AppStateController {
    timer_id: TimerToken,
}

impl AppStateController {
    pub fn new() -> Self {
        return Self {
            timer_id: TimerToken::INVALID,
        } 
    }
}

impl<W: Widget<AppState>> Controller<AppState, W> for AppStateController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env
        ) {
        match event {
            Event::WindowConnected => {
                self.timer_id = ctx.request_timer(API_CALL_INTERVAL);
                ctx.submit_command(API_CALL);
            },
            Event::Timer(id) => {
                if *id == self.timer_id {
                    self.timer_id = ctx.request_timer(API_CALL_INTERVAL);
                    ctx.submit_command(API_CALL);
                }
            },
            _ => ()
        }
        child.event(ctx, event, data, env)
    }
}
