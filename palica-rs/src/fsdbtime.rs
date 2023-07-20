/*
    palica media catalogue program
    Copyright (C) 2023 Yury Benesh

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
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
        let sec: u64 = 1_000_000_000;
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
