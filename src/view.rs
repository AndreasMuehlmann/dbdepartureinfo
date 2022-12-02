use druid::widget::{Flex, Label, List};
use druid::{Widget, WidgetExt};

use crate::app_state_controller::AppStateController;
use crate::data::AppState;
use crate::station::Station;
use crate::departure::Departure;


pub fn build_ui() -> impl Widget<AppState> {
    return List::new(build_station).lens(AppState::stations)
        .controller(AppStateController::new());
}

fn build_station() -> impl Widget<Station> {
    return Flex::column()
        .with_child(Label::raw().lens(Station::name))
        .with_spacer(10.0)
        .with_child(List::new(build_departure).lens(Station::departures));
}

fn build_departure() -> impl Widget<Departure> {
    return Flex::row()
        .with_child(Label::raw().lens(Departure::train))
        .with_child(Label::raw().lens(Departure::destination))
        .with_child(Label::raw().lens(Departure::scheduled_platform))
        .with_spacer(50.0)
        .with_child(Label::raw().lens(Departure::scheduled_departure))
        .with_child(Label::raw().lens(Departure::delay_departure));
}
