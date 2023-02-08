pub fn solution(s: &str) -> Vec<String> {
    let mut i = 0;
    let mut result = Vec::new();
    let mut s = s.to_string();
    if s.len() % 2 != 0 {
        s.push('_');
    }
    while i < s.len() {
        result.push(s[i..i + 2].to_string());
        i += 2;
    }
    result
}

// * 'abc' =>  ['ab', 'c_']
// * 'abcdef' => ['ab', 'cd', 'ef']

pub fn solution2(s: &str) -> Vec<String> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|v| {
            if v.len() == 1 {
                format!("{}_", v[0])
            } else {
                v.iter().collect()
            }
        })
        .collect()
}
