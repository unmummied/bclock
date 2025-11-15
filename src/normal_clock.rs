use chrono::{Duration, NaiveTime, Timelike};
use eframe::{
    App,
    egui::{Align2, CentralPanel, Color32, FontId, Painter, Pos2, Sense, Shape, Stroke, vec2},
};
use std::{f32::consts::TAU, time::Instant};

pub struct ClockApp {
    pub now: NaiveTime,
    pub speed: f32,
    last_update: Instant,
}

impl ClockApp {
    pub fn from(start: NaiveTime, speed: f32) -> Self {
        Self {
            now: start,
            speed,
            last_update: Instant::now(),
        }
    }
}

impl App for ClockApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let elapsed = self.last_update.elapsed().as_secs_f32();
            self.last_update = Instant::now();
            let delta_seconds = elapsed * self.speed;
            self.now += Duration::milliseconds((delta_seconds * 1000.0) as i64);

            let (rect, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());
            let rect = rect.rect;

            let center = rect.center();
            let radius = rect.height().min(rect.width()) * 0.4;

            draw_ticks(&painter, center, radius);
            draw_numbers(&painter, center, radius);

            let now = self.now;
            let (hour, minute, second): (f32, f32, f32) =
                (now.hour12().1 as _, now.minute() as _, now.second() as _);

            let (hour_ang, minute_ang, second_ang) = (
                hour / 12. * TAU - TAU / 4.,
                minute / 60. * TAU - TAU / 4.,
                second / 60. * TAU - TAU / 4.,
            );

            painter.circle_stroke(center, radius, (2., Color32::WHITE));

            painter.line_segment(
                [
                    center,
                    center + vec2(second_ang.cos(), second_ang.sin()) * radius * 0.9,
                ],
                (2., Color32::BLUE),
            );
            painter.line_segment(
                [
                    center,
                    center + vec2(minute_ang.cos(), minute_ang.sin()) * radius * 0.7,
                ],
                (3., Color32::GREEN),
            );
            painter.line_segment(
                [
                    center,
                    center + vec2(hour_ang.cos(), hour_ang.sin()) * radius * 0.5,
                ],
                (4., Color32::RED),
            );

            ui.ctx().request_repaint();
        });
    }
}

fn draw_ticks(painter: &Painter, center: Pos2, radius: f32) {
    for i in 0..60 {
        let ang = i as f32 / 60. * TAU - TAU / 4.;

        let outer = center + vec2(ang.cos(), ang.sin()) * radius;
        let inner = if i % 5 == 0 {
            center + vec2(ang.cos(), ang.sin()) * radius * 0.85
        } else {
            center + vec2(ang.cos(), ang.sin()) * radius * 0.92
        };
        let stroke = if i % 5 == 0 {
            Stroke::new(2., Color32::WHITE)
        } else {
            Stroke::new(1., Color32::GRAY)
        };

        painter.add(Shape::line_segment([inner, outer], stroke));
    }
}

fn draw_numbers(painter: &Painter, center: Pos2, radius: f32) {
    for i in 0..60 {
        let ang = i as f32 / 60. * TAU - TAU / 4.;

        let number = if i % 5 == 0 { i / 5 } else { i };
        let pos = if i % 5 == 0 {
            center + vec2(ang.cos(), ang.sin()) * radius * 0.72
        } else {
            center + vec2(ang.cos(), ang.sin()) * radius * 0.85
        };
        let stroke = if i % 5 == 0 { 0.13 } else { 0.07 };

        painter.text(
            pos,
            Align2::CENTER_CENTER,
            number.to_string(),
            FontId::monospace(radius * stroke),
            Color32::WHITE,
        );
    }
}
