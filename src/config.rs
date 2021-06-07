use clap::ArgMatches;

pub fn get_config_options(config: ArgMatches) -> (String, bool) {
    (
        config.value_of("sort").unwrap().to_string(),
        config.is_present("lookup"),
    )
}
