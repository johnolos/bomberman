use amethyst::{
    core::Time,
    ecs::prelude::{Entity, System, Write, WriteStorage},
    ui::{UiFinder, UiText},
};

#[derive(Default)]
pub struct CountDownSystem {
    count_down_display: Option<Entity>,
}

impl<'a> System<'a> for CountDownSystem {
    type SystemData = (Write<'a, Time>, WriteStorage<'a, UiText>, UiFinder<'a>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut time, mut ui_text, finder) = data;

        if let None = self.count_down_display {
            if let Some(count_down_entity) = finder.find("count_down") {
                self.count_down_display = Some(count_down_entity);
            }
            time.set_fixed_seconds(0.0);
        }

        // FIXME: Optimalization - Change label once every seconds instead of all the time
        if let Some(count_down_entity) = self.count_down_display {
            if let Some(count_down_label) = ui_text.get_mut(count_down_entity) {
                let current_time = 90 - time.absolute_time_seconds().round() as i32;
                let minutes = current_time / 60;
                let seconds = current_time - (minutes * 60);
                count_down_label.text = format!("{:01}:{:02}", minutes, seconds);
            }
        }
    }
}
