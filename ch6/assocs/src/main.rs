fn get<'m>(mapping: &'m [(String, String)], key: &str) -> Option<&'m str> {
    mapping
        .iter()
        .find_map(|(k, v)| if key == k { Some(v.as_str()) } else { None })
}

fn main() {
    let mapping: Vec<(String, String)> = vec![
        ("foo".to_owned(), "bar".to_owned()),
        ("baz".to_owned(), "baz".to_owned()),
    ];

    println!("{:?}", get(&mapping, "foo"));
    println!("{:?}", get(&mapping, "bar"));
    println!("{:?}", get(&mapping, "baz"));
}
