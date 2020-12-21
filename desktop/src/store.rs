use iced::Sandbox;
use vedas_core::macros::*;

component!(OpenStore: "Open Store" => page: String);
message!(AppMessage, SwitchPage);

impl Sandbox for OpenStore {
    type Message = AppMessage;

    f!(new, Self, { Self::default() });
}
