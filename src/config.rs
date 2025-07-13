use crate::{Args, Sort};

pub fn get_config_options(config: Args) -> (Sort, bool, bool, Option<u16>) {
    (config.sort, config.lookup, config.colors, config.count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getting_options_works() {
        let config = Args {
            sort: Sort::Asc,
            lookup: true,
            colors: true,
            count: Some(16),
        };

        let (sorting, is_lookup_enabled, colors, count) = get_config_options(config);

        assert_eq!(sorting, Sort::Asc);
        assert_eq!(is_lookup_enabled, true);
        assert_eq!(colors, true);
        assert_eq!(count, Some(16));
    }
}
