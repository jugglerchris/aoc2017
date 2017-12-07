#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;
#[macro_use] extern crate lazy_static;

#[derive(Debug,Clone, Eq, PartialEq)]
struct TowerInfo {
    name: String,
    weight: usize,
    children: Vec<String>,
}

regex_parser!(parse_tinfo: TowerInfo {
    TOWER = r#"^(\w+) \((\d+)\) -> ([\w, ]*)$"# =>
        | name: String, weight: usize, childstring: String|
            TowerInfo {
                name: name,
                weight: weight,
                children: childstring.split(',')
                                     .map(|s| s.trim())
                                     .map(String::from)
                                     .collect::<Vec<String>>()
            },
    TOP_TOWER = r#"^(\w+) \((\d+)\)$"# => | name: String, weight: usize| TowerInfo {
        name: name, weight: weight, children: Vec::new()
    }
});

#[test]
fn test_parse() {
    assert_eq!(parse_tinfo("blah (123)"), TowerInfo {
        name: "blah".into(),
        weight: 123,
        children: Vec::new()
    });
    assert_eq!(parse_tinfo("blah (123) -> foo, bar, baz"), TowerInfo {
        name: "blah".into(),
        weight: 123,
        children: vec!["foo".into(), "bar".into(), "baz".into()]
    });
}