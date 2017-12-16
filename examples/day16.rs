#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;
#[macro_use] extern crate lazy_static;

#[derive(Copy,Clone,Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}
use Move::*;

regex_parser!(parse_move: Move {
    SPIN = r#"^s(\d+)$"# => | x: usize | Spin(x),
    EXCHANGE = r#"^x(\d+)/(\d+)$"# => | a: usize, b: usize | Exchange(a,b),
    PARTNER = r#"^p(.)/(.)$"# => | a: char, b: char | Partner(a as u8 - b'a', b as u8 - b'a')
});

fn parse_moves(input: &str) -> Vec<Move> {
    input.trim()
         .split(',')
         .map(parse_move)
         .collect()
}

// Return the permutations for a dance (vector of positions after the dance)
// Returns the position permutation and the "renaming" permutation.
fn get_permutations(moves: &[Move], num_dancers: usize) -> (Vec<usize>, Vec<usize>)
{
    let mut dancers: Vec<usize> = (0..num_dancers).collect();
    // positions is a mapping from dancer to position in dancers
    let mut positions: Vec<usize> = (0..num_dancers).collect();
    // Track what dancers have been renamed (swapped by name)
    let mut renames: Vec<usize> = (0..num_dancers).collect();
    let mut offset = 0usize;  // The current first position

    for mv in moves
    {
        //println!("({:?}, {:?}) + {:?} =>", dancers, offset, mv);
        match *mv {
            Spin(u) => {
                offset = (offset + num_dancers - u) % num_dancers;
            },
            Exchange(a, b) => {
                let pos1 = (a + offset) % num_dancers;
                let pos2 = (b + offset) % num_dancers;
                dancers.swap(pos1, pos2);
                // Update the reverse mapping
                let dancer1 = dancers[pos1];
                let dancer2 = dancers[pos2];
                positions.swap(dancer1, dancer2);
            },
            Partner(a, b) => {
                let pos1 = renames.iter().position(|&x| x == a as usize).unwrap();
                let pos2 = renames.iter().position(|&x| x == b as usize).unwrap();
                renames.swap(pos1, pos2);
            },
        }
        //println!("    ({:?}, {:?}) =>", dancers, offset);
    }
    ((&dancers[offset..num_dancers]).iter()
                                    .chain(&dancers[0..offset])
                                    .cloned()
                                    .collect(),
     renames)
}

fn solve(input: &str, num_dancers: usize) -> String {
    let mut dancers: Vec<usize> = (0..num_dancers).collect();
    // positions is a mapping from dancer to position in dancers
    let mut positions: Vec<usize> = (0..num_dancers).collect();
    let mut offset = 0usize;  // The current first position
    let moves = parse_moves(input);

    let (pos_perm, name_perm) = get_permutations(&moves, num_dancers);
    pos_perm.iter()
            .map(|off| (name_perm[*off] as u8 + b'a') as char)
            .collect()
}

fn main() {
    let input = aoc2017::get_input(16).unwrap();

    assert_eq!(&solve("s1,x3/4,pe/b", 5), "baedc");

    println!("Answer: {:?}", solve(&input, 16));
}
