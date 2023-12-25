use tide_rs::TIDE;
use tide_rs::ConfigValue;

fn main() {
    let config_file_path = "./examples/example.tide";

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

    // Example of accessing an integer value.
    match tide_config.get_config_value("database.port") {
        Ok(ConfigValue::Integer(value)) => println!("Database port: {}", value),
        _ => println!("Database port not found or not an integer."),
    }

    // Accessing a string array value.
    match tide_config.get_config_value("myApp.features") {
        Ok(ConfigValue::StringArray(value)) => println!("Features: {:?}", value),
        _ => println!("Features not found or not a string array."),
    }

    // Accessing an integer array value.
    match tide_config.get_config_value("myApp.numbers") {
        Ok(ConfigValue::IntegerArray(value)) => println!("Numbers: {:?}", value),
        _ => println!("Numbers not found or not an integer array."),
    }
}
