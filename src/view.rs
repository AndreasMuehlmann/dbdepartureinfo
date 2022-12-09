use druid::{Env, Widget, WidgetExt};
use druid::widget::{Flex, Label, List};
use druid::{FontDescriptor, FontWeight, FontStyle, FontFamily};

use druid::piet;
use piet::Color;//, LinearGradient, UnitPoint};

use crate::state_controller::StateController;
use crate::state::State;
use crate::station::Station;
use crate::departure::Departure;
use crate::message::Message;


pub fn build_ui() -> impl Widget<State> {
    /*
    let gradient = LinearGradient::new(
        UnitPoint::TOP,
        UnitPoint::BOTTOM,
        (Color::GRAY, Color::BLACK)
    );
    */
    return List::new(build_station).lens(State::stations)
        .center()
        .expand()
        //.background(gradient)
        .controller(StateController::new());
}

fn build_station() -> impl Widget<Station> {
    return Flex::column()
        .with_child(Label::new(|data: &Station, _: &Env| format!("{}", data.displayed_name)).with_text_color(Color::MAROON).with_font(get_font_descriptor_head_line()))
        .with_child(List::new(build_departure).with_spacing(5.0).lens(Station::departures))
        .with_spacer(15.0);
}

fn get_font_descriptor_head_line() -> druid::FontDescriptor {
    return FontDescriptor::new(FontFamily::SERIF).with_style(FontStyle::Italic).with_size(40.0).with_weight(FontWeight::BOLD);
}

fn build_departure() -> impl Widget<Departure> {
    return Flex::column()
        .with_child(
            Flex::row()
                .with_child(Label::new(|data: &Departure, _: &Env| format!("{}", data.train)).with_text_color(Color::GREEN).with_font(get_font_descriptor_departure_element()).fix_width(50.0))
                .with_spacer(50.0)
                .with_child(Label::new(|data: &Departure, _: &Env| format!("{}", data.destination)).with_text_color(Color::WHITE).with_font(get_font_descriptor_departure_element()).fix_width(250.0))
                .with_spacer(250.0)
                .with_child(Label::new(|data: &Departure, _: &Env| format!("{}", data.scheduled_departure)).with_text_color(Color::WHITE).with_font(get_font_descriptor_departure_element()).fix_width(60.0))
                .with_spacer(60.0)
                .with_child(Label::new(|data: &Departure, _: &Env| format!("+{}", data.delay_departure)).with_text_color(Color::RED).with_font(get_font_descriptor_departure_element()).fix_width(50.0))
                .with_spacer(50.0)
                .with_child(Label::new(|data: &Departure, _: &Env| format!("Gleis {}", data.scheduled_platform)).with_text_color(Color::WHITE).with_font(get_font_descriptor_departure_element()).fix_width(50.0))
        )
        .with_child(List::new(build_message).with_spacing(2.5).lens(Departure::messages));
}

fn get_font_descriptor_departure_element() -> druid::FontDescriptor {
    return FontDescriptor::new(FontFamily::SERIF).with_style(FontStyle::Regular).with_size(20.0).with_weight(FontWeight::NORMAL);
}

fn build_message() -> impl Widget<Message> {
    return Label::new(|data: &Message, _: &Env| format!("{}", data.message)).with_text_color(Color::RED).with_font(get_font_descriptor_message()).fix_width(600.0);
}

fn get_font_descriptor_message() -> druid::FontDescriptor {
    return FontDescriptor::new(FontFamily::SERIF).with_style(FontStyle::Regular).with_size(15.0).with_weight(FontWeight::NORMAL);
}
