#![feature(conservative_impl_trait)]
extern crate aoc2017;

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

fn solve2(input: u64) -> u64 {
    // Location 0 is unused; we're just doing 1-based indexing.
    // Filling in locations 1,2 to reduce special cases.
    let mut memory = vec![0u64, 1];

    // Keep track of when the next new corner (when we shoot past
    // what's already there) is.
    let mut next_corner = 2;
    let mut corner_inc = 1;
    let mut corners_until_longer_side = 2;
    /*
    for i in 2..input+1 {
        // Add the previous adjacent values.
        let mut sum = memory[i-1];
        if i == next_corner {
            // No more adjacent to add, but update the corner tracking.
            next_corner += corner_inc;
            if corners_until_longer_side > 0 {
                corners_until_longer_side -= 1;
            } else {
                corners_until_longer_side = 4;
                corner_inc += 1;
            }
        } else {
            // 
        }
    }
    */
    0
}

fn main() {
    let input:u64 = aoc2017::get_input(3).unwrap().parse().unwrap();

    assert_eq!(solve(1), 0);
    assert_eq!(solve(12), 3);
    assert_eq!(solve(23), 2);
    assert_eq!(solve(1024), 31);

    println!("Answer part 1: {}", solve(input));

    solve2(5);
    /*
    assert_eq!(solve2(&aoc2017::parse_rows("5 9 2 8\n9 4 7 3\n3 8 6 5\n")), 9);

    println!("Answer part 2: {}", solve2(&data));
    */
}