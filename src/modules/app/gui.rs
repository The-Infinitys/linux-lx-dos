use super::App;
use crate::LxDosError;
use iced::{
    Application, Command, Element, Settings, Theme,
    widget::{button, column, text},
};
#[derive(Debug, Clone)]
pub enum Message {
    Clicked(String),
}

impl Application for App {
    // Defines the type of executor to use for the application.
    // Default is usually fine for most applications.
    type Executor = iced::executor::Default;
    // The type of messages your application can handle.
    type Message = Message;
    // The theme used for styling your widgets.
    type Theme = Theme;
    // Any data you want to pass to the application upon creation.
    type Flags = ();

    // `new` is called when the application starts.
    // It returns the initial state of your application and any initial commands.
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            App::default(),  // Initial state: counter starts at 0
            Command::none(), // No initial commands
        )
    }

    // `title` provides the title for the application window.
    fn title(&self) -> String {
        String::from("Iced Counter App")
    }

    // `update` handles incoming messages and updates the application state.
    // It returns a `Command` which can trigger side effects or more messages.
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Clicked(id) => {
                match id.as_str() {
                    "increment_button" => self.value += 1, // Increment the counter
                    _ => {}
                }
            }
        }
        Command::none() // No further commands after updating the state
    }

    // `view` describes how your application's UI should look.
    // It returns an `Element` which is the root of your widget tree.
    fn view(&self) -> Element<Message> {
        column![
            text(self.value) // Display the current counter value
                .size(50), // Make the text larger
            button("Increment") // A button with the label "Increment"
                .on_press(Message::Clicked("increment_button".into())), // Send a Clicked message when pressed
        ]
        .spacing(20) // Add spacing between the text and the button
        .align_items(iced::Alignment::Center) // Center items horizontally
        .into() // Convert the column into an Element
    }

    // `theme` allows you to customize the visual theme of your application.
    fn theme(&self) -> Self::Theme {
        Theme::Dark // Use the built-in Dark theme
    }
}

impl App {
    // This `start` method is a wrapper to start the Iced application.
    // It replaces your original `run` function.
    pub fn start() -> Result<(), LxDosError> {
        // Run the Iced application with default settings.
        // You can customize settings like window size, etc., using `Settings::new()`.
        App::run(Settings::default())?;
        Ok(())
    }
}
