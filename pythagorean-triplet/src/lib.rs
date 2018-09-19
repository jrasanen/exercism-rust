pub fn find() -> Option<u32> {
    let target: u32 = 1000;

    for a in 1..=target/3 {
        for b in (a+1)..=target/2 {
            let c: u32 = target-a-b;
            if a*a + b*b == c*c && a + b + c == 1000 {
                return Some(a*b*c);
            }
        }
    }

    return None;
}
