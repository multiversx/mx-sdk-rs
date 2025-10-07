#![cfg(test)]
use super::*;

mod timestamp_millis_tests {
    use super::*;

    #[test]
    fn test_arithmetic_operations() {
        let t1 = TimestampMillis::new(1000);
        let t2 = TimestampMillis::new(500);
        let d = DurationMillis::new(300);

        // Addition: timestamp + duration = timestamp
        assert_eq!(t1 + d, TimestampMillis::new(1300));

        // Subtraction: timestamp - duration = timestamp
        assert_eq!(t1 - d, TimestampMillis::new(700));

        // Subtraction: timestamp - timestamp = duration
        assert_eq!(t1 - t2, DurationMillis::new(500));
    }

    #[test]
    fn test_getters_and_conversions() {
        let t = TimestampMillis::new(1500);

        // Value getter
        assert_eq!(t.as_u64_millis(), 1500);

        // Conversion to seconds (truncating)
        assert_eq!(t.to_seconds(), TimestampSeconds::new(1));
    }
}

mod timestamp_seconds_tests {
    use super::*;

    #[test]
    fn test_arithmetic_operations() {
        let t1 = TimestampSeconds::new(1000);
        let t2 = TimestampSeconds::new(500);
        let d = DurationSeconds::new(300);

        // Addition: timestamp + duration = timestamp
        assert_eq!(t1 + d, TimestampSeconds::new(1300));

        // Subtraction: timestamp - timestamp = duration
        assert_eq!(t1 - t2, DurationSeconds::new(500));
    }

    #[test]
    fn test_getters_and_conversions() {
        let t = TimestampSeconds::new(1);

        // Value getter
        assert_eq!(t.as_u64_seconds(), 1);

        // Conversion to milliseconds
        let ms = t.to_millis();
        assert_eq!(ms.as_u64_millis(), 1000);
    }
}

mod duration_millis_tests {
    use super::*;

    #[test]
    fn test_arithmetic_operations() {
        let d1 = DurationMillis::new(1000);
        let d2 = DurationMillis::new(300);

        // Addition: duration + duration = duration
        assert_eq!(d1 + d2, DurationMillis::new(1300));

        // Subtraction: duration - duration = duration
        assert_eq!(d1 - d2, DurationMillis::new(700));
    }

    #[test]
    fn test_getters_and_conversions() {
        let d = DurationMillis::new(2500);

        // Raw value access
        assert_eq!(d.as_u64_millis(), 2500);

        // Convert to seconds (truncating)
        assert_eq!(d.to_seconds().as_u64_seconds(), 2);
    }
}

mod duration_seconds_tests {
    use super::*;

    #[test]
    fn test_arithmetic_operations() {
        let d1 = DurationSeconds::new(1000);
        let d2 = DurationSeconds::new(300);

        // Addition: duration + duration = duration
        assert_eq!(d1 + d2, DurationSeconds::new(1300));

        // Subtraction: duration - duration = duration
        assert_eq!(d1 - d2, DurationSeconds::new(700));
    }

    #[test]
    fn test_getters_and_conversions() {
        let d = DurationSeconds::new(2);

        // Raw value access
        assert_eq!(d.as_u64_seconds(), 2);

        // Convert to milliseconds and check value
        assert_eq!(d.to_millis().as_u64_millis(), 2000);
    }
}

mod unit_conversion_tests {
    use super::*;

    #[test]
    fn test_timestamp_conversions() {
        // Test milliseconds to seconds conversion (truncating)
        let ts_millis = TimestampMillis::new(5000);
        assert_eq!(ts_millis.to_seconds(), TimestampSeconds::new(5));

        // Test seconds to milliseconds conversion
        let ts_seconds = TimestampSeconds::new(5);
        assert_eq!(ts_seconds.to_millis().as_u64_millis(), 5000);
    }

    #[test]
    fn test_duration_conversions() {
        // Test milliseconds to seconds conversion (truncating)
        let dur_millis = DurationMillis::new(3000);
        assert_eq!(dur_millis.to_seconds().as_u64_seconds(), 3);

        // Test seconds to milliseconds conversion
        let dur_seconds = DurationSeconds::new(3);
        assert_eq!(dur_seconds.to_millis().as_u64_millis(), 3000);
    }
}
