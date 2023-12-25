use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
pub enum ConfigValue {
    String(String),
    Integer(i32),
    Bool(bool),
    StringArray(Vec<String>),
    IntegerArray(Vec<i32>),
}

#[derive(Debug)]
pub struct TIDE {
    pub data: HashMap<String, ConfigValue>,
}

pub trait TypeConverter {
    fn to_string(&self) -> Result<String, ConfigError>;
    fn to_bool(&self) -> Result<bool, ConfigError>;
    fn to_int(&self) -> Result<i32, ConfigError>;
    fn to_string_array(&self) -> Result<Vec<String>, ConfigError>;
    fn to_int_array(&self) -> Result<Vec<i32>, ConfigError>;
}

impl TypeConverter for ConfigValue {
    fn to_string(&self) -> Result<String, ConfigError> {
        match self {
            ConfigValue::String(s) => Ok(s.clone()),
            _ => Err(ConfigError::TypeError("Expected a String".to_string())),
        }
    }

    fn to_bool(&self) -> Result<bool, ConfigError> {
        match self {
            ConfigValue::Bool(b) => Ok(*b),
            _ => Err(ConfigError::TypeError("Expected a Bool".to_string())),
        }
    }

    fn to_int(&self) -> Result<i32, ConfigError> {
        match self {
            ConfigValue::Integer(i) => Ok(*i),
            _ => Err(ConfigError::TypeError("Expected an Integer".to_string())),
        }
    }

    fn to_string_array(&self) -> Result<Vec<String>, ConfigError> {
        match self {
            ConfigValue::StringArray(arr) => Ok(arr.clone()),
            _ => Err(ConfigError::TypeError("Expected a String Array".to_string())),
        }
    }

    fn to_int_array(&self) -> Result<Vec<i32>, ConfigError> {
        match self {
            ConfigValue::IntegerArray(arr) => Ok(arr.clone()),
            _ => Err(ConfigError::TypeError("Expected an Integer Array".to_string())),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    IOError(io::Error),
    ParseError(String),
    TypeError(String),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::IOError(err)
    }
}

impl TIDE {
    pub fn new(filepath: &str) -> Result<Self, ConfigError> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        let mut data = HashMap::new();
        let mut context = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            if line.ends_with("{") {
                context.push(line.trim_end_matches(" {").to_string());
                continue;
            }

            if line == "}" {
                context.pop();
                continue;
            }

            let parts: Vec<&str> = line.splitn(2, "=").collect();
            if parts.len() != 2 {
                continue;
            }

            let key_type: Vec<&str> = parts[0].trim().splitn(2, ":").collect();
            if key_type.len() != 2 {
                continue;
            }

            let key = format!("{}.{}", context.join("."), key_type[0].trim()).trim_start_matches('.').to_string();
            let value = parts[1].trim().to_string();
            let config_value = match key_type[1].trim() {
                "string" => ConfigValue::String(value),
                "integer" => value.parse::<i32>().map(ConfigValue::Integer).map_err(|_| ConfigError::ParseError("Invalid integer".to_string()))?,
                "bool" => value.parse::<bool>().map(ConfigValue::Bool).map_err(|_| ConfigError::ParseError("Invalid bool".to_string()))?,
                "array[string]" => ConfigValue::StringArray(value.trim_matches('[').trim_matches(']').split(',').map(|s| s.trim().to_string()).collect()),
                "array[integer]" => value.trim_matches('[').trim_matches(']').split(',').map(|s| s.trim().parse::<i32>()).collect::<Result<Vec<_>, _>>().map(ConfigValue::IntegerArray).map_err(|_| ConfigError::ParseError("Invalid integer array".to_string()))?,
                _ => return Err(ConfigError::TypeError("Unknown type".to_string())),
            };

            data.insert(key, config_value);
        }

        Ok(TIDE { data })
    }

    pub fn get_config_value(&self, key: &str) -> Result<ConfigValue, ConfigError> {
        let env_key = key.to_uppercase().replace(".", "_");

        if let Ok(env_val) = env::var(&env_key) {
            return match self.data.get(key) {
                Some(ConfigValue::String(_)) => Ok(ConfigValue::String(env_val)),
                Some(ConfigValue::Integer(_)) => env_val.parse::<i32>().map(ConfigValue::Integer).map_err(|_| ConfigError::ParseError("Invalid integer from env".to_string())),
                Some(ConfigValue::Bool(_)) => env_val.parse::<bool>().map(ConfigValue::Bool).map_err(|_| ConfigError::ParseError("Invalid bool from env".to_string())),
                Some(ConfigValue::StringArray(_)) => Ok(ConfigValue::StringArray(env_val.split(',').map(String::from).collect())),
                Some(ConfigValue::IntegerArray(_)) => env_val.split(',').map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>().map(ConfigValue::IntegerArray).map_err(|_| ConfigError::ParseError("Invalid integer array from env".to_string())),
                _ => Err(ConfigError::TypeError("Unsupported type".to_string())),
            };
        }

        self.data.get(key).cloned().ok_or(ConfigError::TypeError("Key not found".to_string()))
    }
}


// Tests must be run in order to ensure that the environment variables are set and unset correctly.
// Run with `cargo test -- --test-threads=1`
#[test]
fn test_correct_parsing() {
    use std::env;

    // Ensure environment variables are unset before running the test
    env::remove_var("DATABASE_HOST");
    env::remove_var("DATABASE_PORT");

    let config_file_path = "examples/example.tide";
    let tide_config = TIDE::new(config_file_path).unwrap();

    assert_eq!(tide_config.get_config_value("database.host").unwrap(), ConfigValue::String("\"localhost\"".to_string()));
    assert_eq!(tide_config.get_config_value("database.port").unwrap(), ConfigValue::Integer(3306));
}

#[test]
fn test_correct_parsing_with_env() {
    use std::env;

    // Set environment variables for testing
    env::set_var("DATABASE_HOST", "testhost");
    env::set_var("DATABASE_PORT", "9999");

    let config_file_path = "examples/example.tide";
    let tide_config = TIDE::new(config_file_path).unwrap();

    // Test with environment variables
    assert_eq!(tide_config.get_config_value("database.host").unwrap(), ConfigValue::String("testhost".to_string()));
    assert_eq!(tide_config.get_config_value("database.port").unwrap(), ConfigValue::Integer(9999));

    // Unset environment variables to clean up
    env::remove_var("DATABASE_HOST");
    env::remove_var("DATABASE_PORT");
}
