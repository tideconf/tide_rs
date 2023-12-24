use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
// use std::str::FromStr;

#[derive(Debug, PartialEq)]
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
}

#[test]
fn test_correct_parsing() {
    let config_file_path = "examples/example.tide";
    let tide_config = TIDE::new(config_file_path).unwrap();

    assert_eq!(*tide_config.data.get("database.host").unwrap(), ConfigValue::String("\"localhost\"".to_string()));
    assert_eq!(*tide_config.data.get("database.port").unwrap(), ConfigValue::Integer(3306));
}
