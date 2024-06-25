use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn generate_container_id() -> String {
    generate_random_string(22)
}

pub fn generate_element_id() -> String {
    generate_random_string(8)
}


fn generate_random_string(length: usize) -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    rand_string
}