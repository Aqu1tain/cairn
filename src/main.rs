use std::io;
use std::path::Path;

// Import the functionality from our crate
use cairn::{bin_to_json, json_to_bin};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        print_usage(&args[0]);
        return Ok(());
    }
    
    let command = &args[1];
    let input = &args[2];
    let output = args.get(3).map(|s| s.to_string()).unwrap_or_else(|| {
        generate_default_output_path(command, input)
    });
    
    match command.as_str() {
        "bin2json" => {
            println!("Converting {} to {}", input, output);
            bin_to_json(input, output)?;
        }
        "json2bin" => {
            println!("Converting {} to {}", input, output);
            json_to_bin(input, output)?;
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage(&args[0]);
            return Ok(());
        }
    }
    
    println!("Conversion complete!");
    Ok(())
}

fn print_usage(program_name: &str) {
    eprintln!("Celeste Map Encoder/Decoder v{}", cairn::VERSION);
    eprintln!("Usage: {} <command> <input> [output]", program_name);
    eprintln!("Commands:");
    eprintln!("  bin2json <input.bin> [output.json]  - Convert binary map to JSON");
    eprintln!("  json2bin <input.json> [output.bin]  - Convert JSON to binary map");
}

fn generate_default_output_path(command: &str, input: &str) -> String {
    match command {
        "bin2json" => {
            if let Some(stem) = Path::new(input).file_stem() {
                if let Some(stem_str) = stem.to_str() {
                    format!("{}.json", stem_str)
                } else {
                    "output.json".to_string()
                }
            } else {
                "output.json".to_string()
            }
        }
        "json2bin" => {
            if let Some(stem) = Path::new(input).file_stem() {
                if let Some(stem_str) = stem.to_str() {
                    format!("{}.bin", stem_str)
                } else {
                    "output.bin".to_string()
                }
            } else {
                "output.bin".to_string()
            }
        }
        _ => "output".to_string(),
    }
}