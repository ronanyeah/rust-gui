use iced::{Subscription, Task, widget};

struct State {
    value: i32,
    width: i32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Overwrite(i32),
}

fn main() -> iced::Result {
    iced::application(|| State { value: 0, width: 0 }, update, view)
        .subscription(subscription)
        .title("Rust GUI")
        .run()
}

fn view(state: &State) -> iced::Element<'_, Message> {
    widget::column![
        widget::text(state.value).size(50),
        widget::button("+").on_press(Message::Increment).padding(10),
        widget::row![
            widget::text("width:").size(20),
            widget::text(state.width).size(20)
        ]
        .spacing(20)
    ]
    .spacing(20)
    .padding(30)
    .into()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Increment => {
            state.value += 1;
            Task::none()
        }
        Message::Overwrite(n) => {
            state.width = n;
            Task::none()
        }
    }
}

fn subscription(_state: &State) -> Subscription<Message> {
    iced::window::resize_events().map(|(_id, size)| Message::Overwrite(size.width as i32))
}
