use std::fmt::Debug;
use std::time::Duration;

/// Trait to get the name of an enum variant
pub trait NameFromEnum {
    fn name(&self) -> String;
}

impl<T: Debug> NameFromEnum for T {
    fn name(&self) -> String {
        format!("{:?}", self)
    }
}

/// Scale a Duration by a factor
pub fn scale_duration(duration: Duration, scale: f32) -> Duration {
    let sec = (duration.as_secs() as f32 + duration.subsec_nanos() as f32 * 1e-9) * scale;
    Duration::new(sec.trunc() as u64, (sec.fract() * 1e9) as u32)
}
