use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use hypnoscript_compiler::{Interpreter, TypeChecker, WasmCodeGenerator};
use hypnoscript_lexer_parser::{Lexer, Parser as HypnoParser};
use semver::Version;
use serde::Deserialize;
use std::{env, fs, time::Duration};
use ureq::{Agent, AgentBuilder};

#[cfg(not(target_os = "windows"))]
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

const GITHUB_OWNER: &str = "Kink-Development-Group";
const GITHUB_REPO: &str = "hyp-runtime";
const GITHUB_API: &str = "https://api.github.com";
#[cfg(not(target_os = "windows"))]
const INSTALLER_FALLBACK_URL: &str =
    "https://kink-development-group.github.io/hyp-runtime/install.sh";

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn into_anyhow<E: std::fmt::Display>(error: E) -> anyhow::Error {
    anyhow::Error::msg(error.to_string())
}

#[derive(Parser)]
#[command(name = "hypnoscript")]
#[command(about = "HypnoScript - The Hypnotic Programming Language (Rust Edition)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a HypnoScript file
    Run {
        /// Path to the .hyp file
        file: String,

        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Lex a HypnoScript file (tokenize)
    Lex {
        /// Path to the .hyp file
        file: String,
    },

    /// Parse a HypnoScript file (show AST)
    Parse {
        /// Path to the .hyp file
        file: String,
    },

    /// Type check a HypnoScript file
    Check {
        /// Path to the .hyp file
        file: String,
    },

    /// Compile to WASM
    CompileWasm {
        /// Path to the .hyp file
        input: String,

        /// Output WASM file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Update oder prüfe die HypnoScript-Installation
    #[command(name = "self-update", alias = "update")]
    SelfUpdate {
        /// Nur nach Updates suchen, keine Installation durchführen
        #[arg(long)]
        check: bool,

        /// Vorabversionen berücksichtigen
        #[arg(long)]
        include_prerelease: bool,

        /// Installation erzwingen, selbst wenn Version identisch ist
        #[arg(long)]
        force: bool,

        /// Installer-Ausgabe reduzieren
        #[arg(long)]
        quiet: bool,

        /// Kein sudo für den Installer verwenden
        #[arg(long)]
        no_sudo: bool,
    },

    /// Show version information
    Version,

    /// Show builtin functions
    Builtins,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            file,
            debug,
            verbose,
        } => {
            if verbose {
                println!("Running file: {}", file);
            }

            let source = fs::read_to_string(&file)?;

            if debug {
                println!("Source code:");
                println!("{}", source);
                println!("\n--- Lexing ---");
            }

            // Lex
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(into_anyhow)?;

            if debug {
                println!("Tokens: {}", tokens.len());
            }

            // Parse
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(into_anyhow)?;

            if debug {
                println!("\n--- Type Checking ---");
            }

            // Type check
            let mut type_checker = TypeChecker::new();
            let errors = type_checker.check_program(&ast);
            if !errors.is_empty() {
                eprintln!("Type errors:");
                for error in errors {
                    eprintln!("  - {}", error);
                }
                if !debug {
                    eprintln!("\nContinuing execution despite type errors...");
                }
            }

            if debug {
                println!("\n--- Executing ---");
            }

            // Execute
            let mut interpreter = Interpreter::new();
            interpreter.execute_program(ast).map_err(into_anyhow)?;

            if verbose {
                println!("\n✅ Program executed successfully!");
            }
        }

        Commands::Lex { file } => {
            let source = fs::read_to_string(&file)?;
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(into_anyhow)?;

            println!("=== Tokens ===");
            for (i, token) in tokens.iter().enumerate() {
                println!("{:4}: {:?}", i, token);
            }
            println!("\nTotal tokens: {}", tokens.len());
        }

        Commands::Parse { file } => {
            let source = fs::read_to_string(&file)?;
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(into_anyhow)?;
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(into_anyhow)?;

            println!("=== AST ===");
            println!("{:#?}", ast);
        }

        Commands::Check { file } => {
            let source = fs::read_to_string(&file)?;
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(into_anyhow)?;
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(into_anyhow)?;

            let mut type_checker = TypeChecker::new();
            let errors = type_checker.check_program(&ast);

            if errors.is_empty() {
                println!("✅ No type errors found!");
            } else {
                println!("❌ Type errors found:");
                for error in errors {
                    println!("  - {}", error);
                }
            }
        }

        Commands::CompileWasm { input, output } => {
            let source = fs::read_to_string(&input)?;
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(into_anyhow)?;
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(into_anyhow)?;

            let mut generator = WasmCodeGenerator::new();
            let wasm_code = generator.generate(&ast);

            let output_file = output.unwrap_or_else(|| input.replace(".hyp", ".wat"));

            fs::write(&output_file, wasm_code)?;
            println!("✅ WASM code written to: {}", output_file);
        }

        Commands::SelfUpdate {
            check,
            include_prerelease,
            force,
            quiet,
            no_sudo,
        } => {
            handle_self_update(check, include_prerelease, force, quiet, no_sudo)?;
        }

        Commands::Version => {
            println!("HypnoScript v{} (Rust Edition)", env!("CARGO_PKG_VERSION"));
            println!("The Hypnotic Programming Language");
            println!();
            println!("Migrated from C# to Rust for improved performance");
            println!();
            println!("Features:");
            println!("  - Full parser and interpreter");
            println!("  - Type checker");
            println!("  - WASM code generation");
            println!("  - 110+ builtin functions");
        }

        Commands::Builtins => {
            println!("=== HypnoScript Builtin Functions ===\n");

            println!("📊 Math Builtins:");
            println!("  - Sin, Cos, Tan, Sqrt, Pow, Log, Log10");
            println!("  - Abs, Floor, Ceil, Round, Min, Max");
            println!("  - Factorial, Gcd, Lcm, IsPrime, Fibonacci");
            println!("  - Clamp");

            println!("\n📝 String Builtins:");
            println!("  - Length, ToUpper, ToLower, Trim");
            println!("  - IndexOf, Replace, Reverse, Capitalize");
            println!("  - StartsWith, EndsWith, Contains");
            println!("  - Split, Substring, Repeat");
            println!("  - PadLeft, PadRight");

            println!("\n📦 Array Builtins:");
            println!("  - Length, IsEmpty, Get, IndexOf, Contains");
            println!("  - Reverse, Sum, Average, Min, Max, Sort");
            println!("  - First, Last, Take, Skip, Slice");
            println!("  - Join, Count, Distinct");

            println!("\n✨ Hypnotic Builtins:");
            println!("  - observe (output)");
            println!("  - drift (sleep)");
            println!("  - DeepTrance");
            println!("  - HypnoticCountdown");
            println!("  - TranceInduction");
            println!("  - HypnoticVisualization");

            println!("\n🔄 Conversion Functions:");
            println!("  - ToInt, ToDouble, ToString, ToBoolean");

            println!("\nTotal: 50+ builtin functions implemented");
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    #[allow(dead_code)]
    prerelease: bool,
    #[allow(dead_code)]
    draft: bool,
}

#[cfg(not(target_os = "windows"))]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct InstallMetadata {
    prefix: Option<String>,
    version: Option<String>,
    target: Option<String>,
}

fn build_agent() -> Agent {
    AgentBuilder::new()
        .timeout(Duration::from_secs(20))
        .user_agent(&format!("hypnoscript-cli/{}", env!("CARGO_PKG_VERSION")))
        .build()
}

fn github_get(agent: &Agent, url: &str) -> ureq::Request {
    let mut request = agent.get(url).set("Accept", "application/vnd.github+json");

    if let Ok(token) = env::var("GITHUB_TOKEN") {
        request = request
            .set("Authorization", &format!("Bearer {}", token))
            .set("X-GitHub-Api-Version", "2022-11-28");
    }

    request
}

fn fetch_latest_release(agent: &Agent, include_prerelease: bool) -> Result<GithubRelease> {
    if include_prerelease {
        let url = format!(
            "{}/repos/{}/{}/releases",
            GITHUB_API, GITHUB_OWNER, GITHUB_REPO
        );
        let releases: Vec<GithubRelease> = github_get(agent, &url)
            .call()?
            .into_json::<Vec<GithubRelease>>()?
            .into_iter()
            .filter(|release| !release.draft)
            .collect();

        releases
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("Keine Veröffentlichung gefunden"))
    } else {
        let url = format!(
            "{}/repos/{}/{}/releases/latest",
            GITHUB_API, GITHUB_OWNER, GITHUB_REPO
        );
        let release: GithubRelease = github_get(agent, &url).call()?.into_json()?;
        Ok(release)
    }
}

fn parse_version(tag: &str) -> Result<Version> {
    let normalized = tag.trim_start_matches(['v', 'V']);
    Version::parse(normalized).map_err(|err| anyhow!("Ungültige Versionsangabe '{}': {}", tag, err))
}

#[cfg(target_os = "windows")]
fn handle_self_update(
    check: bool,
    include_prerelease: bool,
    _force: bool,
    _quiet: bool,
    _no_sudo: bool,
) -> Result<()> {
    let agent = build_agent();
    let release = fetch_latest_release(&agent, include_prerelease)?;
    let latest_version = parse_version(&release.tag_name)?;
    let current_version = Version::parse(env!("CARGO_PKG_VERSION"))?;

    if check {
        if latest_version > current_version {
            println!("Update verfügbar: {} → {}", current_version, latest_version);
        } else {
            println!("HypnoScript ist aktuell (Version {}).", current_version);
        }
        return Ok(());
    }

    Err(anyhow!(
        "Self-Update wird unter Windows derzeit nicht unterstützt. Bitte lade das aktuelle Release manuell herunter."
    ))
}

#[cfg(not(target_os = "windows"))]
fn handle_self_update(
    check: bool,
    include_prerelease: bool,
    force: bool,
    quiet: bool,
    no_sudo: bool,
) -> Result<()> {
    let agent = build_agent();
    let release = fetch_latest_release(&agent, include_prerelease)?;
    let latest_version = parse_version(&release.tag_name)?;
    let current_version = Version::parse(env!("CARGO_PKG_VERSION"))?;

    if check {
        if latest_version > current_version {
            println!("Update verfügbar: {} → {}", current_version, latest_version);
        } else {
            println!("HypnoScript ist aktuell (Version {}).", current_version);
        }
        return Ok(());
    }

    if latest_version <= current_version && !force {
        println!(
            "HypnoScript ist bereits auf dem neuesten Stand (Version {}).",
            current_version
        );
        return Ok(());
    }

    let metadata = load_install_metadata();
    let install_prefix =
        install_prefix_from_metadata(&metadata).or_else(|| derive_prefix_from_binary());

    let (installer_path, remove_after) = match find_shared_installer(metadata.as_ref()) {
        Some(path) => (path, false),
        None => (download_installer(&agent)?, true),
    };

    let mut command = Command::new("bash");
    command.arg(&installer_path);

    if let Some(prefix) = &install_prefix {
        command.arg("--prefix").arg(prefix);
    }
    if include_prerelease {
        command.arg("--include-prerelease");
    }
    if quiet {
        command.arg("--quiet");
    }
    if no_sudo {
        command.arg("--no-sudo");
    }
    if force {
        command.arg("--force");
    }

    command.stdin(Stdio::inherit());
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());

    println!("Starte Installer für Version {}...", latest_version);
    let status = command.status()?;

    if remove_after {
        let _ = fs::remove_file(&installer_path);
    }

    if !status.success() {
        return Err(anyhow!("Installer beendete sich mit Status {}", status));
    }

    println!(
        "HypnoScript wurde auf Version {} aktualisiert.",
        latest_version
    );
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn derive_prefix_from_binary() -> Option<PathBuf> {
    env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
}

#[cfg(not(target_os = "windows"))]
fn install_prefix_from_metadata(metadata: &Option<InstallMetadata>) -> Option<PathBuf> {
    metadata
        .as_ref()
        .and_then(|meta| meta.prefix.as_ref())
        .map(PathBuf::from)
}

#[cfg(not(target_os = "windows"))]
fn load_install_metadata() -> Option<InstallMetadata> {
    let exe_dir = derive_prefix_from_binary()?;
    let share_dir = exe_dir.parent()?.join("share").join("hypnoscript");
    let meta_path = share_dir.join("installation.json");
    let data = fs::read_to_string(meta_path).ok()?;
    serde_json::from_str(&data).ok()
}

#[cfg(not(target_os = "windows"))]
fn find_shared_installer(metadata: Option<&InstallMetadata>) -> Option<PathBuf> {
    if let Some(meta) = metadata {
        if let Some(prefix) = &meta.prefix {
            if let Some(root) = Path::new(prefix).parent() {
                let candidate = root.join("share").join("hypnoscript").join("install.sh");
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
    }

    derive_prefix_from_binary()
        .and_then(|prefix| {
            prefix
                .parent()
                .map(|root| root.join("share").join("hypnoscript").join("install.sh"))
        })
        .filter(|path| path.exists())
}

#[cfg(not(target_os = "windows"))]
fn download_installer(agent: &Agent) -> Result<PathBuf> {
    let response = agent.get(INSTALLER_FALLBACK_URL).call()?;
    let script = response.into_string()?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| anyhow!("Systemzeit liegt vor UNIX_Epoch: {}", err))?;

    let mut path = env::temp_dir();
    path.push(format!("hypnoscript-installer-{}.sh", timestamp.as_nanos()));
    fs::write(&path, script)?;

    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms)?;
    }

    Ok(path)
}
