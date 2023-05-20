pub fn to_leet_speak(s: &str) -> String {
    let mut leet = String::new();
    for c in s.chars() {
        match c {
            'A' => leet.push('@'),
            'B' => leet.push('8'),
            'C' => leet.push('('),
            'D' => leet.push('D'),
            'E' => leet.push('3'),
            'G' => leet.push('6'),
            'H' => leet.push('#'),
            'I' => leet.push('!'),
            'L' => leet.push('1'),
            'O' => leet.push('0'),
            'S' => leet.push('$'),
            'T' => leet.push('7'),
            'Z' => leet.push('2'),
            ' ' => leet.push(' '),
            _ => leet.push(c),
        }
    }
    leet
}

pub fn to_leet_speak_2(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A' => '@',
            'B' => '8',
            'C' => '(',
            'D' => 'D',
            'E' => '3',
            'G' => '6',
            'H' => '#',
            'I' => '!',
            'L' => '1',
            'O' => '0',
            'S' => '$',
            'T' => '7',
            'Z' => '2',
            ' ' => ' ',
            _ => c,
        })
        .collect()
}
