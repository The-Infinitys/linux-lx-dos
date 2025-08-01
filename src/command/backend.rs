use crate::LxDosError;
use crate::modules::app::gui::Gui;
// use crate::modules::app::instance::InstanceMessage;
use gui::prelude::GtkWindowExt;
// use instance_pipe::Client;

pub fn run_backend(_pipe_name: &str) -> Result<(), LxDosError> {
    // let client = Client::connect(pipe_name)?;
    // let message: InstanceMessage = client.recv()?;
    let gui = Gui::new();
    gui.handler(|app| {
        let window = Gui::window_builder(app, "Lx-DOS Window").build();
        window.present();
    });
    gui.run();

    Ok(())
}
