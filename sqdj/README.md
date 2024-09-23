# sqdj

✨ Overview
---

`sqdj` shortens delimited data.

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

`sqdj` uses [squidge][1] behind the scenes.

### 💾 Installation

**cargo**:

```sh
cargo install sqdj
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

[1]: https://crates.io/crates/squidge
