# squidge

✨ Overview
---

`squidge` shortens delimited data.

```rust
use squidge::{Config, shorten_line};

let line = "module/submodule/service/lib.rs";
let result = shorten_line(&Config::default(), &line);
let expected = vec!["m", "s", "s", "lib.rs"];
assert_eq!(result, expected);
```

`squidge`'s functionality is available as a binary via [sqdj][1].

🛠️ Configuration
---

`squidge` can be configured to shorten lines in varying ways, based on its
`Config`.

```rust
use squidge::Config;
use regex::Regex;

let re = Regex::new("module").unwrap();
let cfg = Config {
    // Delimiter to split the line on
    delimiter: "\\",
    // Number of elements to ignore (for shortening) from the start
    ignore_first_n: 2,
    // Number of elements to ignore (for shortening) from the end
    ignore_last_n: 2,
    // Optional regex to determine which components to ignore while shortening
    ignore_regex: Some(re),
};
```

[1]: https://crates.io/crates/sqdj
