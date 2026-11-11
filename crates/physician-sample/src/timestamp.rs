use std::time::{SystemTime, UNIX_EPOCH};

/// UTC RFC 3339 with milliseconds, e.g. `1970-01-01T00:00:00.000Z`.
pub(crate) fn format_rfc3339(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let days = secs / 86_400;
    let tod = secs % 86_400;
    let (year, month, day) = civil_from_unix_days(days);
    let hour = tod / 3_600;
    let minute = (tod % 3_600) / 60;
    let second = tod % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{millis:03}Z")
}

fn civil_from_unix_days(days: u64) -> (i32, u32, u32) {
    let z = days as i64 + 719_468;
    let era = z / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    (year as i32, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    #[test]
    fn rfc3339_formats_the_unix_epoch() {
        assert_eq!(format_rfc3339(UNIX_EPOCH), "1970-01-01T00:00:00.000Z");
        assert_eq!(
            format_rfc3339(UNIX_EPOCH + Duration::from_secs(86_400)),
            "1970-01-02T00:00:00.000Z"
        );
    }
}
