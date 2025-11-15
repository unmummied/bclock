use chrono::NaiveTime;
use eframe::{NativeOptions, run_native};

mod binary_clock;
mod normal_clock;
use binary_clock::BClockApp;
use normal_clock::ClockApp;

fn main() -> eframe::Result {
    let t0 = NaiveTime::from_hms_opt(11, 59, 45).unwrap();
    let speed = 1000.;
    let normal = ClockApp::from(t0, speed);
    let binary = BClockApp::from(t0, speed);

    // run_native(
    //     "Normal Analog Clock",
    //     NativeOptions {
    //         vsync: false,
    //         ..Default::default()
    //     },
    //     Box::new(|_| Ok(Box::new(normal))),
    // )
    run_native(
        "Binary Analog Clock",
        NativeOptions {
            vsync: false,
            ..Default::default()
        },
        Box::new(|_| Ok(Box::new(binary))),
    )
}
