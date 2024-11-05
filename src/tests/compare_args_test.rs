#[cfg(test)]
mod time {
    use crate::compare_args::CompareArgs;
    use chrono::{Utc, Duration};

    #[test]
    fn test_compare_args_time() {
        let now = Utc::now();
        let formated_now = now.format("%Y-%m-%d %H:%M %Z %:z").to_string();
        let args = CompareArgs {
            timezones: vec![],
            local: false,
            from_now: 0,
            ago: 0,
        };
        let formated_args_time = args.time().format("%Y-%m-%d %H:%M %Z %:z").to_string();

        assert_eq!(formated_args_time, formated_now);
    }

    #[test]
    fn test_compare_args_time_three_hours_from_now() {
        let now = Utc::now() + Duration::hours(3);
        let formated_now = now.format("%Y-%m-%d %H:%M %Z %:z").to_string();
        let args = CompareArgs {
            timezones: vec![],
            local: false,
            from_now: 3,
            ago: 0,
        };
        let formated_args_time = args.time().format("%Y-%m-%d %H:%M %Z %:z").to_string();

        assert_eq!(formated_args_time, formated_now);
    }

    #[test]
    fn test_compare_args_time_three_hours_ago_and_1_hour_from_now() {
        let now = Utc::now() - Duration::hours(2);
        let formated_now = now.format("%Y-%m-%d %H:%M %Z %:z").to_string();
        let args = CompareArgs {
            timezones: vec![],
            local: false,
            from_now: 1,
            ago: 3,
        };
        let formated_args_time = args.time().format("%Y-%m-%d %H:%M %Z %:z").to_string();

        assert_eq!(formated_args_time, formated_now);
    }

    #[test]
    fn test_compare_args_time_three_hours_ago() {
        let now = Utc::now() - Duration::hours(3);
        let formated_now = now.format("%Y-%m-%d %H:%M %Z %:z").to_string();
        let args = CompareArgs {
            timezones: vec![],
            local: false,
            from_now: 0,
            ago: 3,
        };
        let formated_args_time = args.time().format("%Y-%m-%d %H:%M %Z %:z").to_string();

        assert_eq!(formated_args_time, formated_now);
    }
}

#[cfg(test)]
mod timezones {
    use crate::compare_args::CompareArgs;
    use std::str::FromStr;
    use chrono::Utc;
    use chrono_tz::Tz;

    #[test]
    fn test_compare_args_timezones() {
        let now = Utc::now();
        let new_york_str = "America/New_York";
        let berlin_str = "Europe/Berlin";
        let new_york_tz = Tz::from_str(new_york_str).unwrap();
        let berlin_tz = Tz::from_str(berlin_str).unwrap();
        let new_york_timezone = now.with_timezone(&new_york_tz).format("%Y-%m-%d %H:%M %Z %:z").to_string();
        let berlin_timezone = now.with_timezone(&berlin_tz).format("%Y-%m-%d %H:%M %Z %:z").to_string();

        let expected_timezones = vec![
            ("America/New_York".to_string(), new_york_timezone),
            ("Europe/Berlin".to_string(), berlin_timezone),
        ];
        let args = CompareArgs {
            timezones: vec![new_york_str.to_string(), berlin_str.to_string()],
            local: false,
            from_now: 0,
            ago: 0,
        };
        let timezones_response = args.timezones_list();
        let formated_timezones_response: Vec<(String, String)> =  timezones_response.into_iter().map(|el| {
            (el.0, el.1.format("%Y-%m-%d %H:%M %Z %:z").to_string())
        }).collect();

        assert_eq!(formated_timezones_response, expected_timezones);
    }

    #[test]
    fn test_compare_args_timezones_with_local() {
        let now = Utc::now();
        let local_str = iana_time_zone::get_timezone().unwrap();
        let formated_local = format!("{} (Local)", local_str);
        let new_york_str = "America/New_York";
        let berlin_str = "Europe/Berlin";
        let local_tz = Tz::from_str(&local_str).unwrap();
        let new_york_tz = Tz::from_str(new_york_str).unwrap();
        let berlin_tz = Tz::from_str(berlin_str).unwrap();
        let local_timezone = now.with_timezone(&local_tz).format("%Y-%m-%d %H:%M %Z %:z").to_string();
        let new_york_timezone = now.with_timezone(&new_york_tz).format("%Y-%m-%d %H:%M %Z %:z").to_string();
        let berlin_timezone = now.with_timezone(&berlin_tz).format("%Y-%m-%d %H:%M %Z %:z").to_string();

        let expected_timezones = vec![
            (formated_local, local_timezone),
            ("America/New_York".to_string(), new_york_timezone),
            ("Europe/Berlin".to_string(), berlin_timezone),
        ];
        let args = CompareArgs {
            timezones: vec![new_york_str.to_string(), berlin_str.to_string()],
            local: true,
            from_now: 0,
            ago: 0,
        };
        let timezones_response = args.timezones_list();
        let formated_timezones_response: Vec<(String, String)> =  timezones_response.into_iter().map(|el| {
            (el.0, el.1.format("%Y-%m-%d %H:%M %Z %:z").to_string())
        }).collect();

        assert_eq!(formated_timezones_response, expected_timezones);
    }
}
