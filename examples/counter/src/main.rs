use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    // The counter value
    value: i32,

    // The local state of the two buttons
    increment_button: button::State,
    decrement_button: button::State,
}

// The possible user interactions of our counter: the button presses.
// These interactions are our messages.
#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    // React to any produced messages and change our state accordingly in our update logic.
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    // Show the actual counter by putting it all together in our view logic.
    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout.
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                // The increment button. We tell it to produce an
                // `IncrementPressed` message when pressed.
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                // The decrement button. We tell it to produce a
                // `DecrementPressed` message when pressed.
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}
