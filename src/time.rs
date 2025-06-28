
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

        while self.accumulator >= 1.0 {
            self.accumulator -= 1.0;
            self.second += 1;
        }

        while self.second >= 60 {
            self.second -= 60;
            self.minute += 1;
        }

        while self.minute >= 60 {
            self.minute -= 60;
            self.hour += 1;
        }

        if self.hour == 24 {
            return true;

        }

        false
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minute, self.second)
    }
}
