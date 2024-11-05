#[cfg(test)]
mod list {
    use crate::Cli;
    use chrono_tz::TZ_VARIANTS;

    #[test]
    fn test_available_timezones() {
        let available_timezones = Cli::available_timezones();

        assert_eq!(available_timezones, TZ_VARIANTS);
    }
}

#[cfg(test)]
mod compare {
    use std::str::FromStr;
    use crate::Cli;
    use crate::compare_args::CompareArgs;
    use chrono::Utc;
    use chrono_tz::Tz;

    #[test]
    fn test_compare_timezones_without_local() {
        let now = Utc::now();
        let new_york_str = "America/New_York";
        let berlin_str = "Europe/Berlin";
        let args = CompareArgs {
            timezones: vec![new_york_str.to_string(), berlin_str.to_string()],
            local: false,
            from_now: 0,
            ago: 0,
        };
        let compare_timezones = Cli::compare_timezones(&args);
        let new_york_tz = Tz::from_str(new_york_str).unwrap();
        let berlin_tz = Tz::from_str(berlin_str).unwrap();
        let new_york_timezone = now.with_timezone(&new_york_tz).format("%Y-%m-%d %H:%M %Z %:z").to_string();
        let berlin_timezone = now.with_timezone(&berlin_tz).format("%Y-%m-%d %H:%M %Z %:z").to_string();

        let response = vec![
            ("America/New_York".to_string(), new_york_timezone),
            ("Europe/Berlin".to_string(), berlin_timezone),
        ];

        let formated_compare_timezones: Vec<(String, String)> = compare_timezones.into_iter().map(|el| {
            (el.0, el.1.format("%Y-%m-%d %H:%M %Z %:z").to_string())
        }).collect();

        assert_eq!(formated_compare_timezones, response);
    }

    #[test]
    fn test_compare_timezones_with_local() {
        let now = Utc::now();
        let local_str = iana_time_zone::get_timezone().unwrap();
        let new_york_str = "America/New_York";
        let berlin_str = "Europe/Berlin";
        let args = CompareArgs {
            timezones: vec![new_york_str.to_string(), berlin_str.to_string()],
            local: true,
            from_now: 0,
            ago: 0,
        };
        let compare_timezones = Cli::compare_timezones(&args);
        let formated_local = format!("{} (Local)", local_str);
        let local_tz = Tz::from_str(&local_str).unwrap();
        let new_york_tz = Tz::from_str(new_york_str).unwrap();
        let berlin_tz = Tz::from_str(berlin_str).unwrap();
        let local_timezone = now.with_timezone(&local_tz).format("%Y-%m-%d %H:%M %Z %:z").to_string();
        let new_york_timezone = now.with_timezone(&new_york_tz).format("%Y-%m-%d %H:%M %Z %:z").to_string();
        let berlin_timezone = now.with_timezone(&berlin_tz).format("%Y-%m-%d %H:%M %Z %:z").to_string();

        let response = vec![
            (formated_local, local_timezone),
            ("America/New_York".to_string(), new_york_timezone),
            ("Europe/Berlin".to_string(), berlin_timezone),
        ];

        let formated_compare_timezones: Vec<(String, String)> = compare_timezones.into_iter().map(|el| {
            (el.0, el.1.format("%Y-%m-%d %H:%M %Z %:z").to_string())
        }).collect();

        assert_eq!(formated_compare_timezones, response);
    }
}
