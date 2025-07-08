use anyhow::{Context, Result};
use jsonnet::JsonnetVm;
use std::{path::PathBuf, fs};
use serde_json::Value;

/// Convert Jsonnet content to YAML
pub fn jsonnet_to_yaml(content: &str, filename: &str) -> Result<String> {
    // Create Jsonnet VM
    let mut vm = JsonnetVm::new();

    // Set the import callback to handle file imports
    vm.import_callback(|_base_dir, rel_path, _| {
        let path = PathBuf::from(rel_path);
        match fs::read_to_string(&path) {
            Ok(content) => Ok((content.into(), path.to_string_lossy().into_owned())),
            Err(e) => Err(format!("Failed to read import '{}': {}", path.display(), e)),
        }
    });

    // Configure VM settings
    vm.max_stack(100);
    vm.max_trace(Some(100));
    
    // Evaluate Jsonnet content
    let json = vm.evaluate_snippet(filename, content)
        .map_err(|e| anyhow::anyhow!("Failed to evaluate Jsonnet: {}", e))?;

    // Parse JSON and convert to YAML
    let value: Value = serde_json::from_str(&json)
        .context("Failed to parse JSON")?;
    let yaml = serde_yaml::to_string(&value)
        .context("Failed to convert to YAML")?;

    Ok(yaml)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::env;

    fn examples_dir() -> PathBuf {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set");
        Path::new(&manifest_dir).join("examples")
    }

    fn read_example(name: &str) -> String {
        let path = examples_dir().join(name);
        fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read example {}: {}", name, e))
    }

    fn evaluate_example(name: &str) -> Result<String> {
        let content = read_example(name);
        let mut vm = JsonnetVm::new();
        
        // Set up import callback to handle relative paths from examples directory
        let examples = examples_dir();
        vm.jpath_add(examples.to_string_lossy().as_ref());
        
        vm.max_stack(100);
        vm.max_trace(Some(100));
        
        let json = vm.evaluate_snippet(name, &content)
            .map_err(|e| anyhow::anyhow!("Failed to evaluate Jsonnet: {}", e))?;

        let value: Value = serde_json::from_str(&json)
            .context("Failed to parse JSON")?;
        let yaml = serde_yaml::to_string(&value)
            .context("Failed to convert to YAML")?;

        Ok(yaml)
    }

    #[test]
    fn test_basic_jsonnet() {
        let result = evaluate_example("basic.jsonnet").unwrap();
        
        // Basic example should contain these fields
        assert!(result.contains("message:"));
        assert!(result.contains("sum:"));
        assert!(result.contains("product:"));
        assert!(result.contains("squares:"));
    }

    #[test]
    fn test_functions_jsonnet() {
        let result = evaluate_example("functions.jsonnet").unwrap();
        
        // Functions example should contain these fields
        assert!(result.contains("people:"));
        assert!(result.contains("settings:"));
        assert!(result.contains("prices:"));
    }

    #[test]
    fn test_imports_jsonnet() {
        let result = evaluate_example("imports.jsonnet").unwrap();
        
        // Imports example should contain these fields and values
        assert!(result.contains("numbers:"));
        assert!(result.contains("strings:"));
        assert!(result.contains("objects:"));
        assert!(result.contains("combined:"));
    }

    #[test]
    fn test_invalid_jsonnet() {
        let result = jsonnet_to_yaml("{ invalid syntax }", "invalid.jsonnet");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_jsonnet() {
        let result = jsonnet_to_yaml("{}", "empty.jsonnet").unwrap();
        assert_eq!(result.trim(), "{}");
    }

    #[test]
    fn test_import_not_found() {
        let content = r#"
        local nonexistent = import 'nonexistent.libsonnet';
        { field: nonexistent }
        "#;
        let result = jsonnet_to_yaml(content, "test.jsonnet");
        assert!(result.is_err());
    }
} 