use iced::theme::Container;
use iced::widget::{column, container, horizontal_space, row};
use iced::{
    window, Application, Color, Command, Element, Length, Renderer, Settings,
    Theme,
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
struct Bench;

struct ContainerStyle(Color);

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: self.0.into(),
            ..Default::default()
        }
    }
}

impl Application for Bench {
    type Executor = iced::executor::Default;
    type Message = ();
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Bench, Command::none())
    }

    fn title(&self) -> String {
        "Solid Bench".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let solid = |color_mul: f32| {
            Element::from(
                container(horizontal_space(Length::Units(20)))
                    .height(Length::Fill)
                    .style(Container::Custom(Box::new(ContainerStyle(
                        Color::from_rgb(
                            1.0 * color_mul,
                            0.8 * color_mul,
                            0.7 * color_mul,
                        ),
                    )))),
            )
        };

        let mut rows = Vec::<Element<Self::Message>>::new();

        let mut c = 1;
        while c < 51 {
            let mut r = 1;
            let mut solids = vec![];

            while r < 101 {
                solids.push(solid(r as f32 / 100.0));
                r += 1;
            }

            rows.push(Element::from(
                row(solids).width(Length::Fill).height(Length::Units(20)),
            ));

            c += 1;
        }

        let content = column(rows).width(Length::Fill).height(Length::Fill);

        container(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_y()
            .center_x()
            .into()
    }
}
