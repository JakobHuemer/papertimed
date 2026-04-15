use crate::config::AppSettings;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Self {}
    }

    /// Evaluates the background image file path for a given AppSettings struct.
    pub fn evaluate_background(&self, settings: AppSettings) -> String {

        // walk through
        
        for wp in settings.wallpapers.iter() {
            for schedules in wp.schedules.iter() {
                for rules in schedules.rules.iter() {

                }
            }
        }


        todo!()
    }
}
