use crate::modules::app::instance::InstanceMessage;
use async_channel::Sender;
use gui::prelude::*;

pub fn handle_gui(app: &gui::Application, tx_message: &Sender<InstanceMessage>) {
	let window_title = "Lx DOS";
	let button = gui::Button::builder()
		.label("Press me!")
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();

	let window = crate::modules::app::gui::Gui::window_builder(app, window_title)
		.child(&button)
		.width_request(480)
		.height_request(360)
		.build();

	let window_weak = window.downgrade();
	button.connect_clicked(move |_| {
		if let Some(window) = window_weak.upgrade() {
			println!("Button clicked, closing window");
			window.close();
		}
	});

	let tx_message_clone = tx_message.clone();
	let pipe_name_for_close = "main_window".to_string();
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
