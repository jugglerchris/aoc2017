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
    let moves = parse_moves(input);

    let (pos_perm, name_perm) = get_permutations(&moves, num_dancers);
    pos_perm.iter()
            .map(|off| (name_perm[*off] as u8 + b'a') as char)
            .collect()
}

fn compose(perm1: &[usize], perm2: &[usize]) -> Vec<usize>
{
    perm2.iter()
         .map(|&off| perm1[off])
         .collect()
}

fn compose_name(perm1: &[usize], perm2: &[usize]) -> Vec<usize> {
    // After perm1, dancer 0 ends up in perm1[0]
    //              dancer 1 ends up in perm1[1]
    //              ...
    // 
    // After perm2, dancer N ends up where perm2[N] ended up above:
    //              => perm1[perm2[N]]
    (0..(perm1.len()))
        .map(|n| perm1[perm2[n]])
        .collect()
}

fn solve2(input: &str, num_dancers: usize, mut iterations: usize) -> String {
    let moves = parse_moves(input);

    let (mut pos_perm, mut name_perm) = get_permutations(&moves, num_dancers);

    let mut final_pos_perm: Vec<usize> = (0..num_dancers).collect();
    let mut final_name_perm = final_pos_perm.clone();

    while iterations != 0 {
        if (iterations & 1) != 0 {
            final_pos_perm = compose(&final_pos_perm, &pos_perm);
            final_name_perm = compose_name(&final_name_perm, &name_perm);
        }
        pos_perm = compose(&pos_perm, &pos_perm);
        name_perm = compose(&name_perm, &name_perm);
        iterations = iterations >> 1;
    }

    final_pos_perm.iter()
                   .map(|off| (final_name_perm[*off] as u8 + b'a') as char)
                   .collect()
}

fn main() {
    let input = aoc2017::get_input(16).unwrap();

    assert_eq!(&solve("s1,x3/4,pe/b", 5), "baedc");

    println!("Answer: {:?}", solve(&input, 16));

    assert_eq!(&solve2("s1,x3/4,pe/b", 5, 2), "ceadb");

    println!("Answer: {:?}", solve2(&input, 16, 1000000000));
}
