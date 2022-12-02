use druid::widget::{Flex, Label};//, List};
use druid::{Widget, WidgetExt};

use crate::app_state_controller::AppStateController;
use crate::data::AppState;


pub fn build_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("Hello"))
        .controller(AppStateController::new())
        //.with_child(List::new(todo_item).lens(AppState::todos))
}
