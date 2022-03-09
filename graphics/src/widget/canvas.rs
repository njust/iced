//! Draw 2D graphics for your users.
//!
//! A [`Canvas`] widget can be used to draw different kinds of 2D shapes in a
//! [`Frame`]. It can be used for animation, data visualization, game graphics,
//! and more!
use crate::renderer::{self, Renderer};
use crate::{Backend, Primitive};

use iced_native::layout;
use iced_native::mouse;
use iced_native::{
    Clipboard, Element, Layout, Length, Point, Rectangle, Shell, Size, Vector,
    Widget,
};

pub mod event;
pub mod path;

mod cache;
mod cursor;
mod fill;
mod frame;
mod geometry;
mod program;
mod stroke;
mod text;

pub use cache::Cache;
pub use cursor::Cursor;
pub use event::Event;
pub use fill::{Fill, FillRule};
pub use frame::Frame;
pub use geometry::Geometry;
pub use path::Path;
pub use program::Program;
pub use stroke::{LineCap, LineDash, LineJoin, Stroke};
pub use text::Text;

/// A widget capable of drawing 2D graphics.
///
/// # Examples
/// The repository has a couple of [examples] showcasing how to use a
/// [`Canvas`]:
///
/// - [`clock`], an application that uses the [`Canvas`] widget to draw a clock
/// and its hands to display the current time.
/// - [`game_of_life`], an interactive version of the Game of Life, invented by
/// John Conway.
/// - [`solar_system`], an animated solar system drawn using the [`Canvas`] widget
/// and showcasing how to compose different transforms.
///
/// [examples]: https://github.com/hecrj/iced/tree/master/examples
/// [`clock`]: https://github.com/hecrj/iced/tree/master/examples/clock
/// [`game_of_life`]: https://github.com/hecrj/iced/tree/master/examples/game_of_life
/// [`solar_system`]: https://github.com/hecrj/iced/tree/master/examples/solar_system
///
/// ## Drawing a simple circle
/// If you want to get a quick overview, here's how we can draw a simple circle:
///
/// ```no_run
/// # mod iced {
/// #     pub use iced_graphics::canvas;
/// #     pub use iced_native::{Color, Rectangle};
/// # }
/// use iced::canvas::{self, Canvas, Cursor, Fill, Frame, Geometry, Path, Program};
/// use iced::{Color, Rectangle};
///
/// // First, we define the data we need for drawing
/// #[derive(Debug)]
/// struct Circle {
///     radius: f32,
/// }
///
/// // Then, we implement the `Program` trait
/// impl Program for Circle {
///     type Message = ();
///
///     fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
///         // We prepare a new `Frame`
///         let mut frame = Frame::new(bounds.size());
///
///         // We create a `Path` representing a simple circle
///         let circle = Path::circle(frame.center(), self.radius);
///
///         // And fill it with some color
///         frame.fill(&circle, Color::BLACK);
///
///         // Finally, we produce the geometry
///         vec![frame.into_geometry()]
///     }
/// }
///
/// // Finally, we simply use our `Circle` to create the `Canvas`!
/// let canvas = Canvas::new(Circle { radius: 50.0 });
/// ```
#[derive(Debug)]
pub struct Canvas<P: Program> {
    width: Length,
    height: Length,
    program: P,
}

impl<P: Program> Canvas<P> {
    const DEFAULT_SIZE: u16 = 100;

    /// Creates a new [`Canvas`].
    pub fn new(program: P) -> Self {
        Canvas {
            width: Length::Units(Self::DEFAULT_SIZE),
            height: Length::Units(Self::DEFAULT_SIZE),
            program,
        }
    }

    /// Sets the width of the [`Canvas`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Canvas`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

impl<P, B> Widget<P::Message, Renderer<B>> for Canvas<P>
where
    P: Program,
    B: Backend,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        _renderer: &Renderer<B>,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn on_event(
        &mut self,
        event: iced_native::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer<B>,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, P::Message>,
    ) -> event::Status {
        let bounds = layout.bounds();

        let canvas_event = match event {
            iced_native::Event::Mouse(mouse_event) => {
                Some(Event::Mouse(mouse_event))
            }
            iced_native::Event::Keyboard(keyboard_event) => {
                Some(Event::Keyboard(keyboard_event))
            }
            _ => None,
        };

        let cursor = Cursor::from_window_position(cursor_position);

        if let Some(canvas_event) = canvas_event {
            let (event_status, message) =
                self.program.update(canvas_event, bounds, cursor);

            if let Some(message) = message {
                shell.publish(message);
            }

            return event_status;
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer<B>,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let cursor = Cursor::from_window_position(cursor_position);

        self.program.mouse_interaction(bounds, cursor)
    }

    fn draw(
        &self,
        renderer: &mut Renderer<B>,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        use iced_native::Renderer as _;

        let bounds = layout.bounds();

        if bounds.width < 1.0 || bounds.height < 1.0 {
            return;
        }

        let translation = Vector::new(bounds.x, bounds.y);
        let cursor = Cursor::from_window_position(cursor_position);

        renderer.with_translation(translation, |renderer| {
            renderer.draw_primitive(Primitive::Group {
                primitives: self
                    .program
                    .draw(bounds, cursor)
                    .into_iter()
                    .map(Geometry::into_primitive)
                    .collect(),
            });
        });
    }
}

impl<'a, P, B> From<Canvas<P>> for Element<'a, P::Message, Renderer<B>>
where
    P::Message: 'static,
    P: Program + 'a,
    B: Backend,
{
    fn from(canvas: Canvas<P>) -> Element<'a, P::Message, Renderer<B>> {
        Element::new(canvas)
    }
}
