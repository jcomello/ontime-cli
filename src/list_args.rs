use clap::Args;
use chrono_tz::{Tz, TZ_VARIANTS};

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Timezones to be compared
    #[clap(name="filter")]
    #[arg(short, long)]
    pub filter: Option<String>,
}

impl ListArgs {
    pub fn available_timezones(&self) -> Vec<Tz> {
        let tz_variants = TZ_VARIANTS.to_vec();

        match &self.filter {
            Some(filter) => {
                tz_variants
                    .into_iter()
                    .filter(|tz_variant| {
                        let variant = tz_variant.to_string().to_lowercase();
                        let lowercase_filter = &filter.to_lowercase();

                        variant.contains(lowercase_filter)
                    })
                    .collect()
            },
            None => tz_variants
        }
    }
}
