pub fn ease_in_out_formula(time: f64, start_value: f64, change: f64, duration: f64) -> f64 {
    // uses ease-in-out formula

    let time = time / (duration / 2.0);

    if time < 1.0 {
        return change / 2.0 * time * time + start_value;
    }

    let time = time - 1.0;
    return -change / 2.0 * (time * (time - 2.0) - 1.0) + start_value;
}