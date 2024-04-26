use sdl2::TimerSubsystem;

pub struct Time {
    _total_time: f64,
    _delta_time: f64,
    _last_time: f64,
}

impl Time {
    pub fn new() -> Self {
        Self {
            _total_time: 0.0,
            _delta_time: 0.0,
            _last_time: 0.0,
        }
    }

    pub fn delta_time(&self) -> f64 {
        self._delta_time
    }

    pub fn update(&mut self, timer: &TimerSubsystem) {
        let current_time = timer.performance_counter() as f64 / timer.performance_frequency() as f64;
        self._total_time = current_time;
        self._delta_time = current_time - self._last_time;
        self._last_time = current_time;
    }
}

pub struct A;