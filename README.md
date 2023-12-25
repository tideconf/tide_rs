# tide_rs

## Introduction

tide-rs is a flexible config parser for Rust, designed to handle the TIDE configuration 
format. It provides an easy-to-use API for accessing configuration values from
TIDE files.

See [TIDE](https://github.com/tideconf/tide) for more information on the TIDE
configuration format.

> [!IMPORTANT]  
> This is no more than a hobby project at the moment. I have always been curious about the design and implementation of configuration frameworks, and this is my attempt at creating one. I am not sure if this will ever be used, but I am hoping to learn a bit more about the whole deal of configuration handling. If you are interested in this project, please feel free to contribute or provide feedback.

## Usage

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
tide_rs = "0.1.0"
```

## Example

```rust
use tide_rs::TIDE;
use tide_rs::ConfigValue;

fn main() {
    let config_file_path = "./path/tp/config.tide";

    let tide_config = match TIDE::new(config_file_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {:?}", e);
            return;
        }
    };

    // Use get_config_value method to access configuration values
    // Example of accessing a string value.
    match tide_config.get_config_value("database.type") {
        Ok(ConfigValue::String(value)) => println!("Database type: {}", value),
        _ => println!("Database type not found or not a string."),
    }
}
```

## Environment variables

TIDE configuration values can be overridden by environment variables. The
environment variable name is the uppercased path to the configuration value,
with the path separator replaced by an underscore.

For example `database.credentials.username` would be overridden by the
`DATABASE_CREDENTIALS_USERNAME` environment variable.

# Example

An example of using tide_rs to parse a TIDE file, is available in the examples
directory.

```bash
cargo run --example main
```

# Contributing

Contributions are welcome. Please feel free to open an issue or submit a pull
request.