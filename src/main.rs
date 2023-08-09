use iced::font::{Family, Stretch, Weight};
use iced::widget::{button, column, text};
use iced::{Alignment, Element, Font, Sandbox, Settings};

#[cfg(any(target_family = "windows", target_os = "macos"))]
static SANS_SERIF_FONT_REGULAR_POSTSCRIPT_NAME: &'static str = "仿宋";
#[cfg(not(any(target_family = "windows", target_os = "macos")))]
static SANS_SERIF_FONT_REGULAR_POSTSCRIPT_NAME: &'static str = "DejaVuSans";

pub fn main() -> iced::Result {
    let font = Font {
        family: Family::Name(SANS_SERIF_FONT_REGULAR_POSTSCRIPT_NAME),
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        monospaced: false,
    };
    Counter::run(Settings {
        id: None,
        window: Default::default(),
        flags: Default::default(),
        default_font: font,
        default_text_size: 16.0,
        antialiasing: false,
        exit_on_close_request: true,
    })
}

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced 你好")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("加一").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("减一").on_press(Message::DecrementPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
