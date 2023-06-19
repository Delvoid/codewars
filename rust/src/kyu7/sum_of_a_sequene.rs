pub fn sequence_sum(start: u32, end: u32, step: u32) -> u32 {
    if start > end {
        return 0;
    }

    (start..=end).step_by(step as usize).sum()
}
