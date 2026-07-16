use iced::widget::{button, center, column, text};
use iced::{Alignment, Element, Task};

pub fn main() -> iced::Result {
    iced::application(Counter::new, Counter::update, Counter::view)
        .run()
}

struct Counter {
    value: i32,
    label: String,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

impl Counter {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                value: 0,
                label: String::from("(no rolls yet)"),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => {
                let (next, label) = futures::executor::block_on(compute_next(self.value));
                self.value = next;
                self.label = label;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        center(
            column![
                text(format!("Count: {}", self.value)),
                text(&self.label).size(14),
                button("Increment").on_press(Message::Increment).padding(12),
            ]
            .spacing(20)
            .align_x(Alignment::Center),
        )
        .into()
    }
}

async fn compute_next(current: i32) -> (i32, String) {
    let next = current + 1;
    (next, format!("just incremented to {}", next))
}
