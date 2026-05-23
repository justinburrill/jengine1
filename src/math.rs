
pub fn sqrt(num:isize) -> usize {
    if num < 0 {
        panic!("can't sqrt an negative")
    }
    for i in 1..num {
        if i*i >= num {
            return i as usize
        }
    };
    panic!("failed to find sqrt of {}", num)
}
