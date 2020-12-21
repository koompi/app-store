#[macro_use]
use vedas_core::*;

use iced::{Color, Element, Error, Sandbox, Settings};

fn main() -> Result<(), Error> {
    OpenStore::run(Settings::default())
}

component!(OpenStore: "Open Store" => page: String);
message!(AppMessage, SwitchPage);

impl Sandbox for OpenStore {
    type Message = AppMessage;

    f!(new, Self, { Self::default() });
    f_ref_self!(self, title, String, { String::from("Open Store") });
    f_ref_mut_self!(self, update, message: AppMessage, {
        match message {
            AppMessage::SwitchPage => {}
        }
    });

    f_ref_mut_self!(self, view, Element<AppMessage>, {
        let main_layout = col!(fill!()).push(text!("Open Store").size(24u16).width(units!(100)));

        container!(units!(300), fill!(), main_layout)
            .style(MainContainerStyle)
            .into()
    });
}

style_container!(MainContainerStyle {
    text_color: Some(Color::BLACK),
    background: Some(iced::Background::Color(Color::from_rgb8(230, 230, 230))),
    border_radius: 0.,
    border_width: 1.,
    border_color: Color::from_rgb8(230, 230, 230)
});
