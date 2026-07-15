use iced::widget::{button, center, column, text};
use iced::{Alignment, Element, Font, Task};

mod entropy;

pub fn main() -> iced::Result {
    iced::application(DiceRoller::new, DiceRoller::update, DiceRoller::view)
        .font(include_bytes!("../assets/DiceFont.ttf").as_slice())
        .default_font(Font::new("Intur-irregular"))
        .run()
}

#[derive(Default)]
struct DiceRoller {
    dice: Option<(u8, u8, u8)>,
}

impl DiceRoller {
    fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Roll => {
                self.dice = Some((roll_die(), roll_die(), roll_die()));
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let dice_display = match self.dice {
            Some((a, b, c)) => format!("{} {} {}", a, b, c),
            None => "- - -".to_string(),
        };
        center(
            column![
                text("3d6 Dice Roller").size(28),
                text(dice_display).size(48),
                button("Roll").on_press(Message::Roll).padding(12),
            ]
            .spacing(20)
            .align_x(Alignment::Center),
        )
        .into()
    }
}

fn roll_die() -> u8 {
    let mut buf = [0u8; 1];
    getrandom::fill(&mut buf).expect("entropy source failed");
    (buf[0] % 6) + 1
}

#[derive(Debug, Clone)]
enum Message {
    Roll,
}
