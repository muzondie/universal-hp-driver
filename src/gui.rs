use iced::{
    Application, Command, Element, Settings, Text, Column, 
    Container, Length, button, Button
};
use crate::detection::{self, DeviceInfo};

pub struct App {
    devices: Vec<DeviceInfo>,
    install_button: button::State,
    status: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ScanDevices,
    InstallDrivers,
}

impl Application for App {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self {
            devices: Vec::new(),
            install_button: button::State::new(),
            status: String::from("Ready"),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Universal HP Driver")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ScanDevices => {
                self.devices = detection::detect_hardware();
                self.status = format!("Found {} devices", self.devices.len());
            }
            Message::InstallDrivers => {
                self.status = String::from("Installing...");
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .push(Text::new(&self.status).size(24))
            .push(Button::new(&mut self.install_button, Text::new("Install"))
                .on_press(Message::InstallDrivers));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}