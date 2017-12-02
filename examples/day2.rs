#![feature(conservative_impl_trait)]
extern crate aoc2017;

fn solve(data: &[Vec<u64>]) -> u64 {
    data.iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

fn find_divisible(row: &Vec<u64>) -> u64 {
    for v in row {
        for v2 in row {
            if v == v2 {
                continue
            } else if v > v2 {
                if v % v2 == 0 {
                    return v / v2
                }
            } else {
                if v2 % v == 0 {
                    return v2 / v
                }
            }
        }
    }
    panic!()
}

fn solve2(data: &[Vec<u64>]) -> u64 {
    data.iter()
        .map(find_divisible)
        .sum()
}

fn main() {
    let input = aoc2017::get_input(2).unwrap();
    let data = aoc2017::parse_rows::<u64>(&input);

    assert_eq!(solve(&aoc2017::parse_rows("5 1 9 5\n7 5 3\n2 4 6 8")), 18);

    println!("Answer part 1: {}", solve(&data));

    assert_eq!(solve2(&aoc2017::parse_rows("5 9 2 8\n9 4 7 3\n3 8 6 5\n")), 9);

    println!("Answer part 2: {}", solve2(&data));
}