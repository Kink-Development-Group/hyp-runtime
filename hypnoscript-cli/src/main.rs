use anyhow::Result;
use clap::{Parser, Subcommand};
use hypnoscript_lexer_parser::Lexer;
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
            
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().map_err(|e| anyhow::anyhow!(e))?;
            
            if debug || verbose {
                println!("Tokens: {} total", tokens.len());
                for token in &tokens {
                    println!("{:?}", token);
                }
            }
            
            println!("\n✅ File processed successfully!");
            println!("Note: Full interpreter implementation pending.");
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
        
        Commands::Version => {
            println!("HypnoScript v1.0.0 (Rust Edition)");
            println!("The Hypnotic Programming Language");
            println!();
            println!("Migrated from C# to Rust for improved performance");
        }
        
        Commands::Builtins => {
            println!("=== HypnoScript Builtin Functions ===\n");
            
            println!("📊 Math Builtins:");
            println!("  - sin, cos, tan, sqrt, pow, log, log10");
            println!("  - abs, floor, ceil, round, min, max");
            println!("  - factorial, gcd, lcm, is_prime, fibonacci");
            println!("  - clamp");
            
            println!("\n📝 String Builtins:");
            println!("  - length, to_upper, to_lower, trim");
            println!("  - index_of, replace, reverse, capitalize");
            println!("  - starts_with, ends_with, contains");
            println!("  - split, substring, repeat");
            println!("  - pad_left, pad_right");
            
            println!("\n📦 Array Builtins:");
            println!("  - length, is_empty, get, index_of, contains");
            println!("  - reverse, sum, average, min, max, sort");
            println!("  - first, last, take, skip, slice");
            println!("  - join, count, distinct");
            
            println!("\n✨ Hypnotic Builtins:");
            println!("  - observe (output)");
            println!("  - drift (sleep)");
            println!("  - deep_trance");
            println!("  - hypnotic_countdown");
            println!("  - trance_induction");
            println!("  - hypnotic_visualization");
            
            println!("\n🔄 Conversion Functions:");
            println!("  - to_int, to_double, to_string, to_boolean");
            
            println!("\nTotal: 50+ builtin functions implemented");
            println!("Note: Full 150+ builtin library migration in progress");
        }
    }

    Ok(())
}

