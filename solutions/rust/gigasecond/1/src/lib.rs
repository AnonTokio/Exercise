use time::{
    Date, Duration, OffsetDateTime, PrimitiveDateTime, UtcOffset,
};

const GIGA_SECOND: i64 = 1_000_000_000;
const DELTA: Duration = Duration::seconds(GIGA_SECOND);

pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    start + DELTA
}
