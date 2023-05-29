pub fn abbrev_name(name: &str) -> String {
    let names = name.split_whitespace().collect::<Vec<&str>>();
    format!(
        "{}.{}",
        names[0].get(0..1).unwrap(),
        names[1].get(0..1).unwrap()
    )
    .to_uppercase()
}

pub fn abbrev_name_2(name: &str) -> String {
    name.split(' ')
        .map(|x| x.chars().next().unwrap().to_string().to_uppercase())
        .collect::<Vec<_>>()
        .join(".")
}

#[test]
fn returns_expected() {
    assert_eq!(abbrev_name("Sam Harris"), "S.H");
    assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
    assert_eq!(abbrev_name("Evan Cole"), "E.C");
    assert_eq!(abbrev_name("P Favuzzi"), "P.F");
    assert_eq!(abbrev_name("David Mendieta"), "D.M");
}

#[test]
fn returns_expected2() {
    assert_eq!(abbrev_name_2("Sam Harris"), "S.H");
    assert_eq!(abbrev_name_2("Patrick Feenan"), "P.F");
    assert_eq!(abbrev_name_2("Evan Cole"), "E.C");
    assert_eq!(abbrev_name_2("P Favuzzi"), "P.F");
    assert_eq!(abbrev_name_2("David Mendieta"), "D.M");
}
