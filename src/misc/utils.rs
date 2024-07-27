use rand::{distributions::Alphanumeric, prelude::*};

pub fn generate_token(length: usize) -> String {
    let mut rng = thread_rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect::<String>()
}
