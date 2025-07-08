use anyhow::{Context, Result};
use jsonnet::JsonnetVm;
use serde_json::Value;
use std::path::PathBuf;

/// Convert Jsonnet content to YAML with optional import paths
pub fn jsonnet_to_yaml(content: &str, import_paths: &[PathBuf]) -> Result<String> {
    let mut vm = JsonnetVm::new();

    // Configure VM settings
    vm.max_stack(100);
    vm.max_trace(Some(100));

    // Add import paths
    for path in import_paths {
        vm.jpath_add(path.to_string_lossy().as_ref());
    }

    // Evaluate Jsonnet content
    let json = vm
        .evaluate_snippet("snippet", content)
        .map_err(|e| anyhow::anyhow!("Failed to evaluate Jsonnet: {}", e))?;

    // Parse JSON
    let value: Value = serde_json::from_str(&json).context("Failed to parse JSON")?;

    // Handle array of resources or single resource
    let yaml = match value {
        Value::Array(resources) => {
            // Convert each resource to YAML and join with separator
            resources
                .into_iter()
                .map(|r| serde_yaml::to_string(&r).context("Failed to convert to YAML"))
                .collect::<Result<Vec<_>>>()?
                .join("\n---\n")
        }
        _ => serde_yaml::to_string(&value).context("Failed to convert to YAML")?,
    };

    Ok(yaml)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::Path;

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
        jsonnet_to_yaml(&content, &[examples_dir()])
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
        let result = jsonnet_to_yaml(content, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_resources() {
        let content = r#"
        [
            {
                apiVersion: "v1",
                kind: "Service",
                metadata: {
                    name: "test-service"
                }
            },
            {
                apiVersion: "apps/v1",
                kind: "Deployment",
                metadata: {
                    name: "test-deployment"
                }
            }
        ]
        "#;
        let result = jsonnet_to_yaml(content, &[]).unwrap();
        assert!(result.contains("---"));
        assert!(result.contains("kind: Service"));
        assert!(result.contains("kind: Deployment"));
    }

    #[test]
    fn test_multi_document() {
        let result = evaluate_example("multi.jsonnet").unwrap();

        // Should contain all three documents
        assert!(result.contains("type: profiles"));
        assert!(result.contains("type: settings"));
        assert!(result.contains("type: features"));

        // Should be separated by ---
        let separator_count = result.matches("---").count();
        assert_eq!(
            separator_count, 2,
            "Should have 2 separators for 3 documents"
        );

        // Should contain nested data
        assert!(result.contains("name: Alice"));
        assert!(result.contains("host: localhost"));
        assert!(result.contains("samplingRate: 0.1"));
    }
}
