use crate::canvas::{Canvas, Cursor, Fill, Geometry};
use iced::widget::canvas::{self, Cache};
use iced::{
    window, Application, Color, Command, Element, Length, Point, Rectangle,
    Renderer, Settings, Size, Theme,
};

fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    Bench::run(Settings {
        try_opengles_first: true,
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
        "Solid Triangle Bench".to_string()
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
        //renders 5151 solids
        let geometry = self.cache.draw(bounds.size(), |frame| {
            let solid_size: f32 = 20.0;

            let mut total_count = 0;
            let mut vert_offset = 0.0;

            while vert_offset <= bounds.height {
                let mut hor_position = Point::new(0.0, vert_offset);

                let mut r = 1;
                while hor_position.x <= bounds.width {
                    let b = Rectangle::new(
                        Point::new(hor_position.x, vert_offset),
                        Size::new(solid_size, solid_size),
                    );

                    let color_mul = r as f32 / 200.0;

                    frame.fill_rectangle(
                        b.position(),
                        b.size(),
                        Fill {
                            style: Color::from_rgb(
                                1.0 * color_mul,
                                0.8 * color_mul,
                                0.9 * color_mul,
                            )
                                .into(),
                            ..Default::default()
                        },
                    );
                    total_count += 1;
                    r += 1;
                    hor_position.x += solid_size;
                }

                vert_offset += solid_size;
            }

            println!("Created {} solids", total_count);
        });

        vec![geometry]
    }
}
