pub fn format_usize(n: usize) -> String {
    if n >= 1_000_000 {
        // Format as millions (Mega)
        format!("{:.0}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        // Format as thousands (Kilo)
        format!("{:.0}K", n as f64 / 1_000.0)
    } else {
        // Print as-is for smaller numbers
        n.to_string()
    }
}
