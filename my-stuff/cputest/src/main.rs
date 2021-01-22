use iced::{
    button, executor, time, Align, Application, Button, Column, Command,
    Container, Element, HorizontalAlignment, Length, Row, Settings,
    Subscription, Text,
};
use std::time::{Duration, Instant};
use sysinfo::{ProcessorExt, System, SystemExt};

pub fn main() -> iced::Result {
    Stopwatch::run(Settings::default())
}

struct Stopwatch {
    counter: u32,
    sysinfo: System,
    duration: Text,
    state: State,
    toggle: button::State,
    reset: button::State,
}

enum State {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
    Reset, // Remove. We don't need reset.
    Tick(Instant), // Replace 'Instant' with value of usage (returned by a function)
}

impl Application for Stopwatch {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Stopwatch, Command<Message>) {
        (
            Stopwatch {
                counter: 0,
                sysinfo: sysinfo::System::new_all(),
                duration: Text::new(format!("---"))
                .size(40), // Replace with output var (e.g. usage), with default of "---" string placeholder
                state: State::Idle,
                toggle: button::State::new(),
                reset: button::State::new(), // Remove.
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("CPU usage test")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(), // Remove: we are not calculating elapsed time
                    };
                }
                State::Ticking { .. } => {
                    self.state = State::Idle;
                }
            },
            Message::Tick(now) => match &mut self.state {
                State::Ticking { last_tick } => {
                    //println!("{:?}", now);
                    self.counter += 1;
                    self.duration = Text::new(format!("{}", self.counter))
                    .size(40);
                }
                _ => {}
            },
            // Remove, we don't need reset.
            Message::Reset => {
                //self.duration = Duration::default();
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        const TICK: u64 = 500; // Tick time step, in miliseconds.
        match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => {
                time::every(Duration::from_millis(TICK)).map(Message::Tick)
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut system = sysinfo::System::new_all();
        let mut num_cores: u8 = 0;
        let mut aves: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0, 0.0];
        let mut i: usize = 0;

        // Count virtual cores.
        for _processor in system.get_processors() {
            num_cores += 1;
        }

        // println!("cores: {}", num_cores);

        let mov_ave = |aves: Vec<f32>| -> f32 {
            let mut total: f32 = 0.0;

            for j in 0..aves.iter().count() {
                total += aves[j];
            }

            total / 5.0
        };

        let mut get_usage = | | -> f32 {
            system.refresh_all();

            let mut total: f32 = 0.0;

            for processor in system.get_processors() {
                total += processor.get_cpu_usage();
                println!("procs: {}", total);
            }

            aves[i] = total / num_cores as f32;

            println!("aves: {:?}", aves);
            println!("total: {}", total);
            println!("cores: {}", num_cores);
            println!("aves[i]: {}", aves[i]);

            let ave = mov_ave(aves.clone());

            i += 1;
            if i == 5 {
                i = 0;
            }

            ave
        };

        let duration = Text::new(format!("{}", self.counter))
        .size(40);

        let button = |state, label, style| {
            Button::new(
                state,
                Text::new(label)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .min_width(80)
            .padding(10)
            .style(style)
        };

        let toggle_button = {
            let (label, color) = match self.state {
                State::Idle => ("Start", style::Button::Primary),
                State::Ticking { .. } => ("Stop", style::Button::Destructive),
            };

            button(&mut self.toggle, label, color).on_press(Message::Toggle)
        };

        // Remove: we don't need a reset button
        let reset_button =
            button(&mut self.reset, "Reset", style::Button::Secondary)
                .on_press(Message::Reset);

        let controls = Row::new()
            .spacing(20)
            .push(toggle_button)
            .push(reset_button);

        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(duration)
            .push(controls);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary, // Remove. This is the unwanted 'reset' button.
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                    Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
