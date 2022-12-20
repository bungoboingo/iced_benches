use crate::canvas::{Canvas, Cursor, Fill, Geometry};
use iced::widget::canvas::{self, Cache};
use iced::{
    window, Application, Color, Command, Element, Length, Point, Rectangle,
    Renderer, Settings, Size, Theme,
};

fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    Bench::run(Settings {
        window: window::Settings {
            size: (2000, 1000),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug)]
struct Bench {
    cache: Cache,
}

impl Application for Bench {
    type Executor = iced::executor::Default;
    type Message = ();
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Bench {
                cache: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Gradient Bench".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<Message> canvas::Program<Message> for Bench {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        //renders 5151 gradients
        let gradient = |color_mul: f32, bounds: &Rectangle| {
            canvas::Gradient::linear(
                bounds.position(),
                Point::new(
                    bounds.x + bounds.size().width,
                    bounds.y + bounds.size().height,
                ),
            )
                .add_stop(
                    0.0,
                    Color::from_rgb(
                        1.0 * color_mul,
                        0.1 * color_mul,
                        0.1 * color_mul,
                    ),
                )
                .add_stop(
                    0.125,
                    Color::from_rgb(
                        0.9 * color_mul,
                        0.2 * color_mul,
                        0.2 * color_mul,
                    ),
                )
                .add_stop(
                    0.250,
                    Color::from_rgb(
                        0.8 * color_mul,
                        0.3 * color_mul,
                        0.3 * color_mul,
                    ),
                )
                .add_stop(
                    0.375,
                    Color::from_rgb(
                        0.7 * color_mul,
                        0.4 * color_mul,
                        0.4 * color_mul,
                    ),
                )
                .add_stop(
                    0.5,
                    Color::from_rgb(
                        0.6 * color_mul,
                        0.5 * color_mul,
                        0.5 * color_mul,
                    ),
                )
                .add_stop(
                    0.625,
                    Color::from_rgb(
                        0.5 * color_mul,
                        0.6 * color_mul,
                        0.6 * color_mul,
                    ),
                )
                .add_stop(
                    0.750,
                    Color::from_rgb(
                        0.4 * color_mul,
                        0.7 * color_mul,
                        0.7 * color_mul,
                    ),
                )
                .add_stop(
                    0.875,
                    Color::from_rgb(
                        0.3 * color_mul,
                        0.8 * color_mul,
                        0.8 * color_mul,
                    ),
                )
                .build()
                .expect("Build Gradient")
        };

        let geometry = self.cache.draw(bounds.size(), |frame| {
            let g_size: f32 = 20.0;

            let mut total_count = 0;
            let mut vert_offset = 0.0;

            while vert_offset <= bounds.height {
                let mut hor_position = Point::new(0.0, vert_offset);

                let mut r = 1;
                while hor_position.x <= bounds.width {
                    let b = Rectangle::new(
                        Point::new(hor_position.x, vert_offset),
                        Size::new(g_size, g_size),
                    );

                    frame.fill_rectangle(
                        b.position(),
                        b.size(),
                        Fill {
                            style: gradient(r as f32 / 200.0, &b).into(),
                            ..Default::default()
                        },
                    );
                    total_count += 1;
                    r += 1;
                    hor_position.x += g_size;
                }

                vert_offset += g_size;
            }

            println!("Created {} gradients", total_count);
        });

        vec![geometry]
    }
}
