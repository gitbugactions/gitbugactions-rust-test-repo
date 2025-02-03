pub struct AppMath;

impl AppMath {
    pub fn new() -> Self {
        AppMath
    }

    pub fn sum(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn sum_twice(&self, a: i32, b: i32) -> i32 {
        a + b + b
    }

    pub fn subtract_twice(&self, a: i32, b: i32) -> i32 {
        a - b - b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let app_math = AppMath::new();
        assert_eq!(app_math.sum(1, 2), 3);
    }

    #[test]
    fn test_subtract() {
        let app_math = AppMath::new();
        assert_eq!(app_math.subtract(2, 1), 1);
    }

    #[test]
    fn test_multiply() {
        let app_math = AppMath::new();
        assert_eq!(app_math.multiply(2, 2), 4);
    }

    #[test]
    fn test_sum_twice() {
        let app_math = AppMath::new();
        assert_eq!(app_math.sum_twice(2, 2), 6);
    }

    #[test]
    fn test_subtract_twice() {
        let app_math = AppMath::new();
        assert_eq!(app_math.subtract_twice(6, 2), 2);
    }
}
