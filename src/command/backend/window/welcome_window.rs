use crate::modules::app::instance::InstanceMessage;
use async_channel::Sender;
use gui::prelude::*;

pub fn handle_gui(app: &gui::Application, tx_message: &Sender<InstanceMessage>) {
	let window_title = "Welcome";
	let label = gui::Label::builder()
		.label("Welcome to Lx DOS!")
		.margin_top(24)
		.margin_bottom(24)
		.margin_start(24)
		.margin_end(24)
		.build();

	let window = crate::modules::app::gui::Gui::window_builder(app, window_title)
		.child(&label)
		.width_request(400)
		.height_request(200)
		.build();

	let tx_message_clone = tx_message.clone();
	let pipe_name_for_close = "welcome_window".to_string();
	window.connect_close_request(move |window| {
		println!("Window close requested, sending CloseWindow: {}", pipe_name_for_close);
		let _ = tx_message_clone.send_blocking(InstanceMessage::CloseWindow {
			pipe_name: pipe_name_for_close.clone(),
		});
		window.close();
		gui::glib::Propagation::Proceed
	});

	window.present();
}
