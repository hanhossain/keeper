use iced::widget::{Column, Grid, button, column, container, row, text};
use iced::{Element, Task};
use serde::Deserialize;

fn main() -> iced::Result {
    tracing_subscriber::fmt().init();
    iced::run(update, view)
}

#[derive(Debug, Clone)]
enum Message {
    GetState,
    StateReceived(SleeperState),
}

fn update(counter: &mut u64, message: Message) -> Task<Message> {
    match message {
        Message::GetState => Task::perform(get_state(), Message::StateReceived),
        Message::StateReceived(state) => {
            tracing::info!(?state, "received state");
            Task::none()
        }
    }
}

fn view(counter: &u64) -> Element<'_, Message> {
    button(text(counter)).on_press(Message::GetState).into()
}

#[derive(Debug, Deserialize, Clone)]
struct SleeperState {
    season: String,
    week: i32,
}

async fn get_state() -> SleeperState {
    reqwest::get("http://localhost:8080/admin/state")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
