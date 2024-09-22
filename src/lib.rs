use regex::Regex;

#[derive(Debug)]
pub struct Config<'a> {
    pub delimiter: &'a str,
    pub ignore_first_n: usize,
    pub ignore_last_n: usize,
    pub ignore_regex: Option<Regex>,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Config {
            delimiter: "/",
            ignore_first_n: 0,
            ignore_last_n: 1,
            ignore_regex: None,
        }
    }
}

pub fn shorten_line(cfg: &Config, line: &str) -> Vec<String> {
    let num_elements = line.matches(cfg.delimiter).count();
    let line_iter = line.split(cfg.delimiter);
    let mut shortened_components: Vec<String> = Vec::new();

    for (i, component) in line_iter.enumerate() {
        if i < cfg.ignore_first_n
            || (cfg.ignore_last_n > num_elements || i > num_elements - cfg.ignore_last_n)
        {
            shortened_components.push(component.to_string());
            continue;
        }

        let shorten_component = match cfg.ignore_regex {
            Some(ref r) => match r.is_match(component) {
                true => false,
                false => true,
            },
            None => true,
        };

        let shortened_component = match shorten_component {
            true => match component.chars().next() {
                Some(c) => c.to_string(),
                None => String::new(),
            },
            false => component.to_string(),
        };

        shortened_components.push(shortened_component);
    }
    shortened_components
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squidge_line_works_with_default_config() {
        let line = String::from("module/submodule/service/lib.rs");
        let result = shorten_line(&Config::default(), &line);
        let expected = vec!["m", "s", "s", "lib.rs"];
        assert_eq!(result, expected);
    }

    #[test]
    fn squidge_line_works_with_a_starting_delimiter() {
        let line = String::from("/module/submodule/service/lib.rs");
        let result = shorten_line(&Config::default(), &line);
        let expected = vec!["", "m", "s", "s", "lib.rs"];
        assert_eq!(result, expected);
    }

    #[test]
    fn squidge_line_respects_delimiter() {
        let line = String::from("module,submodule,service,lib.rs");
        let cfg = Config {
            delimiter: ",",
            ..Config::default()
        };
        let result = shorten_line(&cfg, &line);
        let expected = vec!["m", "s", "s", "lib.rs"];
        assert_eq!(result, expected);
    }

    #[test]
    fn squidge_line_ignores_components_matching_regex() {
        let line = String::from("module/submodule/service/lib.rs");
        let re = Regex::new("module").unwrap();
        let cfg = Config {
            ignore_regex: Some(re),
            ..Config::default()
        };
        let result = shorten_line(&cfg, &line);
        let expected = vec!["module", "submodule", "s", "lib.rs"];
        assert_eq!(result, expected);
    }

    #[test]
    fn squidge_line_ignores_last_n_components() {
        let line = String::from("module/submodule/service/lib.rs");
        let cfg = Config {
            ignore_last_n: 3,
            ..Config::default()
        };
        let result = shorten_line(&cfg, &line);
        let expected = vec!["m", "submodule", "service", "lib.rs"];
        assert_eq!(result, expected);
    }

    #[test]
    fn squidge_line_works_when_ignore_last_n_is_greater_than_num_components() {
        let line = String::from("module/submodule/service/lib.rs");
        let cfg = Config {
            ignore_last_n: 6,
            ..Config::default()
        };
        let result = shorten_line(&cfg, &line);
        let expected = vec!["module", "submodule", "service", "lib.rs"];
        assert_eq!(result, expected);
    }

    #[test]
    fn squidge_line_ignores_first_n_components() {
        let line = String::from("module/submodule/service/lib.rs");
        let cfg = Config {
            ignore_first_n: 1,
            ..Config::default()
        };
        let result = shorten_line(&cfg, &line);
        let expected = vec!["module", "s", "s", "lib.rs"];
        assert_eq!(result, expected);
    }

    #[test]
    fn squidge_line_works_when_ignore_first_n_is_greater_than_num_components() {
        let line = String::from("module/submodule/service/lib.rs");
        let cfg = Config {
            ignore_first_n: 6,
            ..Config::default()
        };
        let result = shorten_line(&cfg, &line);
        let expected = vec!["module", "submodule", "service", "lib.rs"];
        assert_eq!(result, expected);
    }

    #[test]
    fn squidge_line_ignores_first_n_and_last_m_components() {
        let line = String::from("module/submodule/service/lib.rs");
        let cfg = Config {
            ignore_first_n: 1,
            ignore_last_n: 2,
            ..Config::default()
        };
        let result = shorten_line(&cfg, &line);
        let expected = vec!["module", "s", "service", "lib.rs"];
        assert_eq!(result, expected);
    }

    #[test]
    fn squidge_line_works_when_a_component_is_empty() {
        let line = String::from("module//service/lib.rs");
        let result = shorten_line(&Config::default(), &line);
        let expected = vec!["m", "", "s", "lib.rs"];
        assert_eq!(result, expected);
    }

    #[test]
    fn squidge_line_works_with_non_default_config() {
        let line = String::from("/path/to/a/module/submodule/service/lib.rs");
        let re = Regex::new("module").unwrap();
        let cfg = Config {
            ignore_first_n: 2,
            ignore_last_n: 2,
            ignore_regex: Some(re),
            ..Config::default()
        };
        let result = shorten_line(&cfg, &line);
        let expected = vec![
            "",
            "path",
            "t",
            "a",
            "module",
            "submodule",
            "service",
            "lib.rs",
        ];
        assert_eq!(result, expected);
    }
}
