use gui::gio::prelude::ApplicationExtManual;
use std::rc::Rc; // Import Rc
use std::cell::RefCell; // Import RefCell

use super::super::modules::app::App;
use crate::LxDosError;

pub fn run() -> Result<(), LxDosError> {
    let app = Rc::new(RefCell::new(App::default())); // Wrap app in Rc and RefCell
    let gui = app.borrow().gui.clone(); // Clone the gui reference

    // Move the Rc<RefCell<App>> into the closure
    gui.connect_open(move |_gui_app, _f, _hint| {
        use gui::prelude::*;

        // Borrow app from the RefCell to use it
        let app_ref = app.borrow();
        let window = app_ref
            .window_builder("hello")
            .width_request(800)
            .height_request(600)
            .build();
        let button = gui::Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        window.set_child(Some(&button));
        window.present();
    });
    gui.run();
    Ok(())
}