use chrono::{Duration, NaiveTime, Timelike};
use eframe::{
    App,
    egui::{Align2, CentralPanel, Color32, FontId, Painter, Pos2, Sense, Shape, Stroke, vec2},
};
use std::{f32::consts::TAU, time::Instant};

pub struct BClockApp {
    pub now: NaiveTime,
    pub speed: f32,
    last_update: Instant,
}

impl BClockApp {
    pub fn from(start: NaiveTime, speed: f32) -> Self {
        Self {
            now: start,
            speed,
            last_update: Instant::now(),
        }
    }
}

impl App for BClockApp {
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

            let now = BinaiveTime::from(self.now);
            let (half, quarter, eighth, sixteenth): (f32, f32, f32, f32) = (
                now.half2().1 as _,
                now.quarter() as _,
                now.eighth() as _,
                now.sixteenth() as _,
            );

            let (half_ang, quarter_ang, eighth_ang, sixteenth_ang) = (
                half / 2. * TAU - TAU / 4.,
                quarter / 4. * TAU - TAU / 4.,
                eighth / 16. * TAU - TAU / 4.,
                sixteenth / 256. * TAU - TAU / 4.,
            );

            painter.circle_stroke(center, radius, (2., Color32::WHITE));

            painter.line_segment(
                [
                    center,
                    center + vec2(sixteenth_ang.cos(), sixteenth_ang.sin()) * radius * 0.9,
                ],
                (2., Color32::PURPLE),
            );
            painter.line_segment(
                [
                    center,
                    center + vec2(eighth_ang.cos(), eighth_ang.sin()) * radius * 0.7,
                ],
                (3., Color32::BLUE),
            );
            painter.line_segment(
                [
                    center,
                    center + vec2(quarter_ang.cos(), quarter_ang.sin()) * radius * 0.5,
                ],
                (4., Color32::GREEN),
            );
            painter.line_segment(
                [
                    center,
                    center + vec2(half_ang.cos(), half_ang.sin()) * radius * 0.3,
                ],
                (5., Color32::RED),
            );

            ui.ctx().request_repaint();
        });
    }
}

fn draw_ticks(painter: &Painter, center: Pos2, radius: f32) {
    for i in 0..256 {
        let ang = i as f32 / 256. * TAU - TAU / 4.;

        let outer = center + vec2(ang.cos(), ang.sin()) * radius;
        let inner = center
            + vec2(ang.cos(), ang.sin())
                * radius
                * if i % (256 / 2) == 0 {
                    0.71
                } else if i % (256 / 4) == 0 {
                    0.78
                } else if i % (256 / 16) == 0 {
                    0.85
                } else {
                    0.92
                };

        painter.add(Shape::line_segment(
            [inner, outer],
            Stroke::new(1., Color32::WHITE),
        ));
    }
}

fn draw_numbers(painter: &Painter, center: Pos2, radius: f32) {
    for i in 0..256 {
        let ang = i as f32 / 256. * TAU - TAU / 4.;

        let number = if i % (256 / 2) == 0 {
            i / (256 / 2)
        } else if i % (256 / 4) == 0 {
            i / (256 / 4)
        } else if i % (256 / 16) == 0 {
            i / (256 / 16)
        } else {
            i
        };
        let pos = center
            + vec2(ang.cos(), ang.sin())
                * radius
                * if i % (256 / 2) == 0 {
                    0.64
                } else if i % (256 / 4) == 0 {
                    0.71
                } else if i % (256 / 16) == 0 {
                    0.78
                } else {
                    0.85
                };
        let stroke = if i % (256 / 2) == 0 {
            0.14
        } else if i % (256 / 4) == 0 {
            0.07
        } else if i % (256 / 16) == 0 {
            0.035
        } else {
            0.0175
        };

        painter.text(
            pos,
            Align2::CENTER_CENTER,
            number.to_string(),
            FontId::monospace(radius * stroke),
            Color32::WHITE,
        );
    }
}

struct BinaiveTime {
    bsecs: u16,
    frac: u32,
}

impl BinaiveTime {
    /// Returns the hour number 0 or 1 with a boolean flag, which is false for AM and true for PM.
    fn half2(&self) -> (bool, u16) {
        #[allow(clippy::bad_bit_mask)]
        (self.bsecs & 0b1000_0000_0000_0000 == 1, self.half() % 2)
    }

    /// Returns the half number from 0 to 3.
    fn half(&self) -> u16 {
        (self.bsecs & 0b1100_0000_0000_0000) >> 14
    }

    /// Returns the quarter number from 0 to 3.
    fn quarter(&self) -> u16 {
        (self.bsecs & 0b0011_0000_0000_0000) >> 12
    }

    /// Returns the eighth number from `0` to `15`.
    fn eighth(&self) -> u16 {
        (self.bsecs & 0b0000_1111_0000_0000) >> 8
    }

    /// Returns the sixteenth number from `0` to `255`.
    fn sixteenth(&self) -> u16 {
        self.bsecs & 0b0000_0000_1111_1111
    }
}

impl From<NaiveTime> for BinaiveTime {
    fn from(nt: NaiveTime) -> Self {
        // bsecs = secs * 2^16 / 60 / 60 / 24
        let secs = 60 * (60 * nt.hour() + nt.minute()) + nt.second();
        let bsecs_u64 = (secs as u64) * (1 << 16) / (60 * 60 * 24);
        Self {
            bsecs: bsecs_u64 as u16,
            frac: nt.nanosecond(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_naivetime_for_binaivetime() {
        // 12:34:56.789012345
        let t = NaiveTime::from_hms_nano_opt(12, 34, 56, 789_012_345).unwrap();

        let secs = t.hour()  * 3600 + t.minute() * 60 + t.second();

        let expected_bsecs = (secs as u64 * 65536 / 86400) as u16;

        let expected_frac = t.nanosecond();

        let bt = BinaiveTime::from(t);

        assert_eq!(bt.bsecs, expected_bsecs);
        assert_eq!(bt.frac, expected_frac);
    }

    #[test]
    fn test_midnight() {
        // 00:00:00.000
        let t = NaiveTime::from_hms_nano_opt(0, 0, 0, 0).unwrap();
        let bt = BinaiveTime::from(t);

        assert_eq!(bt.bsecs, 0);
        assert_eq!(bt.frac, 0);
    }

    #[test]
    fn test_last_second() {
        // 23:59:59.999999999
        let t = NaiveTime::from_hms_nano_opt(23, 59, 59, 999_999_999).unwrap();

        let secs = t.hour() * 3600 + t.minute() * 60 + t.second();

        let expected_bsecs = (secs as u64 * 65536 / 86400) as u16;

        let expected_frac = t.nanosecond();

        let bt = BinaiveTime::from(t);

        assert_eq!(bt.bsecs, expected_bsecs);
        assert_eq!(bt.frac, expected_frac);
    }
}
