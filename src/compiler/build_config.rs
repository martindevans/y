pub struct BuildConfig {
    configs: Vec<String>,
    line_length: u16,
    line_count: u16,
}

impl BuildConfig {
    pub fn from_matches(matches: &clap::ArgMatches<'_>) -> BuildConfig {
        BuildConfig {
            configs: matches.values_of("config")
                .map(|a| a.map(|s| s.to_string()).collect())
                .unwrap_or(Vec::new()),

            line_length: matches.value_of("line_length").map(|s| s.parse().expect("Cannot parse u16 from line_length")).unwrap_or(70),
            line_count: matches.value_of("line_count").map(|s| s.parse().expect("Cannot parse u16 from line_count")).unwrap_or(20),
        }
    }
}