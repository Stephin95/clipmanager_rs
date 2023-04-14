
use iced::executor;
use iced::widget::scrollable;
use iced::widget::Button;
use iced::widget::Column;
use iced::Application;
use iced::Theme;
use iced::{Alignment, Command, Element, Settings};
use std::ops::{Bound, RangeBounds};

use crate::clip_db::ClipboardEntry;


trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

fn slice_button_text<'a>(clip_text: String) -> String {
    let string_length = clip_text.len();
    let max_slice_length=50;
    if string_length > max_slice_length {
        format!("{}{}",clip_text.replace("\n", " ").slice(..max_slice_length).to_owned(),"...")
    } else {
        clip_text.replace("\n", " ")
    }
}

fn format_button_text(clipboard_entry: &Vec<ClipboardEntry>) -> Vec<ClipboardEntry> {
    clipboard_entry
        .iter()
        .map(|clip_entry| ClipboardEntry {
            clip_text: slice_button_text(clip_entry.clip_text.clone()),
            id: clip_entry.id.clone(),
        })
        .collect()
}

pub fn show<'a>(cliphist: Vec<ClipboardEntry>) -> iced::Result {


    ClipboardManager::run(Settings {
        window: iced::window::Settings {
            size: (300, 500),
            transparent: true,
            always_on_top: true,
            decorations: false,
            ..Default::default()
        },
        flags: cliphist,
        id: Default::default(),
        default_font: Default::default(),
        default_text_size: 14_f32,
        text_multithreading: true,
        antialiasing: Default::default(),
        exit_on_close_request: true,
        try_opengles_first: Default::default(),
        // ..Default::default()
    })
}

struct ClipboardManager {
    cliphist: Vec<ClipboardEntry>,
    formatted_cliphist: Vec<ClipboardEntry>,
}

#[derive(Debug, Clone)]
enum Message {
    Entry(i32),
}

impl Application for ClipboardManager {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Vec<ClipboardEntry>;
    fn new(flags: Vec<ClipboardEntry>) -> (Self, Command<Message>) {
        (
            Self {
                formatted_cliphist: format_button_text(&flags),
                cliphist: flags,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Clipboard Manager")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Entry(copied) => {
                for clip_text in &*self.cliphist{
                    if copied==clip_text.id{
                        set_clipboard(clip_text.clip_text.as_str());

                    }
                }

            }
        };
        Command::none()
    }

    fn view<'a>(&self) -> Element<Message> {
        // let cliphist_display = Vec::new();

        let buttons = self
            .formatted_cliphist
            .iter()
            // .rev()
            .map(|clip_entry| {
                Button::new(clip_entry.clip_text.as_str())
                    .on_press(Message::Entry(clip_entry.id.clone()))
                    .width(iced::Length::Fill)
            })
            .collect::<Vec<iced::widget::Button<Message>>>();
        let column_custom = buttons
            .into_iter()
            .fold(Column::new(), |column_custom, button_ind| {
                column_custom.push(button_ind)
            });
        scrollable(
            column_custom
                .padding(5)
                .align_items(Alignment::Center)
                .spacing(1),
        )
        .into()
    }
}

fn set_clipboard(text: &str) {
    std::process::Command::new("wl-copy")
        .arg(text)
        .spawn()
        .expect("failed to copy value to clipboard please make sure wl-copy is installed");
}
