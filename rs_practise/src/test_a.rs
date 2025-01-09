pub fn test_a() {
    println!("test_a");
}


fn clamp(n: i32, min: i32, max: i32) -> i32 {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_lower() {
        let result = clamp(1, 0, 10);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_clamp_upper() {
        let result = clamp(10, 0, 10);
        assert_eq!(result, 10);
    }
}
