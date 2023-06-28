use std::io::{self, Write};

pub fn get_user_input(prompt: &str, value: &str) -> io::Result<String> {
    let msg = if !value.is_empty() {
        format!(
            ", press G to generate a secure string, or press Enter to use the current value: {}",
            value
        )
    } else {
        String::from(", press G to generate a secure string")
    };

    print!("Please provide a value for {}{}: ", prompt, msg);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
