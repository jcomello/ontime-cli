#[cfg(test)]
mod available_timezones {
    use crate::list_args::ListArgs;
    use std::str::FromStr;
    use chrono_tz::{Tz, TZ_VARIANTS};

    #[test]
    fn test_available_timezones() {
        let args = ListArgs {
            filter: None,
        };
        let available_timezones = args.available_timezones();

        assert_eq!(available_timezones, TZ_VARIANTS.to_vec());
    }

    #[test]
    fn test_available_timezones_with_filter() {
        let args = ListArgs {
            filter: Some("Brazil".to_string()),
        };
        let available_timezones = args.available_timezones();
        let comparing_timezones: Vec<Tz> = vec![
            "Brazil/Acre",
            "Brazil/DeNoronha",
            "Brazil/East",
            "Brazil/West",
        ]
        .into_iter()
        .map(|tz_str| Tz::from_str(tz_str).unwrap())
        .collect();

        assert_eq!(available_timezones, comparing_timezones);
    }

    #[test]
    fn test_available_timezones_with_lowercase_filter() {
        let args = ListArgs {
            filter: Some("brazil".to_string()),
        };
        let available_timezones = args.available_timezones();
        let comparing_timezones: Vec<Tz> = vec![
            "Brazil/Acre",
            "Brazil/DeNoronha",
            "Brazil/East",
            "Brazil/West",
        ]
        .into_iter()
        .map(|tz_str| Tz::from_str(tz_str).unwrap())
        .collect();

        assert_eq!(available_timezones, comparing_timezones);
    }
}
