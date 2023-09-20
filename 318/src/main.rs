fn build_staircase(n: u32) -> u32 {
    ((-1.0 + ((8 * (n as u64) + 1) as f64).sqrt()) / 2.0).floor() as u32
}

fn main() {
    println!("> build_staircase(9)");
    println!("> {}", build_staircase(9));
    println!("");
    println!("> build_staircase(10)");
    println!("> {}", build_staircase(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps_zero() {
        assert_eq!(build_staircase(0), 0);
    }

    #[test]
    fn test_steps_simple() {
        assert_eq!(build_staircase(1), 1);
        assert_eq!(build_staircase(2), 1);
        assert_eq!(build_staircase(3), 2);
        assert_eq!(build_staircase(5), 2);
        assert_eq!(build_staircase(6), 3);
        assert_eq!(build_staircase(7), 3);
        assert_eq!(build_staircase(8), 3);
        assert_eq!(build_staircase(9), 3);
        assert_eq!(build_staircase(10), 4);
        assert_eq!(build_staircase(11), 4);
    }

    #[test]
    fn test_steps_large() {
        assert_eq!(build_staircase(1_000_000), 1413);
        assert_eq!(build_staircase(1_000_000_000), 44720);
        assert_eq!(build_staircase(u32::MAX), 92681);
    }
}
