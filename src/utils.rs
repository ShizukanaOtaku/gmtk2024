pub fn nearest_lower_multiple(n1: i32, n2: i32) -> i32 {
    if n2 == 0 {
        return n1; // If n2 is 0, return n1 as a fallback.
    }
    let remainder = n1 % n2;
    if remainder == 0 {
        return n1; // n1 is already a multiple of n2.
    }
    n1 - remainder
}

