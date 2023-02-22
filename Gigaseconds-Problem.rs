extern crate time;

use time::{Duration, PrimitiveDateTime};

const giga_seconds: i64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    start + Duration::seconds(giga_seconds)
}
