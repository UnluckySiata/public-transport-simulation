use std::time::Instant;

fn simulation_loop() {
    const FIXED_DT: f32 = 1.0 / 2.0;
    const MAX_FRAME_TIME: f32 = 0.25;

    let mut accumulator: f32 = 0.0;
    let mut previous = Instant::now();

    loop {
        let now = Instant::now();
        let mut frame_time = (now - previous).as_secs_f32();
        previous = now;

        if frame_time > MAX_FRAME_TIME {
            frame_time = MAX_FRAME_TIME;
        }
        accumulator += frame_time;

        while accumulator >= FIXED_DT {
            accumulator -= FIXED_DT;
            println!("ft: {frame_time}");
        }
    }
}

fn main() {
    simulation_loop();
}
