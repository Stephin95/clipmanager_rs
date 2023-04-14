
use std::process::Child;
mod toast;
use iced::Length;
use iced::executor;
use iced::widget::column;
use iced::widget::scrollable;
use iced::widget::Button;
use iced::widget::Column;
use iced::widget::text;
use iced::Application;
use iced::Theme;
use iced::{Alignment, Command, Element, Settings};
use crate::clip_db::ClipboardEntry;
mod gui_helpers;
use gui_helpers::{Message,format_button_text};
use toast::{Status, Toast};



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
    toasts:Vec<Toast>,
    cliphist: Vec<ClipboardEntry>,
    formatted_cliphist: Vec<ClipboardEntry>,
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
                toasts: Vec::new(),            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Clipboard Manager")
    }

    fn update<'a>(&'a mut self, message: Message) -> Command<Message> {
        match message {
            Message::Entry(copied) => {
                for clip_text in &*self.cliphist{
                    if copied==clip_text.id{
                        if let Err(err)=set_clipboard(clip_text.clip_text.as_str()){
                           println!("{:?}",err); 
                        self.toasts.push( Toast{
                    title: "Error".into(),
                    body: "Error copying the value to clipboard, Make sure wl-copy is installed".into(),
                    status: Status::Danger,
                });
                        };
                        self.toasts.push( Toast{
                    title: "Copied to clipboard".into(),
                    body: clip_text.clip_text.clone().into(),
                    status: Status::Primary,
                });
                    }
                }

            },
            Message::ToastClose(index)=>{
                self.toasts.remove(index);
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
        let content:Element<Message> =scrollable(
            column_custom
                .padding(5)
                .align_items(Alignment::Center)
                .spacing(1),
        )
        .into();
         toast::Manager::new(content, &self.toasts, Message::ToastClose)
            .timeout(3)
            .into()
    }
}

fn set_clipboard(text: &str)->Result<Child,std::io::Error> {
    std::process::Command::new("wl-copy")
        .arg(text)
        .spawn()
        
}
