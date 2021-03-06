use command::circuit_breaker_stats::CircuitBreakerStats;
use command::window::Window;
use command::window::Point;
use command::Config;
use std::time::{Instant, Duration};

#[derive(Clone, Debug)]
pub struct CircuitBreaker {
    circuit_breaker_stats: CircuitBreakerStats,
    circuit_open_time: Option<Instant>,
    config: Config
}

impl CircuitBreaker {

    pub fn new(config: Config) -> CircuitBreaker {
        let window = Window::new(config);
        return CircuitBreaker {
            circuit_breaker_stats: CircuitBreakerStats {
                window: window
            },
            circuit_open_time: None,
            config: config
        }
    }

    pub fn check_command_allowed(&mut self) -> bool {
        if self.should_close_open_circuit() {
            self.circuit_open_time = None;
            return true;
        } else if self.should_keep_circuit_open() {
            return false;
        } else if self.should_open_circuit() {
            self.circuit_open_time = Some(Instant::now());
            self.circuit_breaker_stats.clear();
            return false
        } else {
            return true;
        }
    }

    pub fn register_result<T, E>(&mut self, res: &Result<T, E>) {
        match *res {
            Ok(_) => self.circuit_breaker_stats.add_point(Point::SUCCESS),
            Err(_) => self.circuit_breaker_stats.add_point(Point::FAILURE)
        }
    }

    fn should_close_open_circuit(&mut self) -> bool {
        return self.circuit_open_time.is_some() && self.circuit_open_time.unwrap() <= self.time_to_close_circuit()
    }

    fn should_keep_circuit_open(&mut self) -> bool {
        return self.circuit_open_time.is_some() && self.circuit_open_time.unwrap() > self.time_to_close_circuit()
    }

    fn should_open_circuit(&mut self) -> bool {
        return self.circuit_breaker_stats.error_percentage() >= self.config.error_threshold_percentage.unwrap() &&
            self.circuit_breaker_stats.error_nr() >= self.config.error_threshold.unwrap()
    }

    fn time_to_close_circuit(&self) -> Instant {
        return Instant::now() - Duration::from_millis(self.config.circuit_open_ms.unwrap());
    }
}