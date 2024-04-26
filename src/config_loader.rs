use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind};
use std::result::Result;


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub transformations: Vec<Transformation>,
    pub queries: Vec<Query>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transformation {
    pub name: String,
    pub description: String,
    pub rule: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub name: String,
    pub description: String,
    pub sql: String,
}

pub fn load_config(file_path: &str) -> Result<Config, std::io::Error> {
    // Read the file content
    let config_str = fs::read_to_string(file_path)?;

    // Parse YAML, manually converting serde_yaml::Error to std::io::Error
    let config: Config = serde_yaml::from_str(&config_str)
        .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    fn setup_config_file(contents: &str) -> std::io::Result<NamedTempFile> {
        let mut file = NamedTempFile::new()?;
        write!(file, "{}", contents)?;
        Ok(file)
    }

    #[test]
    fn test_load_config_success() {
        let yaml = r#"
        transformations:
          - name: "transform1"
            description: "Description of transform1"
            rule: "rule1"
        queries:
          - name: "query1"
            description: "Description of query1"
            sql: "SELECT * FROM table1"
        "#;

        let file = setup_config_file(yaml).expect("Failed to create file");
        let config = load_config(file.path().to_str().unwrap()).expect("Failed to load config");

        assert_eq!(config.transformations.len(), 1);
        assert_eq!(config.transformations[0].name, "transform1");
        assert_eq!(config.queries.len(), 1);
        assert_eq!(config.queries[0].sql, "SELECT * FROM table1");
    }

    #[test]
    fn test_load_config_file_not_found() {
        let result = load_config("non_existent_file.yaml");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_config_bad_yaml() {
        let yaml = r#"
        transformations:
          - name: "transform1"
            description: "Description of transform1"
            rule: "rule1"
        queries:
          - name: "query1"
            description: "Description of query1"
            sql: "SELECT * FROM table1
        "#;  // Intentionally malformed YAML

        let file = setup_config_file(yaml).expect("Failed to create file");
        let result = load_config(file.path().to_str().unwrap());
        assert!(result.is_err());
    }
}
