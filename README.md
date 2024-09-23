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

🔧 squidge as a binary: sqdj
---

### 💾 Installation

**cargo**:

```sh
cargo install --git https://github.com/dhth/squidge.git --bin sqdj
```

### ⚡️ Usage

```text
$ sqdj -h
sqdj shortens delimited data

Usage: sqdj [OPTIONS]

Options:
  -d, --delimiter <STRING>         Delimiter [default: /]
  -r, --ignore-regex <STRING>      Regex for ignoring elements (ie, they won't be shortened)
  -f, --ignore-first-n <NUMBER>    Ignore first n elements [default: 0]
  -l, --ignore-last-n <NUMBER>     Ignore last n elements [default: 1]
      --output-delimiter <STRING>  Output delimiter [default: /]
  -h, --help                       Print help
```

```bash
cat << EOF | sqdj
src/main/scala/admin/billing/ApplicationComponents.scala
src/main/scala/admin/billing/Components.scala
src/main/scala/admin/billing/Server.scala
EOF

# s/m/s/a/b/ApplicationComponents.scala
# s/m/s/a/b/Components.scala
# s/m/s/a/b/Server.scala
```

```bash
cat << EOF | sqdj --ignore-first-n 1
src/main/scala/admin/billing/ApplicationComponents.scala
src/main/scala/admin/billing/Components.scala
src/main/scala/admin/billing/Server.scala
EOF

# src/m/s/a/b/ApplicationComponents.scala
# src/m/s/a/b/Components.scala
# src/m/s/a/b/Server.scala
```

```bash
cat << EOF | sqdj --ignore-last-n 2
src/main/scala/admin/billing/ApplicationComponents.scala
src/main/scala/admin/billing/Components.scala
src/main/scala/admin/billing/Server.scala
EOF

# s/m/s/a/billing/ApplicationComponents.scala
# s/m/s/a/billing/Components.scala
# s/m/s/a/billing/Server.scala
```

```bash
cat << EOF | sqdj --ignore-regex 'billing|utils'
src/main/scala/admin/billing/api/PlayTapir.scala
src/main/scala/admin/billing/api/billing/BillingApiModule.scala
src/main/scala/admin/billing/api/utils/Authenticator.scala
EOF

# s/m/s/a/billing/a/PlayTapir.scala
# s/m/s/a/billing/a/billing/BillingApiModule.scala
# s/m/s/a/billing/a/utils/Authenticator.scala
```
