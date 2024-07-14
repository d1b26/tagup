use iced::widget::{button, column, text, text_input, Column};


#[derive(Default)]
struct Counter {
    value: i64,
    input: String,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    InputValue(String),
    Exectue,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::Exectue => {
                println!("exec ed");
                println!("{:?}", self.input);
            }
            Message::InputValue(val) => {
                println!("{:?}", val);
                self.input = val;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        // The buttons
        let increment = button("+").on_press(Message::Increment);
        let decrement = button("-").on_press(Message::Decrement);

        // The number
        let counter = text(self.value);

        let inputbox = text_input("ya", 
        &self.input).on_input(|value| Message::InputValue(value)).on_submit(Message::Exectue);

        // The layout
        let interface = column![increment, counter, decrement, inputbox];

        interface
    }
}

pub fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}