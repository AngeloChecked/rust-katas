pub fn fizz_buzz(num: u64) -> String {
    match num {
       num if num % 15 == 0 => { "fizzbuzz".to_string() }
       num if num % 5 == 0 => { "buzz".to_string() }
       num if num % 3 == 0 => { "fizz".to_string() }
       _ => num.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::fizz_buzz;

    #[test]
    fn when_fizzbuzz_1_i_get_back_1() {
        let result = fizz_buzz(1);
        assert_eq!(result, "1");
    }

    #[test]
    fn when_fizzbuzz_2_i_get_back_2() {
        let result = fizz_buzz(2);
        assert_eq!(result, "2");
    }

    #[test]
    fn when_fizzbuzz_4_i_get_back_4() {
        let result = fizz_buzz(4);
        assert_eq!(result, "4")
    }

    #[test]
    fn when_fizzbuzz_multiple_of_3_i_get_back_fizz() {
        let result = fizz_buzz(3);
        assert_eq!(result, "fizz");
    }

    #[test]
    fn when_fizzbuzz_multiple_of_6_i_get_back_fizz() {
        let result = fizz_buzz(6);
        assert_eq!(result, "fizz");
    }

    #[test]
    fn when_fizzbuzz_multiple_of_9_i_get_back_fizz() {
        let result = fizz_buzz(9);
        assert_eq!(result, "fizz");
    }

    #[test]
    fn when_fizzbuzz_number_5_i_get_back_buzz() {
        let result = fizz_buzz(5);

        assert_eq!(result, "buzz");
    }

    #[test]
    fn when_fizzbuzz_number_10_i_get_back_buzz() {
        let result = fizz_buzz(10);

        assert_eq!(result, "buzz");
    }

    #[test]
    fn when_fizzbuzz_number_20_i_get_back_buzz() {
        let result = fizz_buzz(20);

        assert_eq!(result, "buzz");
    }

    #[test]
    fn when_fizzbuzz_number_15_i_get_back_fizzbuzz() {
        let result = fizz_buzz(15);

        assert_eq!(result, "fizzbuzz");
    }

    #[test]
    fn when_fizzbuzz_number_30_i_get_back_fizzbuzz() {
        let result = fizz_buzz(30);

        assert_eq!(result, "fizzbuzz");
    }

    #[test]
    fn when_fizzbuzz_a_number_not_multiple_of_3_or_5_i_get_back_itself() {
        let result = fizz_buzz(1);
        let result1 = fizz_buzz(2);
        let result2 = fizz_buzz(4);
        let result3 = fizz_buzz(7);

        assert_eq!(result, "1");
        assert_eq!(result1, "2");
        assert_eq!(result2, "4");
        assert_eq!(result3, "7");
    }

    #[test]
    fn when_fizzbuzz_number_multiple_of_3_i_get_back_fizz() {
        let result = fizz_buzz(3);
        let result1 = fizz_buzz(6);
        let result2 = fizz_buzz(9);
        let result3 = fizz_buzz(12);
        let result4 = fizz_buzz(18);

        assert_eq!(result, "fizz");
        assert_eq!(result1, "fizz");
        assert_eq!(result2, "fizz");
        assert_eq!(result3, "fizz");
        assert_eq!(result4, "fizz");
    }

    #[test]
    fn when_fizzbuzz_number_both_multiple_of_3_and_5_i_get_back_fizzbuzz() {
        let result = fizz_buzz(15);
        let result1 = fizz_buzz(30);
        let result2 = fizz_buzz(45);
        let result3 = fizz_buzz(60);
        let result4 = fizz_buzz(75);

        assert_eq!(result, "fizzbuzz");
        assert_eq!(result1, "fizzbuzz");
        assert_eq!(result2, "fizzbuzz");
        assert_eq!(result3, "fizzbuzz");
        assert_eq!(result4, "fizzbuzz");
    }
}