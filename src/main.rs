use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

mod input;
mod password;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run -- <source_file> <output_file>");
        return Ok(());
    }

    let source_file = &args[1];
    let output_file = &args[2];

    let file = fs::File::open(source_file)?;
    let reader = io::BufReader::new(file);

    let mut variables = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                let var = String::from(parts[0]);
                let value = String::from(parts[1]);

                let final_value = input::get_user_input(&var, &value)?;
                let final_value = password::process_input(&var, &value, final_value);

                variables.push((var.clone(), final_value.clone()));
                println!("Variable: {}", var);
                println!("Value: {}", final_value);
                println!();
            }
        }
    }

    let output = fs::File::create(output_file)?;
    let mut writer = io::BufWriter::new(output);

    for (var, value) in variables {
        writeln!(writer, "{}={}", var, value)?;
    }

    println!("{} file has been generated", output_file);

    Ok(())
}
