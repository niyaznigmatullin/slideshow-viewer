use iced::{image, Element, Column, Container, Length, Settings, Application, Command, window, Subscription};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::vec::IntoIter;
use iced::keyboard::KeyCode::Escape;
use native_dialog::{FileDialog};


fn main() -> iced::Result {
    State::run(Settings::default())
}

struct State {
    image: Option<image::Handle>,
    directory: PathBuf,
    images: IntoIter<PathBuf>,
    image_viewer: image::viewer::State,
    exit: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdatePicture(Instant),
    EscapePressed,
}

fn load_directory(path: &PathBuf) -> IntoIter<PathBuf> {
    match fs::read_dir(path) {
        Ok(read_dir) => {
            read_dir.map(|x| x.unwrap().path()).collect::<Vec<_>>().into_iter()
        },
        Err(e) => {
            eprintln!("Couldn't read directory {}, an error happened {}", path.as_path().to_str().unwrap_or(""), e);
            Vec::new().into_iter()
        }
    }
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let path = match FileDialog::new()
            .set_location(r"~")
            .show_open_single_dir() {
            Err(_) => {
                eprintln!("Couldn't open dialog window");
                None
            }
            Ok(path) => path
        };
        let directory = path.unwrap_or(PathBuf::from(r"C:\tmp\images"));
        let mut images = load_directory(&directory);
        let image = images.next()
            .map(|path| image::Handle::from_path(&path));
        (
            State {
                image,
                directory,
                images,
                image_viewer: image::viewer::State::new(),
                exit: false,
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        "Slideshow".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EscapePressed => {
                self.exit = true;
                Command::none()
            }
            Message::UpdatePicture(_) => {
                let path = match self.images.next() {
                    None => {
                        self.images = load_directory(&self.directory);
                        self.images.next()
                    }
                    other => other
                };
                self.image = path.map(|path| image::Handle::from_path(&path));
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let subscriptions = vec![iced_native::subscription::events_with(|event, _status| {
            if let iced_native::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key_code: Escape,
                modifiers: _ }) = event {
                Some(Message::EscapePressed)
            } else {
                None
            }
        }), iced::time::every(Duration::from_secs(5)).map(Message::UpdatePicture)];
        Subscription::batch(subscriptions)
    }

    fn view(&mut self) -> Element<Message> {
        let mut content = Column::new();
        if let Some(handle) = &self.image {
            content = content.push(image::Viewer::new(&mut self.image_viewer, handle.clone()).height(Length::Fill).width(Length::Fill));
        }
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn mode(&self) -> window::Mode {
        window::Mode::Fullscreen
    }

    fn should_exit(&self) -> bool {
        self.exit
    }
}