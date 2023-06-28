#[cfg(test)]
mod tests {
    use crate::input;
    use crate::password;
    use std::io::{self, BufRead, Read};

    struct MockInput<'a> {
        lines: &'a [&'a str],
    }

    impl<'a> Read for MockInput<'a> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let line = match self.lines.first() {
                Some(&line) => line.as_bytes(),
                None => return Ok(0),
            };

            let len = line.len().min(buf.len());
            buf[..len].copy_from_slice(&line[..len]);

            if len == line.len() {
                self.lines = &self.lines[1..];
            }

            Ok(len)
        }
    }

    impl<'a> BufRead for MockInput<'a> {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(b"")
        }

        fn consume(&mut self, _amt: usize) {}

        fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
            if let Some(line) = self.lines.first() {
                buf.push_str(line);
                self.lines = &self.lines[1..];
                Ok(line.len())
            } else {
                Ok(0)
            }
        }
    }

    #[test]
    fn test_get_user_input() {
        let _input = MockInput { lines: &["test\n"] };
        let prompt = "Input Prompt";
        let value = "Default Value";
        let result = input::get_user_input(prompt, value).unwrap();
        assert_eq!(result, "test");
    }

    #[test]
    fn test_generate_secure_string() {
        let password = password::generate_secure_string();
        assert_eq!(password.len(), 15);
        assert!(password.chars().all(|c| c.is_ascii_alphanumeric()));
    }
}
