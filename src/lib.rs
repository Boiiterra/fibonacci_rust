pub fn fibonacci(n: u128) -> u128 {
    if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    } else if n == 3 {
        return 1;
    } else {
        let mut previous: u128 = 1; // 2nd
        let mut new: u128 = 1;  // 3rd
        let mut ghost: u128;
        let mut runner: u128 = 4;

        while runner < n {
            ghost = new;
            new = previous + new;
            previous = ghost;
            runner += 1;
        }

        return new + previous;
    }
}
