use anyhow::{Context, Result};
use jsonnet::JsonnetVm;
use serde_json::Value;

/// Convert Jsonnet content to YAML
pub fn jsonnet_to_yaml(content: &str, filename: &str) -> Result<String> {
    let mut vm = JsonnetVm::new();

    // Configure VM settings
    vm.max_stack(100);
    vm.max_trace(Some(100));

    // Evaluate Jsonnet content
    let json = vm
        .evaluate_snippet(filename, content)
        .map_err(|e| anyhow::anyhow!("Failed to evaluate Jsonnet: {}", e))?;

    // Parse JSON and convert to YAML
    let value: Value = serde_json::from_str(&json).context("Failed to parse JSON")?;
    let yaml = serde_yaml::to_string(&value).context("Failed to convert to YAML")?;

    Ok(yaml)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn examples_dir() -> PathBuf {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
        Path::new(&manifest_dir).join("examples")
    }

    fn read_example(name: &str) -> String {
        let path = examples_dir().join(name);
        fs::read_to_string(path).expect("Failed to read example file")
    }

    fn evaluate_example(name: &str) -> Result<String> {
        let content = read_example(name);
        let mut vm = JsonnetVm::new();

        // Set up import callback to handle relative paths from examples directory
        let examples = examples_dir();
        vm.jpath_add(examples.to_string_lossy().as_ref());

        vm.max_stack(100);
        vm.max_trace(Some(100));

        let json = vm
            .evaluate_snippet(name, &content)
            .map_err(|e| anyhow::anyhow!("Failed to evaluate Jsonnet: {}", e))?;

        let value: Value = serde_json::from_str(&json).context("Failed to parse JSON")?;
        let yaml = serde_yaml::to_string(&value).context("Failed to convert to YAML")?;

        Ok(yaml)
    }

    #[test]
    fn test_basic_jsonnet() {
        let result = evaluate_example("basic.jsonnet").unwrap();

        // Basic example should contain these fields
        assert!(result.contains("message:"));
        assert!(result.contains("sum:"));
        assert!(result.contains("squares:"));
    }

    #[test]
    fn test_functions_jsonnet() {
        let result = evaluate_example("functions.jsonnet").unwrap();

        // Functions example should contain these fields
        assert!(result.contains("people:"));
        assert!(result.contains("settings:"));
    }

    #[test]
    fn test_imports_jsonnet() {
        let result = evaluate_example("imports.jsonnet").unwrap();

        // Imports example should contain these fields and values
        assert!(result.contains("numbers:"));
        assert!(result.contains("strings:"));
        assert!(result.contains("combined:"));
    }

    #[test]
    fn test_invalid_jsonnet() {
        let content = r#"
            {
                invalid: 'jsonnet,
            }
        "#;
        let result = jsonnet_to_yaml(content, "test.jsonnet");
        assert!(result.is_err());
    }
}
