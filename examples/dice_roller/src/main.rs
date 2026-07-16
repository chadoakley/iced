use std::future::poll_fn;
use std::task::Poll;

use iced::widget::{button, center, column, text};
use iced::{Alignment, Element, Task};

pub fn main() -> iced::Result {
    iced::application(DiceRoller::new, DiceRoller::update, DiceRoller::view)
        .run()
}

struct DiceRoller {
    dice: Option<(u8, u8, u8)>,
    status: String,
}

#[derive(Debug, Clone)]
enum Message {
    Roll,
    Rolled((u8, u8, u8)),
}

impl DiceRoller {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                dice: None,
                status: String::from("click Roll"),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Roll => Task::perform(roll_dice(), Message::Rolled),
            Message::Rolled((a, b, c)) => {
                self.dice = Some((a, b, c));
                self.status = format!("total: {}", a as u32 + b as u32 + c as u32);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let dice_display = match self.dice {
            Some((a, b, c)) => format!("{} {} {}", a, b, c),
            None => String::from("- - -"),
        };
        center(
            column![
                text(dice_display).size(48),
                text(&self.status).size(14),
                button("Roll").on_press(Message::Roll).padding(12),
            ]
            .spacing(20)
            .align_x(Alignment::Center),
        )
        .into()
    }
}

async fn roll_dice() -> (u8, u8, u8) {
    let mut buf = [0u8; 3];
    getrandom::fill(&mut buf).expect("entropy source failed");

    let mut mixer: u64 = ((buf[0] as u64) << 16) | ((buf[1] as u64) << 8) | (buf[2] as u64);
    for i in 0..200_000u64 {
        mixer = mixer.wrapping_mul(1103515245).wrapping_add(12345).wrapping_add(i);
        cooperative_yield().await;
    }

    buf[0] = mixer as u8;
    buf[1] = (mixer >> 8) as u8;
    buf[2] = (mixer >> 16) as u8;

    ((buf[0] % 6) + 1, (buf[1] % 6) + 1, (buf[2] % 6) + 1)
}

async fn cooperative_yield() {
    let mut yielded = false;
    poll_fn(|cx| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
    .await
}
