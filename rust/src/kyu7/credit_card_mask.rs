pub fn maskify(cc: &str) -> String {
    let len = cc.chars().count();
    cc.chars()
        .enumerate()
        .map(|(i, c)| if len > 4 && i < len - 4 { '#' } else { c })
        .collect()
}

pub fn maskify2(cc: &str) -> String {
    if cc.len() < 4 {
        return cc.to_string();
    }

    "#".repeat(cc.len() - 4) + &cc[cc.len() - 4..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maskify() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify("11111"), "#1111");
    }
}
