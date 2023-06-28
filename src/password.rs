use std::iter;
use std::time::SystemTime;

pub fn process_input(_var: &str, value: &str, input: String) -> String {
    if input.trim().eq_ignore_ascii_case("G") {
        generate_secure_string()
    } else if input.trim().is_empty() {
        String::from(value)
    } else {
        input.trim().to_string()
    }
}

fn generate_secure_string() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const PASSWORD_LEN: usize = 15;

    let mut rng = XorShiftRng::from_system_time();
    let password: String = iter::repeat(())
        .map(|()| {
            let idx = rng.next_u32() as usize % CHARSET.len();
            CHARSET[idx] as char
        })
        .take(PASSWORD_LEN)
        .collect();
    password
}

struct XorShiftRng {
    seed: u64,
}

impl XorShiftRng {
    fn from_system_time() -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!");

        Self {
            seed: now.as_secs(),
        }
    }

    fn next_u32(&mut self) -> u32 {
        let x = self.seed;
        self.seed = x ^ (x << 13);
        self.seed = self.seed ^ (self.seed >> 17);
        self.seed = self.seed ^ (self.seed << 5);
        self.seed as u32
    }
}
