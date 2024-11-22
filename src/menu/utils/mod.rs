use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

// Thank you https://stackoverflow.com/questions/59553586/how-do-i-generate-a-string-of-random-ascii-printable-characters-with-a-specific
// Didn't feel like writing it myself lol
pub fn random_string(length: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)  // From link above, this is needed in later versions
        .collect()
}

pub fn get_random_window_name(random_left_length: usize, center: &str, random_right_length: usize) -> String {
    format!("{} {} {}", random_string(random_left_length), center, random_string(random_right_length))
}

pub fn get_varied_window_size(base_size: glam::Vec2, variation_amount: f32) -> glam::Vec2 {
    let mut rng = rand::thread_rng();

    let variation_x = rng.gen_range(-variation_amount..=variation_amount);
    let variation_y = rng.gen_range(-variation_amount..=variation_amount);

    glam::Vec2::new(
        base_size.x + variation_x,
        base_size.y + variation_y
    )
}