# DREW

*D*ashboard for *R*eleases, *E*xceptions, and *W*orkitems.

Namesake for our late colleague, Andrew Babb.

## Installation

```bash
#Install Diesel CLI with SQLite Feature
cargo install diesel_cli --no-default-features --features sqlite
```

```bash
#Install nightly, which is a Rocket Framework dependency
rustup toolchain install nightly
```

```bash
#Set nightly to default
rustup default nightly
```

```bash
#Install cargo watch, which allows changes in your code to automatically restart running server on code change
cargo install cargo-watch
```

```bash
#Install Rust Code Formatter
rustup component add rustfmt
```


### Common Development Tasks

```bash
#Run server using watch
cargo watch -x run
```


#### Run Code Formatter

```bash
# From project root
cargo fmt
```

