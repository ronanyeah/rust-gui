use iced::{widget, Application, Command};

struct Counter {
    value: i32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

impl Application for Counter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;
    fn new(_flags: ()) -> (Counter, Command<Message>) {
        let init_state = Counter { value: 0 };
        (init_state, Command::none())
    }
    fn title(&self) -> String {
        String::from("Rust GUI")
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Increment => {
                self.value += 1;
                Command::none()
            }
        }
    }
    fn view(self: &Counter) -> iced::Element<Message> {
        widget::column![
            widget::text(self.value).size(50),
            widget::button("+").on_press(Message::Increment).padding(10),
        ]
        .spacing(20)
        .padding(30)
        .into()
    }
}

fn main() -> iced::Result {
    Counter::run(iced::Settings::default())
}
