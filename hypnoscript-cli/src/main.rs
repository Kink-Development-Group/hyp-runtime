mod debug_repl;
mod package;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use debug_repl::{DebugConfig, run_debug_session};
use hypnoscript_compiler::{
    Interpreter, NativeCodeGenerator, OptimizationLevel, Optimizer, TargetPlatform, TypeChecker,
    WasmBinaryGenerator, WasmCodeGenerator,
};
use hypnoscript_lexer_parser::{Lexer, Parser as HypnoParser};
use package::PackageManager;
use semver::Version;
use serde::Deserialize;
#[cfg(not(target_os = "windows"))]
use std::io::Write;
use std::process::{Command, Stdio};
use std::{env, fs, time::Duration};
use ureq::Agent;

#[cfg(not(target_os = "windows"))]
use std::path::{Path, PathBuf};

const GITHUB_OWNER: &str = "Kink-Development-Group";
const GITHUB_REPO: &str = "hyp-runtime";
const GITHUB_API: &str = "https://api.github.com";
const DEFAULT_TIMEOUT_SECS: u64 = 20;
const DEFAULT_PACKAGE_VERSION: &str = "^1.2.0";
#[cfg(not(target_os = "windows"))]
const INSTALLER_FALLBACK_URL: &str =
    "https://kink-development-group.github.io/hyp-runtime/install.sh";

#[cfg(not(target_os = "windows"))]
use tempfile::{Builder, TempPath};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn into_anyhow<E: std::fmt::Display>(error: E) -> anyhow::Error {
    anyhow::Error::msg(error.to_string())
}

/// Read a HypnoScript source file, tolerating UTF-8/UTF-16 BOMs and UTF-16
/// encodings (common for files created on Windows).
fn read_source(path: &str) -> Result<String> {
    let bytes = fs::read(path)?;
    hypnoscript_lexer_parser::decode_source(&bytes).map_err(|e| anyhow!("{}: {}", path, e))
}

#[derive(Parser)]
#[command(name = "hypnoscript")]
#[command(about = "HypnoScript - The Hypnotic Programming Language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a HypnoScript file directly
    #[command(name = "exec", alias = "execute")]
    Exec {
        /// Path to the .hyp file
        file: String,

        /// Enable debug mode (interactive debugger)
        #[arg(short, long)]
        debug: bool,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Set breakpoints at specific lines (comma-separated)
        #[arg(long, value_delimiter = ',')]
        breakpoints: Option<Vec<usize>>,

        /// Watch expressions to monitor (comma-separated)
        #[arg(long, value_delimiter = ',')]
        watch: Option<Vec<String>>,

        /// Output trace information to file
        #[arg(long)]
        trace_file: Option<String>,

        /// Restrict file builtins to this directory (filesystem sandbox).
        /// Can also be set via the HYPNO_SANDBOX environment variable.
        #[arg(long)]
        sandbox: Option<String>,

        /// Maximum function call depth before aborting with an error.
        /// Can also be set via the HYPNO_MAX_CALL_DEPTH environment variable.
        #[arg(long)]
        max_call_depth: Option<usize>,
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

        /// Generate binary WASM (.wasm) instead of text format (.wat)
        #[arg(short, long)]
        binary: bool,
    },

    /// Compile to native binary
    CompileNative {
        /// Path to the .hyp file
        input: String,

        /// Output binary file
        #[arg(short, long)]
        output: Option<String>,

        /// Target platform (windows-x64, linux-x64, macos-arm64, etc.)
        #[arg(short, long)]
        target: Option<String>,

        /// Optimization level (none, less, default, aggressive, release)
        #[arg(long, default_value = "default")]
        opt_level: String,
    },

    /// Optimize HypnoScript code
    Optimize {
        /// Path to the .hyp file
        input: String,

        /// Output file (optimized)
        #[arg(short, long)]
        output: Option<String>,

        /// Show optimization statistics
        #[arg(short, long)]
        stats: bool,
    },

    /// Update or check the HypnoScript installation
    #[command(name = "self-update", alias = "update")]
    SelfUpdate {
        /// Only check for updates, do not install
        #[arg(long)]
        check: bool,

        /// Include pre-release versions
        #[arg(long)]
        include_prerelease: bool,

        /// Force installation even if version is identical
        #[arg(long)]
        force: bool,

        /// Reduce installer output
        #[arg(long)]
        quiet: bool,

        /// Do not use sudo for the installer
        #[arg(long)]
        no_sudo: bool,
    },

    /// Initialize a new trance.json manifest
    Init {
        /// Name of the package
        #[arg(short, long)]
        name: Option<String>,

        /// Template to use (cli, library)
        #[arg(short, long)]
        template: Option<String>,
    },

    /// Install all dependencies from trance.json
    Install,

    /// Add a dependency to trance.json
    Add {
        /// Package name
        package: String,

        /// Version specification (e.g., ^1.0.0)
        #[arg(short, long)]
        version: Option<String>,

        /// Add as a development dependency
        #[arg(short = 'D', long)]
        dev: bool,
    },

    /// Remove a dependency from trance.json
    Remove {
        /// Package name
        package: String,
    },

    /// List all dependencies
    List,

    /// Run a suggestion (script) from trance.json
    Run {
        /// Name of the suggestion to run
        name: String,
    },

    /// Validate trance.json manifest
    Validate,

    /// Show version information
    Version,

    /// Show builtin functions
    Builtins,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Exec {
            file,
            debug,
            verbose,
            breakpoints,
            watch,
            trace_file,
            sandbox,
            max_call_depth,
        } => {
            if verbose {
                println!("Running file: {}", file);
            }

            if let Some(sandbox_dir) = sandbox {
                hypnoscript_runtime::set_sandbox_root(&sandbox_dir)
                    .map_err(|e| anyhow!("Invalid sandbox directory '{}': {}", sandbox_dir, e))?;
                if verbose {
                    println!("Filesystem sandbox: {}", sandbox_dir);
                }
            }

            let source = read_source(&file)?;

            // If debug mode is enabled, start interactive debug session
            if debug {
                let config = DebugConfig::new()
                    .with_breakpoints(breakpoints.unwrap_or_default())
                    .with_watches(watch.unwrap_or_default())
                    .with_verbose(verbose)
                    .with_trace_file(trace_file);

                return run_debug_session(source, config);
            }

            // Normal execution mode
            if verbose {
                println!("Source code: {} bytes", source.len());
            }

            // Lex
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(into_anyhow)?;

            if verbose {
                println!("Tokens: {}", tokens.len());
            }

            // Parse
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(into_anyhow)?;

            if verbose {
                println!("AST parsed successfully");
            }

            // Type check
            let mut type_checker = TypeChecker::new();
            let errors = type_checker.check_program(&ast);
            if !errors.is_empty() {
                eprintln!("Type errors:");
                for error in errors {
                    eprintln!("  - {}", error);
                }
                eprintln!("\nContinuing execution despite type errors...");
            }

            if verbose {
                println!("Executing program...");
            }

            // Execute
            let mut interpreter = Interpreter::new();
            if let Some(depth) = max_call_depth {
                interpreter.set_max_call_depth(depth);
            }
            interpreter.execute_program(ast).map_err(into_anyhow)?;

            if verbose {
                println!("\n✅ Program executed successfully!");
            }
        }

        Commands::Lex { file } => {
            let source = read_source(&file)?;
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(into_anyhow)?;

            println!("=== Tokens ===");
            for (i, token) in tokens.iter().enumerate() {
                println!("{:4}: {:?}", i, token);
            }
            println!("\nTotal tokens: {}", tokens.len());
        }

        Commands::Parse { file } => {
            let source = read_source(&file)?;
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(into_anyhow)?;
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(into_anyhow)?;

            println!("=== AST ===");
            println!("{:#?}", ast);
        }

        Commands::Check { file } => {
            let source = read_source(&file)?;
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
                for error in &errors {
                    println!("  - {}", error);
                }
                return Err(anyhow!(
                    "type checking failed with {} error(s)",
                    errors.len()
                ));
            }
        }

        Commands::CompileWasm {
            input,
            output,
            binary,
        } => {
            let source = read_source(&input)?;
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(into_anyhow)?;
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(into_anyhow)?;

            if binary {
                // Generate binary WASM
                let mut generator = WasmBinaryGenerator::new();
                let wasm_bytes = generator.generate(&ast).map_err(into_anyhow)?;

                let output_file = output.unwrap_or_else(|| input.replace(".hyp", ".wasm"));
                fs::write(&output_file, wasm_bytes)?;
                println!("✅ Binary WASM written to: {}", output_file);
            } else {
                // Generate text WASM (WAT)
                let mut generator = WasmCodeGenerator::new();
                let wasm_code = generator.generate(&ast);

                let output_file = output.unwrap_or_else(|| input.replace(".hyp", ".wat"));
                fs::write(&output_file, wasm_code)?;
                println!("✅ WASM text format written to: {}", output_file);
            }
        }

        Commands::CompileNative {
            input,
            output,
            target,
            opt_level,
        } => {
            let source = read_source(&input)?;
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(into_anyhow)?;
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(into_anyhow)?;

            let mut generator = NativeCodeGenerator::new();

            // Set target platform
            if let Some(target_str) = target {
                let platform = match target_str.as_str() {
                    "windows-x64" => TargetPlatform::WindowsX64,
                    "windows-arm64" => TargetPlatform::WindowsArm64,
                    "macos-x64" => TargetPlatform::MacOsX64,
                    "macos-arm64" => TargetPlatform::MacOsArm64,
                    "linux-x64" => TargetPlatform::LinuxX64,
                    "linux-arm64" => TargetPlatform::LinuxArm64,
                    "linux-riscv" => TargetPlatform::LinuxRiscV,
                    _ => return Err(anyhow!("Unsupported target platform: {}", target_str)),
                };
                generator.set_target_platform(platform);
            }

            // Set optimization level
            let opt = match opt_level.as_str() {
                "none" => OptimizationLevel::None,
                "less" => OptimizationLevel::Less,
                "default" => OptimizationLevel::Default,
                "aggressive" => OptimizationLevel::Aggressive,
                "release" => OptimizationLevel::Release,
                _ => return Err(anyhow!("Invalid optimization level: {}", opt_level)),
            };
            generator.set_optimization_level(opt);

            if let Some(out) = output {
                generator.set_output_path(out.into());
            }

            println!("🔨 Compiling to native code...");
            println!("{}", generator.target_info());

            match generator.generate(&ast) {
                Ok(path) => {
                    println!("✅ Native binary written to: {}", path.display());
                }
                Err(e) => {
                    println!("⚠️  {}", e);
                    println!(
                        "\nNote: Native code generation will be implemented in a future version."
                    );
                    println!("Use instead:");
                    println!("  - 'hypnoscript run {}' for interpretation", input);
                    println!("  - 'hypnoscript compile-wasm {}' for WebAssembly", input);
                }
            }
        }

        Commands::Optimize {
            input,
            output,
            stats,
        } => {
            let source = read_source(&input)?;
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(into_anyhow)?;
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(into_anyhow)?;

            let mut optimizer = Optimizer::new();
            optimizer.enable_all_optimizations();

            println!("🔧 Optimizing code...");
            let optimized_ast = optimizer.optimize(&ast).map_err(into_anyhow)?;

            if stats {
                let opt_stats = optimizer.stats();
                println!("\n📊 Optimization Statistics:");
                println!(
                    "  - Constant folding:     {} optimizations",
                    opt_stats.folded_constants
                );
                println!(
                    "  - Dead code elimination: {} blocks removed",
                    opt_stats.eliminated_dead_code
                );
                println!(
                    "  - CSE:                  {} eliminations",
                    opt_stats.eliminated_common_subexpr
                );
                println!(
                    "  - Loop invariants:      {} moved",
                    opt_stats.moved_loop_invariants
                );
                println!(
                    "  - Function inlining:    {} functions",
                    opt_stats.inlined_functions
                );
            }

            // For now, just report that optimization was performed
            // In a full implementation, we would regenerate source code from the optimized AST
            let output_file = output.unwrap_or_else(|| input.replace(".hyp", ".opt.hyp"));
            println!("✅ Optimized AST available (output generation not yet implemented)");
            println!("   Would write to: {}", output_file);
            println!("\nOptimized AST:\n{:#?}", optimized_ast);
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

        Commands::Init { name, template } => {
            let pm = PackageManager::new();
            let ritual_name = match name {
                Some(n) => n,
                None => {
                    // Try to get from current directory name
                    let cwd = env::current_dir()?;
                    cwd.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("my-hypno-package")
                        .to_string()
                }
            };

            pm.init(ritual_name, template.as_deref())?;
        }

        Commands::Install => {
            let pm = PackageManager::new();
            pm.install()?;
        }

        Commands::Add {
            package,
            version,
            dev,
        } => {
            let pm = PackageManager::new();
            let ver = version.unwrap_or_else(|| DEFAULT_PACKAGE_VERSION.to_string());
            pm.add_dependency(package, ver, dev)?;
        }

        Commands::Remove { package } => {
            let pm = PackageManager::new();
            pm.remove_dependency(&package)?;
        }

        Commands::List => {
            let pm = PackageManager::new();
            pm.list_dependencies()?;
        }

        Commands::Run { name } => {
            let pm = PackageManager::new();
            let command_str = pm.run_suggestion(&name)?;
            println!("📜 Running suggestion '{}': {}", name, command_str);

            // Parse and execute the command using proper shell argument parsing
            let parts = shlex::split(&command_str)
                .ok_or_else(|| anyhow!("Failed to parse command in suggestion '{}'", name))?;

            if parts.is_empty() {
                return Err(anyhow!("Empty command in suggestion '{}'", name));
            }

            let program = &parts[0];
            let args = &parts[1..];

            let status = Command::new(program)
                .args(args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .map_err(|e| anyhow!("Failed to execute command '{}': {}", command_str, e))?;

            if !status.success() {
                let code = status.code().unwrap_or(-1);
                return Err(anyhow!("Command '{}' exited with code {}", name, code));
            }
        }

        Commands::Validate => {
            let pm = PackageManager::new();
            pm.validate()?;
        }

        Commands::Version => {
            println!("HypnoScript v{}", env!("CARGO_PKG_VERSION"));
            println!("The Hypnotic Programming Language");
            println!();
            println!("Features:");
            println!("  - Full parser and interpreter");
            println!("  - Type checker");
            println!("  - WASM code generation (text & binary)");
            println!("  - Native code compilation (planned)");
            println!("  - Code optimization");
            println!("  - 180+ builtin functions");
        }

        Commands::Builtins => {
            use hypnoscript_compiler::builtin_registry;

            println!("=== HypnoScript Builtin Functions ===");
            for category in builtin_registry::categories() {
                println!("\n{}:", category);
                for sig in builtin_registry::by_category(category) {
                    println!("  {}", sig.render());
                }
            }
            println!(
                "\nTotal: {} builtin functions",
                builtin_registry::BUILTINS.len()
            );
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

#[cfg(not(target_os = "windows"))]
enum InstallerScript {
    Shared(PathBuf),
    Temporary(TempPath),
}

#[cfg(not(target_os = "windows"))]
impl InstallerScript {
    fn shared(path: PathBuf) -> Self {
        Self::Shared(path)
    }

    fn temporary(temp_path: TempPath) -> Self {
        Self::Temporary(temp_path)
    }

    fn path(&self) -> &Path {
        match self {
            InstallerScript::Shared(path) => path,
            InstallerScript::Temporary(temp_path) => temp_path.as_ref(),
        }
    }
}

fn build_agent() -> Agent {
    let timeout_secs = env::var("HYP_UPDATE_TIMEOUT")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(DEFAULT_TIMEOUT_SECS);

    Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(timeout_secs)))
        .user_agent(format!("hypnoscript-cli/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .new_agent()
}

fn github_get(agent: &Agent, url: &str) -> ureq::RequestBuilder<ureq::typestate::WithoutBody> {
    let mut request = agent
        .get(url)
        .header("Accept", "application/vnd.github+json");

    if let Ok(token) = env::var("GITHUB_TOKEN") {
        request = request
            .header("Authorization", &format!("Bearer {}", token))
            .header("X-GitHub-Api-Version", "2022-11-28");
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
            .body_mut()
            .read_json::<Vec<GithubRelease>>()?
            .into_iter()
            .filter(|release| !release.draft)
            .collect();

        releases
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No release found"))
    } else {
        let url = format!(
            "{}/repos/{}/{}/releases/latest",
            GITHUB_API, GITHUB_OWNER, GITHUB_REPO
        );
        let release: GithubRelease = github_get(agent, &url).call()?.body_mut().read_json()?;
        Ok(release)
    }
}

fn parse_version(tag: &str) -> Result<Version> {
    let normalized = tag.trim_start_matches(['v', 'V']);
    Version::parse(normalized).map_err(|err| anyhow!("Invalid version tag '{}': {}", tag, err))
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
            println!("Update available: {} → {}", current_version, latest_version);
            std::process::exit(2);
        } else {
            println!("HypnoScript is up to date (version {}).", current_version);
            std::process::exit(0);
        }
    }

    Err(anyhow!(
        "Self-update is not currently supported on Windows. Please download the latest release manually from:\n\
         https://github.com/{}/{}/releases",
        GITHUB_OWNER,
        GITHUB_REPO
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
            println!("Update available: {} → {}", current_version, latest_version);
            std::process::exit(2);
        } else {
            println!("HypnoScript is up to date (version {}).", current_version);
            std::process::exit(0);
        }
    }

    if latest_version <= current_version && !force {
        println!(
            "HypnoScript is already up to date (version {}).",
            current_version
        );
        return Ok(());
    }

    let metadata = load_install_metadata();
    // Try to determine the current installation prefix from metadata or binary location.
    // If both fail, install_prefix will be None, and the installer will use its default
    // prefix (/usr/local/bin), which is the correct fallback behavior.
    let install_prefix = install_prefix_from_metadata(&metadata).or_else(derive_prefix_from_binary);

    let installer = match find_shared_installer(metadata.as_ref()) {
        Some(path) => InstallerScript::shared(path),
        None => download_installer(&agent)?,
    };

    let mut command = Command::new("bash");
    command.arg(installer.path());

    // Only pass --prefix if we successfully determined the current installation location.
    // Otherwise, let the installer use its default prefix.
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

    println!("Starting installer for version {}...", latest_version);
    let status = command.status()?;

    if !status.success() {
        return Err(anyhow!("Installer exited with status {}", status));
    }

    println!("HypnoScript updated to version {}.", latest_version);
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
    if let Some(meta) = metadata
        && let Some(prefix) = &meta.prefix
        && let Some(root) = Path::new(prefix).parent()
    {
        let candidate = root.join("share").join("hypnoscript").join("install.sh");
        if candidate.exists() {
            return Some(candidate);
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
fn download_installer(agent: &Agent) -> Result<InstallerScript> {
    let response = agent.get(INSTALLER_FALLBACK_URL).call()?;
    let script = response.into_body().read_to_string()?;

    let mut temp_file = Builder::new()
        .prefix("hypnoscript-installer-")
        .suffix(".sh")
        .tempfile()?;
    temp_file.write_all(script.as_bytes())?;

    #[cfg(unix)]
    {
        let mut perms = temp_file.as_file().metadata()?.permissions();
        perms.set_mode(0o755);
        temp_file.as_file().set_permissions(perms)?;
    }

    let temp_path = temp_file.into_temp_path();
    Ok(InstallerScript::temporary(temp_path))
}
