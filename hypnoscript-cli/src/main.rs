use anyhow::Result;
use clap::{Parser, Subcommand};
use hypnoscript_lexer_parser::{Lexer, Parser as HypnoParser};
use hypnoscript_compiler::{Interpreter, TypeChecker, WasmCodeGenerator};
use std::fs;

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
    
    /// Show version information
    Version,
    
    /// Show builtin functions
    Builtins,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file, debug, verbose } => {
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
            let tokens = lexer.lex().map_err(|e| anyhow::anyhow!(e))?;
            
            if debug {
                println!("Tokens: {}", tokens.len());
            }
            
            // Parse
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(|e| anyhow::anyhow!(e))?;
            
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
            interpreter.execute_program(ast).map_err(|e| anyhow::anyhow!(e))?;
            
            if verbose {
                println!("\n✅ Program executed successfully!");
            }
        }
        
        Commands::Lex { file } => {
            let source = fs::read_to_string(&file)?;
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(|e| anyhow::anyhow!(e))?;
            
            println!("=== Tokens ===");
            for (i, token) in tokens.iter().enumerate() {
                println!("{:4}: {:?}", i, token);
            }
            println!("\nTotal tokens: {}", tokens.len());
        }
        
        Commands::Parse { file } => {
            let source = fs::read_to_string(&file)?;
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(|e| anyhow::anyhow!(e))?;
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(|e| anyhow::anyhow!(e))?;
            
            println!("=== AST ===");
            println!("{:#?}", ast);
        }
        
        Commands::Check { file } => {
            let source = fs::read_to_string(&file)?;
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(|e| anyhow::anyhow!(e))?;
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(|e| anyhow::anyhow!(e))?;
            
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
            let tokens = lexer.lex().map_err(|e| anyhow::anyhow!(e))?;
            let mut parser = HypnoParser::new(tokens);
            let ast = parser.parse_program().map_err(|e| anyhow::anyhow!(e))?;
            
            let mut generator = WasmCodeGenerator::new();
            let wasm_code = generator.generate(&ast);
            
            let output_file = output.unwrap_or_else(|| {
                input.replace(".hyp", ".wat")
            });
            
            fs::write(&output_file, wasm_code)?;
            println!("✅ WASM code written to: {}", output_file);
        }
        
        Commands::Version => {
            println!("HypnoScript v1.0.0 (Rust Edition)");
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

