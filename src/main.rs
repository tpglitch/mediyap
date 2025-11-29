use mediyap::MedicalDecoder;
use std::env;
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let decoder = MedicalDecoder::new();

    if args.len() > 1 {
        // Decode terms from command line arguments
        for term in &args[1..] {
            let decoded = decoder.decode(term);
            println!("{}: {}", term, decoded);
        }
    } else {
        // Interactive mode
        println!("MediYap - Interactive Mode");
        println!("Enter medical terms to decode (Ctrl+D or Ctrl+C to exit):");
        println!();

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            match line {
                Ok(term) => {
                    let term = term.trim();
                    if term.is_empty() {
                        continue;
                    }
                    let decoded = decoder.decode(term);
                    println!("→ {}", decoded);
                    println!();
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            }
            print!("> ");
            stdout.flush().unwrap();
        }
    }
}
