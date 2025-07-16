use base62::encode;
use rand::random;

pub fn generate_code() -> String {
    let random: u64 = random();
    encode(random)
}