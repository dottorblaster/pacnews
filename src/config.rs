use crate::Args;
use crate::Sort;

pub fn get_config_options(config: Args) -> (Sort, bool, bool) {
    (config.sort, config.lookup, config.colors)
}
