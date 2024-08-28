use iced::{
  executor, widget::text, Application, Command, Element, Settings, Theme,
};

pub(super) struct Flags {}

pub(super) fn run(flags: Flags) -> anyhow::Result<()> {
  App::run(Settings::with_flags(flags))?;

  Ok(())
}

struct App {}

#[derive(Debug, Clone)]
enum AppMessage {}

impl Application for App {
  type Executor = executor::Default;
  type Flags = Flags;
  type Message = AppMessage;
  type Theme = Theme;

  fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (Self {}, Command::none())
  }

  fn title(&self) -> String {
    String::from("App - Audio thing")
  }

  fn update(&mut self, message: Self::Message) -> iced::Command<AppMessage> {
    match message {}
  }

  fn view(&self) -> Element<Self::Message> {
    text("Hello World!").into()
  }
}
