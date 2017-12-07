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
}