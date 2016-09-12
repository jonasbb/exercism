#![feature(associated_consts)]

/// Duration in seconds
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    /// Specifies the duration of a planets year
    const YEAR_DURATION: Duration;

    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / Self::YEAR_DURATION.0 as f64
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

// 31557600 is the number of seconds in one earth year
// all fractions are taken from the readme
impl Planet for Mercury {
    const YEAR_DURATION: Duration = Duration((31557600.0 * 0.2408467) as u64);
}
impl Planet for Venus {
    const YEAR_DURATION: Duration = Duration((31557600.0 * 0.61519726) as u64);
}
impl Planet for Earth {
    const YEAR_DURATION: Duration = Duration((31557600.0 * 1.0) as u64);
}
impl Planet for Mars {
    const YEAR_DURATION: Duration = Duration((31557600.0 * 1.8808158) as u64);
}
impl Planet for Jupiter {
    const YEAR_DURATION: Duration = Duration((31557600.0 * 11.862615) as u64);
}
impl Planet for Saturn {
    const YEAR_DURATION: Duration = Duration((31557600.0 * 29.447498) as u64);
}
impl Planet for Uranus {
    const YEAR_DURATION: Duration = Duration((31557600.0 * 84.016846) as u64);
}
impl Planet for Neptune {
    const YEAR_DURATION: Duration = Duration((31557600.0 * 164.79132) as u64);
}
