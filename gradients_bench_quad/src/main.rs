use iced::theme::Container;
use iced::widget::{column, container, horizontal_space, row};
use iced::{
    window, Application, Color, Command, Element, Length, Radians, Renderer,
    Settings, Theme,
};
use std::f32::consts::PI;

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
struct Bench;

struct ContainerStyle([Color; 8], Radians);

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: iced::Gradient::linear(self.1)
                .add_stop(0.0, self.0[0])
                .add_stop(0.125, self.0[1])
                .add_stop(0.250, self.0[2])
                .add_stop(0.375, self.0[3])
                .add_stop(0.5, self.0[4])
                .add_stop(0.625, self.0[5])
                .add_stop(0.750, self.0[6])
                .add_stop(0.875, self.0[7])
                .build()
                .expect("Build gradient")
                .into(),
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
        "Gradient Bench".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        //renders 5151 gradients

        let gradient = |color_mul: f32, angle: Radians| {
            Element::from(
                container(horizontal_space(Length::Units(20)))
                    .height(Length::Fill)
                    .style(Container::Custom(Box::new(ContainerStyle(
                        [
                            Color::from_rgb(
                                1.0 * color_mul,
                                0.1 * color_mul,
                                0.1 * color_mul,
                            ),
                            Color::from_rgb(
                                0.9 * color_mul,
                                0.2 * color_mul,
                                0.2 * color_mul,
                            ),
                            Color::from_rgb(
                                0.8 * color_mul,
                                0.3 * color_mul,
                                0.3 * color_mul,
                            ),
                            Color::from_rgb(
                                0.7 * color_mul,
                                0.4 * color_mul,
                                0.4 * color_mul,
                            ),
                            Color::from_rgb(
                                0.6 * color_mul,
                                0.5 * color_mul,
                                0.5 * color_mul,
                            ),
                            Color::from_rgb(
                                0.5 * color_mul,
                                0.6 * color_mul,
                                0.6 * color_mul,
                            ),
                            Color::from_rgb(
                                0.4 * color_mul,
                                0.7 * color_mul,
                                0.7 * color_mul,
                            ),
                            Color::from_rgb(
                                0.3 * color_mul,
                                0.8 * color_mul,
                                0.8 * color_mul,
                            ),
                        ],
                        angle,
                    )))),
            )
        };

        let mut rows = Vec::<Element<Self::Message>>::new();

        let mut c = 1;
        while c < 51 {
            let mut r = 1;
            let mut gradients = vec![];

            while r < 101 {
                gradients
                    .push(gradient(r as f32 / 100.0, Radians(PI * (c as f32))));
                r += 1;
            }

            rows.push(Element::from(
                row(gradients).width(Length::Fill).height(Length::Units(20)),
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
