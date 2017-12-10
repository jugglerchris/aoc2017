#![feature(conservative_impl_trait)]
extern crate aoc2017;

fn reverse_permute(size: usize, lengths: &[usize], offsets: &[usize], mut pos: usize) -> usize
{
    for i in (0..lengths.len()).rev() {
        let knotstart = offsets[i] as usize;
        let knotend = knotstart + (lengths[i] as usize);
        // If out position is before the start of the revesed range,
        // then move it up by `size` so we don't have to worry about
        // wrapping anymore.
        let adjpos = if pos < knotstart { pos + size } else { pos };
        if adjpos >= knotstart && adjpos < knotend {
            let knot_offset = adjpos - knotstart;
            pos = knotend - knot_offset - 1;
        }
    }
    pos
}

fn solve(size: usize, input: &str) -> usize {
    let lengths: Vec<usize> = input.trim().split(',').map(|s| s.parse().unwrap()).collect();
    // Precalculate the starting offsets for each step
    let offsets: Vec<usize> = {
        let mut v = Vec::new();
        let mut pos = 0;
        for skip in 0..lengths.len() {
            v.push(pos);
            pos += skip + lengths[skip];
            pos %= size;
        }
        v
    };
    // Now reverse the knotting to find out where the element at 0 came from.
    let val0 = reverse_permute(size, &lengths, &offsets, 0);
    let val1 = reverse_permute(size, &lengths, &offsets, 1);
    val0 * val1
}

struct Hasher {
    lengths: Vec<usize>,
    cur_positions: Vec<usize>,
    skip: usize,
    cur_pos: usize,
}

impl Hasher {
    pub fn new(lengths: &[usize]) -> Hasher {
        Hasher {
            lengths: lengths.into(),
            cur_positions: (0..256).collect(),
            skip: 0,
            cur_pos: 0,
        }
    }

    pub fn one_round(&mut self) {
        for &l in &self.lengths {
            let knotstart = self.cur_pos;
            let knotend = knotstart + l - 1;
            for i in 0..l/2 {
                self.cur_positions.swap((knotstart+i)&0xff, (knotend - i)&0xff);
            }
            self.cur_pos += l + self.skip;
            self.skip += 1;
        }
    }

    pub fn as_hex(&self) -> String {
        let mut result = String::new();
        for i in 0..16 {
            let mut b = 0;
            for j in 0..16 {
                b ^= self.cur_positions[i*16 + j];
            }
            result.push_str(&format!("{:02x}", b));
        }
        result
    }
}

fn solve2(input: &str) -> String {
    let lengths: Vec<usize> =
        input.as_bytes()
             .iter()
             .chain(&[17, 31, 73, 47, 23])
             .map(|&x| x as usize)
             .collect();
    let mut hasher = Hasher::new(&lengths);
    for _ in 0..64 {
        hasher.one_round();
    }
    hasher.as_hex()
}

fn main() {
    let input = aoc2017::get_input(10).unwrap();

    assert_eq!(solve(5, "3,4,1,5"), 12);

    println!("Answer: {:?}", solve(256, &input));

    assert_eq!(&solve2(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(&solve2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(&solve2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(&solve2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");

    println!("Answer 2: {}", solve2(input.trim()));
}
