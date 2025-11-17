use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Represents the trance.json manifest file for HypnoScript packages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranceManifest {
    /// Name of the ritual/package
    pub ritual_name: String,

    /// Version of the package (semver)
    pub mantra: String,

    /// Type of the project (cli, library, etc.)
    pub intent: String,

    /// Metadata about the package
    #[serde(default)]
    pub induction: Option<TranceInduction>,

    /// Contributors/authors
    #[serde(default)]
    pub hypnotists: Vec<TranceHypnotist>,

    /// Links and resources
    #[serde(default)]
    pub auras: Option<TranceAuras>,

    /// Scripts that can be run
    #[serde(default)]
    pub suggestions: HashMap<String, String>,

    /// Production dependencies
    #[serde(default)]
    pub anchors: HashMap<String, String>,

    /// Development dependencies
    #[serde(default)]
    pub deep_anchors: HashMap<String, String>,

    /// Binary/CLI configuration
    #[serde(default)]
    pub channels: Option<TranceChannels>,

    /// Lifecycle hooks
    #[serde(default)]
    pub triggers: Option<TranceTriggers>,
}

/// Metadata about the package
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranceInduction {
    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub entry_script: Option<String>,

    #[serde(default)]
    pub keywords: Vec<String>,

    #[serde(default)]
    pub license: Option<String>,
}

/// Author/contributor information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranceHypnotist {
    pub name: String,

    #[serde(default)]
    pub role: Option<String>,

    #[serde(default)]
    pub contact: Option<String>,
}

/// Links and resources
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranceAuras {
    #[serde(default)]
    pub repository: Option<String>,

    #[serde(default)]
    pub homepage: Option<String>,

    #[serde(default)]
    pub documentation: Option<String>,

    #[serde(default)]
    pub support_channel: Option<String>,
}

/// Binary/CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranceChannels {
    #[serde(default)]
    pub binary: Option<String>,

    #[serde(default)]
    pub entry: Option<String>,

    #[serde(default)]
    pub targets: Vec<String>,

    #[serde(default)]
    pub telemetry: Option<TranceTelemetry>,
}

/// Telemetry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranceTelemetry {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default)]
    pub endpoint: Option<String>,
}

/// Lifecycle hooks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranceTriggers {
    #[serde(default)]
    pub pre_focus: Option<String>,

    #[serde(default)]
    pub post_relax: Option<String>,
}

/// Represents the trance-lock.json file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranceLock {
    /// Version of the lock file format
    pub lock_version: String,

    /// Locked dependencies
    pub locked_anchors: HashMap<String, LockedDependency>,
}

/// A locked dependency with resolved version
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LockedDependency {
    /// Resolved version
    pub version: String,

    /// Resolved source/registry
    #[serde(default)]
    pub source: Option<String>,

    /// Integrity hash
    #[serde(default)]
    pub integrity: Option<String>,

    /// Transitive dependencies
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

/// Package manager implementation
pub struct PackageManager {
    /// Current working directory
    cwd: PathBuf,
}

impl PackageManager {
    /// Create a new package manager instance
    pub fn new() -> Self {
        Self {
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    /// Create a new package manager with a specific working directory
    #[cfg(test)]
    pub fn with_cwd(cwd: PathBuf) -> Self {
        Self { cwd }
    }

    /// Get the path to the trance.json file
    pub fn manifest_path(&self) -> PathBuf {
        self.cwd.join("trance.json")
    }

    /// Get the path to the trance-lock.json file
    pub fn lock_path(&self) -> PathBuf {
        self.cwd.join("trance-lock.json")
    }

    /// Load the trance.json manifest
    pub fn load_manifest(&self) -> Result<TranceManifest> {
        let path = self.manifest_path();
        if !path.exists() {
            return Err(anyhow!("No trance.json found in {}", self.cwd.display()));
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read trance.json from {}", path.display()))?;

        let manifest: TranceManifest = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse trance.json from {}", path.display()))?;

        Ok(manifest)
    }

    /// Save the trance.json manifest
    pub fn save_manifest(&self, manifest: &TranceManifest) -> Result<()> {
        let path = self.manifest_path();
        let content =
            serde_json::to_string_pretty(manifest).context("Failed to serialize manifest")?;

        fs::write(&path, content)
            .with_context(|| format!("Failed to write trance.json to {}", path.display()))?;

        Ok(())
    }

    /// Load the trance-lock.json file
    #[cfg(test)]
    pub fn load_lock(&self) -> Result<TranceLock> {
        let path = self.lock_path();
        if !path.exists() {
            // Return empty lock file if it doesn't exist
            return Ok(TranceLock {
                lock_version: "1.0.0".to_string(),
                locked_anchors: HashMap::new(),
            });
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read trance-lock.json from {}", path.display()))?;

        let lock: TranceLock = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse trance-lock.json from {}", path.display()))?;

        Ok(lock)
    }

    /// Save the trance-lock.json file
    pub fn save_lock(&self, lock: &TranceLock) -> Result<()> {
        let path = self.lock_path();
        let content =
            serde_json::to_string_pretty(lock).context("Failed to serialize lock file")?;

        fs::write(&path, content)
            .with_context(|| format!("Failed to write trance-lock.json to {}", path.display()))?;

        Ok(())
    }

    /// Initialize a new trance.json in the current directory
    pub fn init(&self, ritual_name: String, template: Option<&str>) -> Result<()> {
        let manifest_path = self.manifest_path();

        if manifest_path.exists() {
            return Err(anyhow!(
                "trance.json already exists in {}",
                self.cwd.display()
            ));
        }

        let manifest = match template {
            Some("cli") => self.create_cli_template(ritual_name),
            Some("library") => self.create_library_template(ritual_name),
            _ => self.create_basic_template(ritual_name),
        };

        self.save_manifest(&manifest)?;

        println!("✅ Created trance.json in {}", self.cwd.display());
        Ok(())
    }

    /// Create a basic template
    fn create_basic_template(&self, ritual_name: String) -> TranceManifest {
        TranceManifest {
            ritual_name,
            mantra: "0.1.0".to_string(),
            intent: "library".to_string(),
            induction: Some(TranceInduction {
                description: Some("A HypnoScript package".to_string()),
                entry_script: Some("src/main.hyp".to_string()),
                keywords: vec!["hypnoscript".to_string()],
                license: Some("MIT".to_string()),
            }),
            hypnotists: vec![],
            auras: None,
            suggestions: HashMap::new(),
            anchors: HashMap::new(),
            deep_anchors: HashMap::new(),
            channels: None,
            triggers: None,
        }
    }

    /// Create a CLI template
    fn create_cli_template(&self, ritual_name: String) -> TranceManifest {
        let mut suggestions = HashMap::new();
        suggestions.insert(
            "focus".to_string(),
            "hypnoscript run src/main.hyp".to_string(),
        );
        suggestions.insert(
            "test".to_string(),
            "hypnoscript run tests/smoke.hyp".to_string(),
        );

        TranceManifest {
            ritual_name: ritual_name.clone(),
            mantra: "0.1.0".to_string(),
            intent: "cli".to_string(),
            induction: Some(TranceInduction {
                description: Some(format!("A HypnoScript CLI application: {}", ritual_name)),
                entry_script: Some("src/main.hyp".to_string()),
                keywords: vec!["hypnoscript".to_string(), "cli".to_string()],
                license: Some("MIT".to_string()),
            }),
            hypnotists: vec![],
            auras: None,
            suggestions,
            anchors: HashMap::new(),
            deep_anchors: HashMap::new(),
            channels: Some(TranceChannels {
                binary: Some(ritual_name),
                entry: Some("focus".to_string()),
                targets: vec![
                    "windows-x64".to_string(),
                    "linux-x64".to_string(),
                    "macos-universal".to_string(),
                ],
                telemetry: Some(TranceTelemetry {
                    enabled: false,
                    endpoint: None,
                }),
            }),
            triggers: None,
        }
    }

    /// Create a library template
    fn create_library_template(&self, ritual_name: String) -> TranceManifest {
        let mut suggestions = HashMap::new();
        suggestions.insert(
            "test".to_string(),
            "hypnoscript run tests/test.hyp".to_string(),
        );
        suggestions.insert(
            "build".to_string(),
            "hypnoscript compile-wasm src/lib.hyp".to_string(),
        );

        TranceManifest {
            ritual_name,
            mantra: "0.1.0".to_string(),
            intent: "library".to_string(),
            induction: Some(TranceInduction {
                description: Some("A HypnoScript library".to_string()),
                entry_script: Some("src/lib.hyp".to_string()),
                keywords: vec!["hypnoscript".to_string(), "library".to_string()],
                license: Some("MIT".to_string()),
            }),
            hypnotists: vec![],
            auras: None,
            suggestions,
            anchors: HashMap::new(),
            deep_anchors: HashMap::new(),
            channels: None,
            triggers: None,
        }
    }

    /// Add a dependency to the manifest
    pub fn add_dependency(&self, name: String, version: String, dev: bool) -> Result<()> {
        let mut manifest = self.load_manifest()?;

        if dev {
            manifest.deep_anchors.insert(name.clone(), version.clone());
            println!("✅ Added {} @ {} to deepAnchors", name, version);
        } else {
            manifest.anchors.insert(name.clone(), version.clone());
            println!("✅ Added {} @ {} to anchors", name, version);
        }

        self.save_manifest(&manifest)?;
        Ok(())
    }

    /// Remove a dependency from the manifest
    pub fn remove_dependency(&self, name: &str) -> Result<()> {
        let mut manifest = self.load_manifest()?;

        let removed_from_anchors = manifest.anchors.remove(name).is_some();
        let removed_from_deep = manifest.deep_anchors.remove(name).is_some();

        if !removed_from_anchors && !removed_from_deep {
            return Err(anyhow!("Dependency '{}' not found in manifest", name));
        }

        self.save_manifest(&manifest)?;
        println!("✅ Removed {} from manifest", name);
        Ok(())
    }

    /// List all dependencies
    pub fn list_dependencies(&self) -> Result<()> {
        let manifest = self.load_manifest()?;

        println!("📦 {} v{}", manifest.ritual_name, manifest.mantra);

        if !manifest.anchors.is_empty() {
            println!("\nAnchors (dependencies):");
            for (name, version) in &manifest.anchors {
                println!("  {} @ {}", name, version);
            }
        }

        if !manifest.deep_anchors.is_empty() {
            println!("\nDeep Anchors (devDependencies):");
            for (name, version) in &manifest.deep_anchors {
                println!("  {} @ {}", name, version);
            }
        }

        if manifest.anchors.is_empty() && manifest.deep_anchors.is_empty() {
            println!("\nNo dependencies installed.");
        }

        Ok(())
    }

    /// Install all dependencies
    pub fn install(&self) -> Result<()> {
        let manifest = self.load_manifest()?;

        println!(
            "📦 Installing dependencies for {} v{}",
            manifest.ritual_name, manifest.mantra
        );

        // For now, we'll create a lock file with the dependencies
        // In a full implementation, this would fetch packages from a registry
        let mut lock = TranceLock {
            lock_version: "1.0.0".to_string(),
            locked_anchors: HashMap::new(),
        };

        // Lock production dependencies
        for (name, version_spec) in &manifest.anchors {
            lock.locked_anchors.insert(
                name.clone(),
                LockedDependency {
                    version: version_spec.clone(),
                    source: Some("registry".to_string()),
                    integrity: None,
                    dependencies: HashMap::new(),
                },
            );
        }

        // Lock development dependencies
        for (name, version_spec) in &manifest.deep_anchors {
            lock.locked_anchors.insert(
                name.clone(),
                LockedDependency {
                    version: version_spec.clone(),
                    source: Some("registry".to_string()),
                    integrity: None,
                    dependencies: HashMap::new(),
                },
            );
        }

        self.save_lock(&lock)?;

        println!(
            "✅ Dependencies installed ({} total)",
            lock.locked_anchors.len()
        );
        println!("📝 Lock file written to trance-lock.json");

        Ok(())
    }

    /// Run a suggestion (script) from the manifest
    pub fn run_suggestion(&self, name: &str) -> Result<String> {
        let manifest = self.load_manifest()?;

        let command = manifest
            .suggestions
            .get(name)
            .ok_or_else(|| anyhow!("Suggestion '{}' not found in manifest", name))?;

        Ok(command.clone())
    }

    /// Validate the manifest
    pub fn validate(&self) -> Result<()> {
        let manifest = self.load_manifest()?;

        // Validate ritual name
        if manifest.ritual_name.is_empty() {
            return Err(anyhow!("ritualName cannot be empty"));
        }

        // Validate version
        semver::Version::parse(&manifest.mantra)
            .with_context(|| format!("Invalid version in mantra: {}", manifest.mantra))?;

        // Validate dependency versions
        for (name, version) in manifest.anchors.iter().chain(manifest.deep_anchors.iter()) {
            if !version.starts_with('^') && !version.starts_with('~') && !version.starts_with('=') {
                semver::Version::parse(version).with_context(|| {
                    format!("Invalid version for dependency {}: {}", name, version)
                })?;
            }
        }

        println!("✅ Manifest is valid");
        Ok(())
    }
}

impl Default for PackageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_basic_template() {
        let pm = PackageManager::new();
        let manifest = pm.create_basic_template("test-package".to_string());

        assert_eq!(manifest.ritual_name, "test-package");
        assert_eq!(manifest.mantra, "0.1.0");
        assert_eq!(manifest.intent, "library");
    }

    #[test]
    fn test_create_cli_template() {
        let pm = PackageManager::new();
        let manifest = pm.create_cli_template("test-cli".to_string());

        assert_eq!(manifest.ritual_name, "test-cli");
        assert_eq!(manifest.intent, "cli");
        assert!(manifest.channels.is_some());
        assert!(manifest.suggestions.contains_key("focus"));
    }

    #[test]
    fn test_init_manifest() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let pm = PackageManager::with_cwd(temp_dir.path().to_path_buf());

        pm.init("test-package".to_string(), None)?;

        assert!(pm.manifest_path().exists());

        let manifest = pm.load_manifest()?;
        assert_eq!(manifest.ritual_name, "test-package");

        Ok(())
    }

    #[test]
    fn test_add_remove_dependency() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let pm = PackageManager::with_cwd(temp_dir.path().to_path_buf());

        pm.init("test-package".to_string(), None)?;
        pm.add_dependency(
            "hypnoscript-runtime".to_string(),
            "^1.0.0".to_string(),
            false,
        )?;

        let manifest = pm.load_manifest()?;
        assert!(manifest.anchors.contains_key("hypnoscript-runtime"));

        pm.remove_dependency("hypnoscript-runtime")?;

        let manifest = pm.load_manifest()?;
        assert!(!manifest.anchors.contains_key("hypnoscript-runtime"));

        Ok(())
    }

    #[test]
    fn test_install_creates_lock() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let pm = PackageManager::with_cwd(temp_dir.path().to_path_buf());

        pm.init("test-package".to_string(), None)?;
        pm.add_dependency("dep1".to_string(), "^1.0.0".to_string(), false)?;
        pm.add_dependency("dep2".to_string(), "^2.0.0".to_string(), true)?;

        pm.install()?;

        assert!(pm.lock_path().exists());

        let lock = pm.load_lock()?;
        assert_eq!(lock.locked_anchors.len(), 2);
        assert!(lock.locked_anchors.contains_key("dep1"));
        assert!(lock.locked_anchors.contains_key("dep2"));

        Ok(())
    }

    #[test]
    fn test_manifest_serialization() -> Result<()> {
        let manifest = TranceManifest {
            ritual_name: "test".to_string(),
            mantra: "1.0.0".to_string(),
            intent: "library".to_string(),
            induction: Some(TranceInduction {
                description: Some("Test".to_string()),
                entry_script: Some("main.hyp".to_string()),
                keywords: vec!["test".to_string()],
                license: Some("MIT".to_string()),
            }),
            hypnotists: vec![],
            auras: None,
            suggestions: HashMap::new(),
            anchors: HashMap::new(),
            deep_anchors: HashMap::new(),
            channels: None,
            triggers: None,
        };

        let json = serde_json::to_string(&manifest)?;
        let deserialized: TranceManifest = serde_json::from_str(&json)?;

        assert_eq!(manifest.ritual_name, deserialized.ritual_name);
        assert_eq!(manifest.mantra, deserialized.mantra);

        Ok(())
    }
}
