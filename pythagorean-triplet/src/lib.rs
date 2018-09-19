pub fn find() -> Option<u32> {
    let target: i32 = 1000;

    for a in 1..=target/3 {
        for b in (a+1)..=target/2 {
            let c: i32 = target-a-b;
            if a*a + b*b == c*c && a + b + c == 1000 {
                return Some((a*b*c) as u32);
            }
        }
    }

    return None;
}
