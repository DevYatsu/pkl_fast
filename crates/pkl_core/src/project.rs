//! Pkl project support — loads `PklProject` files and resolves dependencies.
//!
//! A Pkl project is defined by a `PklProject` file that amends `pkl:Project`.
//! It declares dependencies on remote packages and local projects.
//!
//! Use [`load_project`] to load a project from a directory, then pass it
//! to `EvaluatorManager::new_project_evaluator`.

use crate::error::{PklError, PklResult};
use crate::evaluator::PklEvaluator;
use crate::module_source::ModuleSource;

/// A loaded Pkl project, matching `pkl:Project`.
#[derive(Debug, Clone)]
pub struct Project {
    /// URI of the PklProject file.
    pub project_file_uri: String,
    /// Package metadata (if this project is a package).
    pub package: Option<ProjectPackage>,
    /// Declared dependencies.
    pub dependencies: ProjectDependencies,
}

/// Package metadata from a `PklProject`.
#[derive(Debug, Clone)]
pub struct ProjectPackage {
    pub name: String,
    pub base_uri: String,
    pub version: String,
    pub package_zip_url: String,
    pub uri: String,
}

/// Resolved project dependencies (local + remote).
#[derive(Debug, Clone, Default)]
pub struct ProjectDependencies {
    pub local: Vec<ProjectLocalDependency>,
    pub remote: Vec<ProjectRemoteDependency>,
}

/// A local project dependency (another PklProject on disk).
#[derive(Debug, Clone)]
pub struct ProjectLocalDependency {
    pub package_uri: String,
    pub project_file_uri: String,
    pub dependencies: ProjectDependencies,
}

/// A remote package dependency.
#[derive(Debug, Clone)]
pub struct ProjectRemoteDependency {
    pub package_uri: String,
    pub checksums: Option<String>,
    pub dependencies: ProjectDependencies,
}

/// Load a project from a directory containing `PklProject` (and optionally `PklProject.deps.json`).
pub async fn load_project(project_dir: &str) -> PklResult<Project> {
    // Use a temporary evaluator to load the PklProject file
    let source = ModuleSource::from_file(&format!("{}/PklProject", project_dir));
    load_project_from_source(&source).await
}

/// Load a project from a module source (evaluates the PklProject file).
pub async fn load_project_from_source(source: &ModuleSource) -> PklResult<Project> {
    // Create a temporary evaluator to parse the PklProject
    let eval = crate::CliEvaluator::new();

    // Evaluate the PklProject file to get its value tree
    let value = eval.evaluate_raw(source).await?;
    project_from_value(&value)
}

fn project_from_value(value: &crate::Value) -> PklResult<Project> {
    let obj = value.as_object().ok_or_else(|| {
        PklError::DecodeError("expected object for Project".to_string())
    })?;

    let project_file_uri = get_str(obj, "projectFileUri")
        .unwrap_or("").to_string();

    let package = obj.get("package").and_then(|v| {
        let o = v.as_object()?;
        Some(ProjectPackage {
            name: get_str(o, "name").unwrap_or("").to_string(),
            base_uri: get_str(o, "baseUri").unwrap_or("").to_string(),
            version: get_str(o, "version").unwrap_or("").to_string(),
            package_zip_url: get_str(o, "packageZipUrl").unwrap_or("").to_string(),
            uri: get_str(o, "uri").unwrap_or("").to_string(),
        })
    });

    let dependencies = parse_dependencies(obj.get("dependencies")).unwrap_or_default();

    Ok(Project { project_file_uri, package, dependencies })
}

fn parse_dependencies(value: Option<&crate::Value>) -> Option<ProjectDependencies> {
    // Convert both Object (String→Value) and Mapping (Value→Value) to a unified string map
    let items: Vec<(String, &crate::Value)> = match value {
        Some(crate::Value::Object(m)) => {
            m.iter().map(|(k, v)| (k.clone(), v)).collect()
        }
        Some(crate::Value::Mapping(m)) => {
            m.iter().map(|(k, v)| {
                let key_str = match k {
                    crate::Value::String(s) => s.clone(),
                    other => format!("{:?}", other),
                };
                (key_str, v)
            }).collect()
        }
        _ => return None,
    };

    let mut local = Vec::new();
    let mut remote = Vec::new();

    for (_name, dep_val) in &items {
        let dep_obj = match dep_val.as_object() {
            Some(o) => o,
            _ => continue,
        };

        // Determine if it's local (has projectFileUri) or remote (has uri)
        if let Some(project_file_uri) = get_str(dep_obj, "projectFileUri") {
            local.push(ProjectLocalDependency {
                package_uri: get_str(dep_obj, "packageUri").or_else(|| get_str(dep_obj, "uri")).unwrap_or("").to_string(),
                project_file_uri: project_file_uri.to_string(),
                dependencies: parse_dependencies(dep_obj.get("dependencies")).unwrap_or_default(),
            });
        } else if let Some(package_uri) = get_str(dep_obj, "uri").or_else(|| get_str(dep_obj, "packageUri")) {
            let checksums = dep_obj.get("checksums").and_then(|c| c.as_object())
                .and_then(|c| get_str(c, "sha256"))
                .map(|s| s.to_string());
            remote.push(ProjectRemoteDependency {
                package_uri: package_uri.to_string(),
                checksums,
                dependencies: parse_dependencies(dep_obj.get("dependencies")).unwrap_or_default(),
            });
        }
    }

    Some(ProjectDependencies { local, remote })
}

fn get_str<'a>(obj: &'a std::collections::BTreeMap<String, crate::Value>, key: &str) -> Option<&'a str> {
    obj.get(key).and_then(|v| v.as_str())
}

/// Convert a `Project` into the protocol-level `ProjectOrDependency` message.
pub fn project_to_message(project: &Project) -> crate::msgapi::ProjectOrDependency {
    let deps = project_deps_to_message(&project.dependencies);
    let checksums = None; // Root project doesn't have checksums
    crate::msgapi::ProjectOrDependency {
        packageUri: project.package.as_ref().map(|p| p.uri.clone()),
        r#type: "project".to_string(),
        projectFileUri: Some(project.project_file_uri.clone()),
        checksums,
        dependencies: deps,
    }
}

fn project_deps_to_message(deps: &ProjectDependencies) -> std::collections::BTreeMap<String, crate::msgapi::ProjectOrDependency> {
    let mut map = std::collections::BTreeMap::new();

    for dep in &deps.local {
        let inner = crate::msgapi::ProjectOrDependency {
            packageUri: Some(dep.package_uri.clone()),
            r#type: "local".to_string(),
            projectFileUri: Some(dep.project_file_uri.clone()),
            checksums: None,
            dependencies: project_deps_to_message(&dep.dependencies),
        };
        // Use package URI or a placeholder as the key name
        map.insert(dep.package_uri.clone(), inner);
    }

    for dep in &deps.remote {
        let inner = crate::msgapi::ProjectOrDependency {
            packageUri: Some(dep.package_uri.clone()),
            r#type: "remote".to_string(),
            projectFileUri: None,
            checksums: dep.checksums.clone().map(|c| crate::msgapi::Checksums { sha256: c }),
            dependencies: project_deps_to_message(&dep.dependencies),
        };
        map.insert(dep.package_uri.clone(), inner);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_project_basic() {
        if std::process::Command::new("pkl").arg("--version").output().is_err() {
            eprintln!("Skipping: pkl CLI not found");
            return;
        }

        // Create a minimal PklProject file
        let dir = std::env::temp_dir().join("pkl_project_test");
        std::fs::create_dir_all(&dir).ok();
        std::fs::write(dir.join("PklProject"), r#"
amends "pkl:Project"

package {
  name = "my-project"
  baseUri = "package://example.com/my-project"
  version = "1.0.0"
  packageZipUrl = "https://example.com/my-project@1.0.0.zip"
}
"#).ok();

        let project = load_project(dir.to_str().unwrap()).await.unwrap();
        assert!(project.package.is_some());
        let pkg = project.package.unwrap();
        assert_eq!(pkg.name, "my-project");
        assert_eq!(pkg.version, "1.0.0");

        // Cleanup
        std::fs::remove_dir_all(&dir).ok();
    }

    #[tokio::test]
    async fn test_load_project_with_deps() {
        if std::process::Command::new("pkl").arg("--version").output().is_err() {
            eprintln!("Skipping: pkl CLI not found");
            return;
        }

        let dir = std::env::temp_dir().join("pkl_project_deps_test");
        std::fs::create_dir_all(&dir).ok();
        std::fs::write(dir.join("PklProject"), r#"
amends "pkl:Project"

dependencies {
  ["examples"] {
    uri = "package://example.com/examples@1.0.0"
  }
}
"#).ok();

        let project = load_project(dir.to_str().unwrap()).await.unwrap();
        // Remote deps should be found
        assert_eq!(project.dependencies.remote.len(), 1, "expected 1 remote dep, got {} (local: {})", project.dependencies.remote.len(), project.dependencies.local.len());
        if !project.dependencies.remote.is_empty() {
            assert_eq!(project.dependencies.remote[0].package_uri, "package://example.com/examples@1.0.0",
                "expected package_uri to match the uri from PklProject");
        }

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_project_to_message_conversion() {
        let project = Project {
            project_file_uri: "file:///project/PklProject".to_string(),
            package: Some(ProjectPackage {
                name: "test".to_string(),
                base_uri: "package://test/test".to_string(),
                version: "1.0.0".to_string(),
                package_zip_url: "https://test/test@1.0.0.zip".to_string(),
                uri: "package://test/test@1.0.0".to_string(),
            }),
            dependencies: ProjectDependencies {
                local: vec![],
                remote: vec![ProjectRemoteDependency {
                    package_uri: "package://example/lib@2.0.0".to_string(),
                    checksums: Some("abc123".to_string()),
                    dependencies: ProjectDependencies { local: vec![], remote: vec![] },
                }],
            },
        };

        let msg = project_to_message(&project);
        assert_eq!(msg.r#type, "project");
        assert!(msg.projectFileUri.is_some());
        assert_eq!(msg.dependencies.len(), 1);
    }
}
