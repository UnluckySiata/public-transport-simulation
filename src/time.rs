
pub struct Time {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    accumulator: f64,
}

impl Time {
    pub fn new(hour: u32, minute: u32, second: u32) -> Self {
        Self {
            hour,
            minute,
            second,
            accumulator: 0.0
        }
    }
    pub fn increase(&mut self, time_step: f64) -> bool {
        self.accumulator += time_step;

        if self.accumulator >= 1.0 {
            self.second += 1;
        }

        if self.second == 60 {
            self.second = 0;
            self.minute += 1;
        }

        if self.minute == 60 {
            self.minute = 0;
            self.hour += 1;
        }

        if self.hour == 24 {
            return true;

        }

        false
    }
}
