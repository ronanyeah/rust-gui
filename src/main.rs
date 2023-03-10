use iced::{widget, Application, Command};

struct Counter {
    value: i32,
}

#[derive(serde::Serialize, Clone)]
struct Data {
    value: i32,
}

pub fn main() -> iced::Result {
    //let client = reqwest::Client::new();
    //let mut env = envy::from_env::<Env>().unwrap();
    //env.hasura_endpoint.set_path("/v1/graphql");
    Counter::run(iced::Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
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
        String::from("Dashboard")
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            //Message::FetchContent => {
            //self.value += 1;
            //Command::perform(sleep(Duration::from_secs(2)), Message::FetchCb)
            //Command::none()
            //}
            Message::IncrementPressed => {
                self.value += 1;
                Command::none()
            }
        }
    }
    //pub fn view(&mut self) -> Column<Message> {
    fn view(self: &Counter) -> iced::Element<Message> {
        let d = Data { value: 999 };
        let xs = vec![
            d.clone(),
            d.clone(),
            d.clone(),
            d.clone(),
            d.clone(),
            d.clone(),
            d.clone(),
            d.clone(),
            d.clone(),
        ];
        let cnts = widget::text(serde_json::to_string_pretty(&xs).unwrap())
            .size(30)
            .width(iced::Length::Fixed(300.0));
        let scrol = widget::scrollable(cnts)
            //.id(Variant::id(i))
            //.width(iced::Length::Fill)
            .height(iced::Length::Fixed(300.0));
        //.on_scroll(move |offset| Message::Scrolled(i, offset));
        widget::column![
            scrol,
            widget::text(self.value).size(50),
            widget::button("+").on_press(Message::IncrementPressed),
        ]
        .width(iced::Length::Fill)
        .into()

        //iced::widget::text("hi there").into()
    }
}
