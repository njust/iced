use std::{f32::consts::PI, time::Instant};

use iced::executor;
use iced::pure::widget::canvas::{
    self, Cache, Canvas, Cursor, Geometry, Path, Stroke,
};
use iced::pure::{Application, Element};
use iced::{
    Color, Command, Length, Point, Rectangle, Settings, Subscription, Theme,
};

pub fn main() -> iced::Result {
    Arc::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Arc {
    start: Instant,
    cache: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
}

impl Application for Arc {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Arc {
                start: Instant::now(),
                cache: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Arc - Iced")
    }

    fn update(&mut self, _: Message) -> Command<Message> {
        self.cache.clear();

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(10))
            .map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl<Message> canvas::Program<Message> for Arc {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 5.0;

            let start = Point::new(center.x, center.y - radius);

            let angle = (self.start.elapsed().as_millis() % 10_000) as f32
                / 10_000.0
                * 2.0
                * PI;

            let end = Point::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );

            let circles = Path::new(|b| {
                b.circle(start, 10.0);
                b.move_to(end);
                b.circle(end, 10.0);
            });

            frame.fill(&circles, Color::WHITE);

            let path = Path::new(|b| {
                b.move_to(start);
                b.arc_to(center, end, 50.0);
                b.line_to(end);
            });

            let palette = theme.palette();

            frame.stroke(
                &path,
                Stroke {
                    color: palette.text,
                    width: 10.0,
                    ..Stroke::default()
                },
            );
        });

        vec![geometry]
    }
}
