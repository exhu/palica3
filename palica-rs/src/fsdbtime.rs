use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn systime_from_db(dbtime: i64) -> SystemTime {
    UNIX_EPOCH + Duration::from_nanos(dbtime.try_into().expect("failed to convert nanos"))
}

pub fn dbtime_from_sys(systime: SystemTime) -> i64 {
    (systime.duration_since(UNIX_EPOCH))
        .expect("failed to get duration")
        .as_nanos()
        .try_into()
        .expect("failed to get nanos")
}

#[cfg(test)]
mod tests {
    use crate::fsdbtime::*;
    use std::time::UNIX_EPOCH;
    #[test]
    fn test_time() {
        assert_eq!(systime_from_db(0), UNIX_EPOCH);
        let sec:u64 = 1_000_000_000;
        let minute = sec * 60;
        let hour = minute * 60;
        let day = 24 * hour;
        assert_eq!(
            systime_from_db((day * 31 + sec).try_into().unwrap())
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            u128::from(day * 31 + sec)
        );
    }
}
