pub fn dna_strand(dna: &str) -> String {
    dna.chars()
        .map(|c| match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => panic!("Invalid DNA"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(dna_strand("AAAA"), "TTTT");
        assert_eq!(dna_strand("ATTGC"), "TAACG");
        assert_eq!(dna_strand("GTAT"), "CATA");
    }
}
