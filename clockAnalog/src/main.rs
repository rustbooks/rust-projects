// ─────────────────────────────────────────────────────────────────────────────
//  Analog Clock  ·  Rust + Iced 0.14
// ─────────────────────────────────────────────────────────────────────────────
use chrono::{Local, Timelike};
use iced::mouse;
use iced::widget::canvas::{self, Cache, Canvas, Frame, Geometry, Path, Stroke, Text};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Subscription, Theme};
use std::f32::consts::PI;
use std::time::Duration;

fn main() -> iced::Result {
    iced::application(ClockApp::new, ClockApp::update, ClockApp::view)
        .title("Analog Clock")
        .antialiasing(true)
        .window_size((420.0, 460.0))
        .subscription(ClockApp::subscription)
        .run()
}

// ── State ─────────────────────────────────────────────────────────────────────

#[derive(Default)]
struct ClockApp {
    clock: ClockWidget,
}

impl ClockApp {
    fn new() -> (Self, iced::Task<Message>) {
        (Self::default(), iced::Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => self.clock.cache.clear(),
        }
    }

    fn view(&self) -> Element<Message> {
        Canvas::new(&self.clock)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(ticker)
    }
}

/// Async stream that yields Message::Tick every 500ms using tokio
fn ticker() -> impl iced::futures::Stream<Item = Message> {
    iced::futures::stream::unfold((), |()| async {
        tokio::time::sleep(Duration::from_millis(500)).await;
        Some((Message::Tick, ()))
    })
}

// ── Message ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
}

// ── Canvas Program ────────────────────────────────────────────────────────────

#[derive(Default)]
struct ClockWidget {
    cache: Cache,
}

impl canvas::Program<Message> for ClockWidget {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            draw_clock(frame);
        });
        vec![geometry]
    }
}

// ── Drawing ───────────────────────────────────────────────────────────────────

fn draw_clock(frame: &mut Frame) {
    let center = frame.center();
    let radius = frame.width().min(frame.height()) / 2.0 - 12.0;
    let now = Local::now();

    // Background
    let face = Path::circle(center, radius);
    frame.fill(&face, Color::from_rgb8(13, 15, 25));

    // Glow rings
    for i in 0..3 {
        let ring = Path::circle(center, radius - i as f32 * 2.0);
        frame.stroke(
            &ring,
            Stroke::default()
                .with_color(Color::from_rgba8(100, 149, 237, 0.12 + i as f32 * 0.08))
                .with_width(1.0),
        );
    }

    // Outer rim
    frame.stroke(
        &face,
        Stroke::default()
            .with_color(Color::from_rgb8(80, 130, 220))
            .with_width(3.5),
    );

    // Hour ticks
    for h in 0..12 {
        let angle = h as f32 * (2.0 * PI / 12.0) - PI / 2.0;
        let (len, w, col) = if h % 3 == 0 {
            (radius * 0.13, 3.0, Color::from_rgb8(100, 149, 237))
        } else {
            (radius * 0.07, 1.5, Color::from_rgba8(140, 170, 220, 0.8))
        };
        frame.stroke(
            &Path::line(
                polar(center, radius * 0.90, angle),
                polar(center, radius * 0.90 - len, angle),
            ),
            Stroke::default().with_color(col).with_width(w),
        );
    }

    // Hour numbers
    for h in 1..=12 {
        let angle = h as f32 * (2.0 * PI / 12.0) - PI / 2.0;
        frame.fill_text(Text {
            content: format!("{}", h),
            position: polar(center, radius * 0.70, angle),
            color: Color::from_rgb8(190, 210, 245),
            size: iced::Pixels(radius * 0.115),
            align_x: iced::alignment::Horizontal::Center.into(),
            align_y: iced::alignment::Vertical::Center.into(),
            ..Text::default()
        });
    }

    // Minute dots
    for m in 0..60 {
        if m % 5 == 0 {
            continue;
        }
        let angle = m as f32 * (2.0 * PI / 60.0) - PI / 2.0;
        frame.fill(
            &Path::circle(polar(center, radius * 0.87, angle), 1.5),
            Color::from_rgba8(100, 149, 237, 0.35),
        );
    }

    // Time values
    let h = (now.hour() % 12) as f32;
    let m = now.minute() as f32;
    let s = now.second() as f32;
    let hour_angle = (h + m / 60.0) * (2.0 * PI / 12.0) - PI / 2.0;
    let min_angle = (m + s / 60.0) * (2.0 * PI / 60.0) - PI / 2.0;
    let sec_angle = s * (2.0 * PI / 60.0) - PI / 2.0;

    // Hour hand
    draw_hand(
        frame,
        center,
        hour_angle,
        radius * 0.48,
        radius * 0.038,
        Color::from_rgb8(220, 228, 255),
    );
    // Minute hand
    draw_hand(
        frame,
        center,
        min_angle,
        radius * 0.70,
        radius * 0.025,
        Color::from_rgb8(170, 200, 255),
    );

    // Second hand
    let red = Color::from_rgb8(255, 75, 75);
    frame.stroke(
        &Path::line(center, polar(center, radius * 0.18, sec_angle + PI)),
        Stroke::default().with_color(red).with_width(1.5),
    );
    let sec_tip = polar(center, radius * 0.83, sec_angle);
    frame.stroke(
        &Path::line(center, sec_tip),
        Stroke::default().with_color(red).with_width(1.5),
    );
    frame.fill(&Path::circle(sec_tip, 3.5), red);

    // Center hub
    frame.fill(&Path::circle(center, radius * 0.045), red);
    frame.fill(&Path::circle(center, radius * 0.020), Color::WHITE);

    // Digital time
    frame.fill_text(Text {
        content: now.format("%H:%M:%S").to_string(),
        position: Point {
            x: center.x,
            y: center.y + radius * 0.50,
        },
        color: Color::from_rgba8(150, 185, 255, 0.65),
        size: iced::Pixels(radius * 0.11),
        align_x: iced::alignment::Horizontal::Center.into(),
        align_y: iced::alignment::Vertical::Center.into(),
        ..Text::default()
    });

    // Date
    frame.fill_text(Text {
        content: now.format("%d %b %Y").to_string(),
        position: Point {
            x: center.x,
            y: center.y + radius * 0.63,
        },
        color: Color::from_rgba8(130, 165, 230, 0.50),
        size: iced::Pixels(radius * 0.09),
        align_x: iced::alignment::Horizontal::Center.into(),
        align_y: iced::alignment::Vertical::Center.into(),
        ..Text::default()
    });
}

fn polar(center: Point, r: f32, angle: f32) -> Point {
    Point {
        x: center.x + angle.cos() * r,
        y: center.y + angle.sin() * r,
    }
}

fn draw_hand(frame: &mut Frame, center: Point, angle: f32, length: f32, width: f32, color: Color) {
    let tip = polar(center, length, angle);
    let back = polar(center, length * 0.10, angle + PI);
    frame.stroke(
        &Path::line(back, tip),
        Stroke::default().with_color(color).with_width(width * 2.0),
    );
    frame.fill(&Path::circle(tip, width * 0.9), color);
}
