#![feature(conservative_impl_trait)]
extern crate aoc2017;

use aoc2017::Hasher;

fn solve(input: &str) -> usize {
    let mut bits_set = 0;
    for i in 0..128 {
        let mut hasher = Hasher::new_from_string(&format!("{}-{}", input, i));
        for _ in 0..64 {
            hasher.one_round();
        }
        bits_set += hasher.bits_set();
    }
    bits_set
}

fn clear_group(disk: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    disk[x][y] = false;
    if x > 0 && disk[x-1][y] { clear_group(disk, x-1, y); }
    if x < 127 && disk[x+1][y] { clear_group(disk, x+1, y); }
    if y > 0 && disk[x][y-1] { clear_group(disk, x, y-1); }
    if y < 127 && disk[x][y+1] { clear_group(disk, x, y+1); }
}

fn solve2(input: &str) -> usize {
    let mut disk: Vec<Vec<bool>> =
        (0..128).map(|i| {
            let mut hasher = Hasher::new_from_string(&format!("{}-{}", input, i));
            for _ in 0..64 {
                hasher.one_round();
            }
            hasher.into_bool_vec()
        }).collect();

    let mut groups = 0;
    for x in 0..128 {
        for y in 0..128 {
            if disk[x][y] {
                clear_group(&mut disk, x,y);
                groups += 1;
            }
        }
    }
    groups
}

fn main() {
    let input: String = aoc2017::get_input(14).unwrap().trim().into();

    assert_eq!(solve("flqrgnkx"), 8108);

    println!("Answer: {:?}", solve(&input));

    assert_eq!(solve2("flqrgnkx"), 1242);

    println!("Answer: {:?}", solve2(&input));
}
