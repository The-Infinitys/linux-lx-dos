use crate::LxDosError;
use crate::modules::app::gui::Gui;
// use crate::modules::app::instance::InstanceMessage;
use instance_pipe::Client;

pub fn run_backend(pipe_name: &str) -> Result<(), LxDosError> {
    let _client = Client::connect(pipe_name)?;
    // let message: InstanceMessage = client.recv()?;
    let gui = Gui::new();
    let pipe_name = pipe_name.to_string();
    let pipe_name_clone = pipe_name.clone();
    gui.handler(move |app| {
        use gui::Button;
        use gui::prelude::*;
        let button = Button::builder()
            .label("Press me!")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        // Connect to "clicked" signal of `button`
        let pipe_name_clone = pipe_name_clone.clone();
        button.connect_clicked(move |_| {
            println!("Hello, World!: {}", pipe_name_clone);
        });
        let window = Gui::window_builder(app, "Lx-DOS Window")
            .child(&button)
            .width_request(480)
            .height_request(360)
            .build();
        window.present();
    });
    gui.run();

    Ok(())
}
