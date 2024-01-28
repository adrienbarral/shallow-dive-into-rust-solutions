use std::{sync::mpsc, time::Duration};

use iced::{
    executor,
    widget::{text, Container},
    Application, Command, Length, Settings, Theme,
};

pub fn init_window(rx: mpsc::Receiver<f32>) {
    let _ = State::run(Settings {
        antialiasing: true,
        ..Settings::with_flags(rx)
    });
}

#[derive(Debug)]
enum Message {
    Tick,
}

struct State {
    position: f32,
    rx: mpsc::Receiver<f32>,
}

impl Application for State {
    type Executor = executor::Default;
    type Message = self::Message;
    type Theme = Theme;
    type Flags = mpsc::Receiver<f32>;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                position: -1.0_f32,
                rx: flags,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Position Plotter".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Tick => {
                if let Ok(pos) = self.rx.try_recv() {
                    self.position = pos;
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = text(format!("Position : {}", self.position));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        const FPS: u64 = 25;
        iced::time::every(Duration::from_millis(1000 / FPS)).map(|_| Message::Tick)
    }
}
