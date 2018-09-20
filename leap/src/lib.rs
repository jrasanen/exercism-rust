fn divisible_by(year: i32, divisor: i32) -> bool {
    year % divisor == 0
}

pub fn is_leap_year(year: i32) -> bool {
    divisible_by(year, 400) ||
    (!divisible_by(year, 100) && divisible_by(year, 4))
}
