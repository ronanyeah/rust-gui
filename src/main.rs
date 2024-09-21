use iced::{widget, Subscription, Task};

struct State {
    value: i32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Overwrite(i32),
}

fn main() -> iced::Result {
    let init_state = State { value: 0 };
    iced::application("Rust GUI", update, view)
        .subscription(subscription)
        .run_with(|| (init_state, Task::none()))
}

fn view(state: &State) -> iced::Element<Message> {
    widget::column![
        widget::text(state.value).size(50),
        widget::button("+").on_press(Message::Increment).padding(10),
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
            state.value = n;
            Task::none()
        }
    }
}

fn subscription(_state: &State) -> Subscription<Message> {
    iced::window::resize_events().map(|(_id, size)| Message::Overwrite(size.width as i32))
}
