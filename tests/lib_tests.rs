use calc_sim::SumCurrent;

#[cfg(test)]
mod lib_test_arithmatic {
    use super::*;

    #[test]
    fn check_sum() {
        let mut sum = SumCurrent::new_from(5.0);
        assert_eq!(sum.sum, 5.0);
    }

    #[test]
    fn check_add() {
        let mut sum = SumCurrent::new_from(5.0);
        assert_eq!(sum.sum, 5.0);

        let sum1 = sum.add(2);
        assert_eq!(sum1.sum, 7.0);

        let sum2 = sum.add(12.0);
        assert_eq!(sum2.sum, 17.0);

        let sum3 = sum.add(24.0 as usize);
        assert_eq!(sum3.sum, 29.0);
    }

    #[test]
    fn check_subtract() {
        let mut sum = SumCurrent::new_from(120.0);

        let sum1 = sum.subtract(2);
        assert_eq!(sum1.sum, 118.0);

        let sum2 = sum.subtract(12.0);
        assert_eq!(sum2.sum, 108.0);

        let sum3 = sum.subtract(24 as usize);
        assert_eq!(sum3.sum, 96.0);
    }

    #[test]
    fn check_multiply() {
        let mut sum = SumCurrent::new_from(120.0);
        
        let sum1 = sum.multiply(2);
        assert_eq!(sum1.sum, 240.0);
        
        let sum2 = sum.multiply(12.0);
        assert_eq!(sum2.sum, 1440.0);
        
        let sum3 = sum.multiply(24 as usize);
        assert_eq!(sum3.sum, 2880.0);
    }

    #[test]
    fn check_divide() {
        let mut sum = SumCurrent::new_from(120.0);
        
        let sum1 = sum.divide(2);
        assert_eq!(sum1.sum, 60.0);
        
        let sum2 = sum.divide(12.0);
        assert_eq!(sum2.sum, 10.0);
        
        let sum3 = sum.divide(24 as usize);
        assert_eq!(sum3.sum, 5.0);
    }
}