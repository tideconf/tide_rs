use tide_rs::TIDE;
use tide_rs::ConfigValue;

fn main() {
    // Replace 'config_file_path' with the actual path to your configuration file.
    let config_file_path = "./examples/example.tide";

    // Load the configuration.
    let tide_config = match TIDE::new(config_file_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {:?}", e);
            return;
        }
    };

    // Example of accessing a string value.
    if let Some(ConfigValue::String(value)) = tide_config.data.get("database.type") {
        println!("Database type: {}", value);
    } else {
        println!("Database type not found or not a string.");
    }

    // Example of accessing an integer value.
    if let Some(ConfigValue::Integer(value)) = tide_config.data.get("database.port") {
        println!("Database port: {}", value);
    } else {
        println!("Database port not found or not an integer.");
    }

     // Accessing a string array value.
    if let Some(ConfigValue::StringArray(value)) = tide_config.data.get("myApp.features") {
        println!("Features: {:?}", value);
    } else {
        println!("Features not found or not a string array.");
    }

    // Accessing an integer array value.
    if let Some(ConfigValue::IntegerArray(value)) = tide_config.data.get("myApp.numbers") {
        println!("Numbers: {:?}", value);
    } else {
        println!("Numbers not found or not an integer array.");
    }
}

