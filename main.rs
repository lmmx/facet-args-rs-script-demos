#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! facet = { version = "0.*", default-features = false, features = [] }
//! facet-args = { version = "0.*", default-features = false, features = [] }
//! facet-pretty = { version = "0.*", default-features = false, features = [] }
//! ```

use facet::Facet;
use facet_pretty::FacetPretty;
use std::{env, process};

#[derive(Facet)]
struct HelloArgs {
    // A positional argument for the name to greet
    #[facet(positional)]
    name: String,
    
    // An optional verbose flag
    #[facet(named, short = 'v')]
    verbose: bool,
    
    // Add help flag
    #[facet(named, short = 'h')]
    help: bool,
}

fn print_help() {
    println!("Usage: {} [OPTIONS] NAME", env::args().next().unwrap_or_else(|| "program".to_string()));
    println!();
    println!("Arguments:");
    println!("  NAME                    The name to greet");
    println!();
    println!("Options:");
    println!("  -h, --help              Show this help message and exit");
    println!("  -v, --verbose           Enable verbose output");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect the arguments first so they stay in scope
    let string_args: Vec<String> = std::env::args().skip(1).collect();
    
    // Check for help flag first
    if string_args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
        process::exit(0);
    }
    
    // Now create a vector of string slices that reference the owned strings
    let arg_refs: Vec<&str> = string_args.iter().map(|s| s.as_str()).collect();
    
    // Parse command line arguments
    let args_result: Result<HelloArgs, _> = facet_args::from_slice(&arg_refs);
    
    match args_result {
        Ok(args) => {
            // If verbose mode is on, print the args structure
            if args.verbose {
                eprintln!("args: {}", args.pretty());
            }
            
            // Print the hello message
            println!("Hello, {}!", args.name);
            Ok(())
        },
        Err(e) => {
            // Check if the error is about missing the name field
            if e.to_string().contains("'HelloArgs::name' was not initialized") {
                eprintln!("Error: No name provided!");
                eprintln!();
                print_help();
                process::exit(1);
            } else {
                // For other errors, propagate them
                Err(e.into())
            }
        }
    }
}
