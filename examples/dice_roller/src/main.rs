mod animation;

use iced::widget::{button, center, column, text};
use iced::{Alignment, Element, Task};

pub fn main() -> iced::Result {
    iced::application(DiceRoller::new, DiceRoller::update, DiceRoller::view)
        .run()
}

const ROLLING_FRAMES: usize = 25;

struct DiceRoller {
    dice: (u8, u8, u8),
    rolling: bool,
    status: String,
}

#[derive(Debug, Clone)]
enum Message {
    Roll,
    Tick((u8, u8, u8)),
    Settled((u8, u8, u8)),
}

impl DiceRoller {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                dice: (0, 0, 0),
                rolling: false,
                status: String::from("click Roll"),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Roll => {
                self.rolling = true;
                self.status = String::from("rolling…");
                Task::stream(roll_stream())
            }
            Message::Tick(next) => {
                self.dice = next;
                Task::none()
            }
            Message::Settled(next) => {
                self.dice = next;
                self.rolling = false;
                self.status = format!(
                    "total: {}",
                    next.0 as u32 + next.1 as u32 + next.2 as u32
                );
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let (a, b, c) = self.dice;
        let display = if a == 0 && b == 0 && c == 0 {
            String::from("- - -")
        } else {
            format!("{} {} {}", a, b, c)
        };
        center(
            column![
                text(display).size(48),
                text(&self.status).size(14),
                button("Roll").on_press(Message::Roll).padding(12),
            ]
            .spacing(20)
            .align_x(Alignment::Center),
        )
        .into()
    }
}

fn roll_stream() -> impl futures::Stream<Item = Message> + Send + 'static {
    use futures::stream::StreamExt;

    futures::stream::iter(0..ROLLING_FRAMES).then(|i| async move {
        let mut buf = [0u8; 3];
        getrandom::fill(&mut buf).expect("entropy source failed");
        animation::mix_entropy(&mut buf);
        let dice = ((buf[0] % 6) + 1, (buf[1] % 6) + 1, (buf[2] % 6) + 1);
        animation::advance().await;
        if i + 1 == ROLLING_FRAMES {
            Message::Settled(dice)
        } else {
            Message::Tick(dice)
        }
    })
}
