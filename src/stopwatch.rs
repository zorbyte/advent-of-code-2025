use std::{
    fmt,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Stopwatch(Instant);

impl Stopwatch {
    pub fn start() -> Self {
        Self(Instant::now())
    }

    pub fn stop(self) -> FinishedStopwatch {
        FinishedStopwatch(self.0.elapsed())
    }
}

#[derive(Debug)]
pub struct FinishedStopwatch(Duration);

impl fmt::Display for FinishedStopwatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut written = false;

        let secs = self.0.as_secs();
        if secs > 0 {
            write!(f, "{secs}s")?;
            written = true;
        }

        let ms = self.0.subsec_millis();
        if ms > 0 {
            if written {
                f.write_str(" ")?;
            }

            write!(f, "{ms}ms")?;
            written = true;
        }

        let micros = self.0.subsec_micros() - (ms * 1000);
        if micros > 0 {
            if written {
                f.write_str(" ")?;
            }

            write!(f, "{micros}µs")?;
        }

        Ok(())
    }
}
