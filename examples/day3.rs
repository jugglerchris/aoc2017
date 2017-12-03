#![feature(conservative_impl_trait)]
extern crate aoc2017;
use std::collections::HashMap;

// I solved this in my head originally; this is roughly the algorithm
// I used.
fn solve(input: u64) -> u64 {
    // find the next odd square up, and work out the distance from there.
    let sqrt = (input as f64).sqrt().ceil() as u64 | 1;
    let mut dist = sqrt - 1;  // distance from odd square corner
    let mut short = (sqrt * sqrt) - input;
    // Knock off any complete sides, which don't change the distance.
    if sqrt > 1 {
        short = short % (sqrt - 1);
    }
    let corner_to_middle = (sqrt - 1) / 2;
    if short > corner_to_middle {
        dist -= (sqrt-1) - short;
    } else {
        dist -= short;
    }
    dist
}

fn square_to_coord(square_num: i64) -> (i64, i64) {
    let sqrt = (square_num as f64).sqrt().floor() as i64;
    let extra = square_num - (sqrt*sqrt);

    if (sqrt & 1) == 0 {
        // Even: start at top left (just above the diagonal)
        let corner = (-sqrt/2 + 1, -sqrt/2);
        if extra == 0 {
            corner
        } else if extra <= (sqrt + 1) {
            (corner.0 - 1, corner.1 + (extra - 1))
        } else {
            (corner.0 + (extra - (sqrt + 2)), corner.1 + sqrt)
        }
    } else {
        let corner = ((sqrt - 1)/2, (sqrt - 1)/2);
        if extra == 0 {
            corner
        } else if extra <= (sqrt + 1) {
            (corner.0 + 1, corner.1 - (extra - 1))
        } else {
            (corner.0 - (extra - (sqrt + 2)), corner.1 - sqrt)
        }

    }
}

#[test]
fn test_sq_to_co() {
    assert_eq!((1..24).map(square_to_coord).collect::<Vec<_>>(),
               vec![
                   (0,0),
                   (1,0),
                   (1,-1),
                   (0,-1),
                   (-1,-1),
                   (-1,0),
                   (-1,1),
                   (0,1),
                   (1,1),
                   (2,1),
                   (2,0),
                   (2,-1),
                   (2,-2),
                   (1,-2),
                   (0,-2),
                   (-1,-2),
                   (-2,-2),
                   (-2,-1),
                   (-2,0),
                   (-2,1),
                   (-2,2),
                   (-1,2),
                   (0,2)]);
}

// Returns (square_num, val)
fn solve2(input: u64) -> (u64, u64) {
    // Location 0 is unused; we're just doing 1-based indexing.
    // Filling in locations 1,2 to reduce special cases.
    let mut memory: HashMap<(i64,i64),u64> = HashMap::new();
    memory.insert(square_to_coord(1), 1);

    let mut i = 2;
    loop {
        let coord = square_to_coord(i);
        let mut sum = 0;
        for dx in -1..2 {
            for dy in -1..2 {
                if let Some(v) = memory.get(&(coord.0 + dx, coord.1 + dy)) {
                    sum += v;
                }
            }
        }
        if sum > input {
            // found!
            return (i as u64, sum);
        }
        memory.insert(coord, sum);
        i += 1;
    }
}

fn main() {
    let input:u64 = aoc2017::get_input(3).unwrap().parse().unwrap();

    assert_eq!(solve(1), 0);
    assert_eq!(solve(12), 3);
    assert_eq!(solve(23), 2);
    assert_eq!(solve(1024), 31);

    println!("Answer part 1: {}", solve(input));

    assert_eq!(vec![1, 2, 4, 5, 10].into_iter().map(solve2).collect::<Vec<_>>(),
               vec![(3, 2), (4, 4), (5, 5), (6, 10), (7, 11)]);
//               vec![1, 1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147,
 //                   304, 330, 351, 362, 747, 806]);

    println!("Answer part 2: {:?}", solve2(input).1);
}