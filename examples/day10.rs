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

fn main() {
    let input = aoc2017::get_input(10).unwrap();

    assert_eq!(solve(5, "3,4,1,5"), 12);

    println!("Answer: {:?}", solve(256, &input));
}
