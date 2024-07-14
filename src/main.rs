use iced::widget::{button, column, text, Column};

pub fn main() -> iced::Result {
    iced::run("A counter", update, view)
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn update(value: &mut u64, message: Message) {
    match message {
        Message::Increment => *value += 1,
    }
}

fn view(value: &u64) -> Column<Message> {
    column![
        text(value),
        button("+").on_press(Message::Increment),
    ]
}