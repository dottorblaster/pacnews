use crate::{Args, Sort};

pub fn get_config_options(config: Args) -> (Sort, bool, bool) {
    (config.sort, config.lookup, config.colors)
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
        };

        let (sorting, is_lookup_enabled, colors) = get_config_options(config);

        assert_eq!(sorting, Sort::Asc);
        assert_eq!(is_lookup_enabled, true);
        assert_eq!(colors, true);
    }
}
