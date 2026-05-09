mod event_loop;
mod renderer;

use crate::display::Display;

pub struct WindowRenderer {
    display: Display,
    title: String,
}

impl WindowRenderer {
    pub fn new(display: Display, title: &str) -> Self {
        Self {
            display,
            title: title.to_string(),
        }
    }
    
    pub fn run<F>(self, update: F)
    where
        F: FnMut(&mut crate::display::Display) -> bool + 'static,
    {
        event_loop::run(self.display, &self.title, update);
    }
}
