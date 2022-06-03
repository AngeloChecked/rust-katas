// 0
// 1
// 0 + 1 = 1
// 1 + 1 = 2
// 1 + 2 = 3
// 2 + 3 = 5
// 3 + 5 = 8

fn nth_fibonacci(position: u64) -> u64 {
    match position {
        1 => { 0 }
        2 => { 1 }
        3 => { 1 }
        _ => { nth_fibonacci(position-2) + nth_fibonacci(position-1) }
    }
}

#[cfg(test)]
mod tests {
    use super::nth_fibonacci;

    #[test]
    fn fibonacci_1_get_back_0() {
        let result = nth_fibonacci(1);
        assert_eq!(result, 0)
    }

    #[test]
    fn fibonacci_2_get_back_1() {
        let result = nth_fibonacci(2);
        assert_eq!(result, 1)
    }

    #[test]
    fn fibonacci_3_get_back_1() {
        let result = nth_fibonacci(3);
        assert_eq!(result, 1)
    }

    #[test]
    fn fibonacci_4_get_back_2() {
        let result = nth_fibonacci(4);
        assert_eq!(result, 2)
    }

    #[test]
    fn fibonacci_5_get_back_3() {
        let result = nth_fibonacci(5);
        assert_eq!(result, 3)
    }

    #[test]
    fn fibonacci_6_get_back_5() {
        let result = nth_fibonacci(6);
        assert_eq!(result, 5)
    }

    #[test]
    fn fibonacci_7_get_back_8() {
        let result = nth_fibonacci(7);
        assert_eq!(result, 8)
    }

    #[test]
    fn fibonacci_8_get_back_13() {
        let result = nth_fibonacci(8);
        assert_eq!(result, 13)
    }

    #[test]
    fn fibonacci_9_get_back_21() {
        let result = nth_fibonacci(9);
        assert_eq!(result, 21)
    }
}