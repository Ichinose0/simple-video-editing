use igat::{
    menu::Menubar,
    menu::Menu,
    widget::{build_component, Button, Component, Text},
    Application, ApplicationEvent, Color, Executable, Theme, Frame,
};
use igat::ApplicationResponse;

#[derive(Clone, Copy, Debug)]
pub enum Message {
    ButtonClicked,
}

pub struct VideoEditor {
    theme: Theme,
    menu: Menubar,
}

impl Application<Message> for VideoEditor {
    type Message = Message;

    fn on_close(&self) {}

    fn set_up(&mut self) {}

    fn message(
        &mut self,
        event: ApplicationEvent,
        _message: Option<Message>,
        frame: &Frame
    ) -> Option<ApplicationResponse> {
        None
    }

    fn menu(&self) -> Option<&igat::menu::Menubar> {
        Some(&self.menu)
    }

    fn ui(&mut self,frame: &Frame) -> Option<Component<Message>> {
        let button = Button::new(frame)
            .width(70)
            .height(30)
            .text("Button".to_string())
            .on_click(Message::ButtonClicked);
        Some(build_component(button))
    }

    fn theme(&self) -> igat::Theme {
        self.theme
    }
}

fn main() {
    let exe = Executable::new();

    let mut menubar = Menubar::new();
    menubar.add(Menu::new("File".to_string()));
    menubar.add(Menu::new("Edit".to_string()));
    menubar.add(Menu::new("Encode".to_string()));
    let app = VideoEditor {
        theme: Theme::default(),
        menu: menubar,
    };
    exe.run(app);
}