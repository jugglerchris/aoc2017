#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;
#[macro_use] extern crate lazy_static;

fn parse_map(input: &str) -> Vec<Vec<u8>> {
    input.lines()
         .map(str::as_bytes)
         .map(From::from)
         .collect()
}

fn solve1(input: &str) -> (String, usize) {
    let map = parse_map(input);
    let width = map[0].len() as isize;
    let height = map.len() as isize;

    let mut y = 0isize;
    let mut x = map[0].iter().position(|&b| b == b'|').unwrap() as isize;
    let mut dx = 0isize;
    let mut dy = 1isize;
    let mut letters = String::new();
    let mut steps = 0;
    loop {
        x += dx;
        y += dy;
        steps += 1;
        if x < 0 || y < 0 || x >= width || y >= width {
            return (letters, steps);
        }
        //print!("pos=({},{}) dir=({},{})  ", x, y, dx, dy);
        let c = map[y as usize][x as usize];
        //println!(" char=[{}]", c as char);

        match c {
            b'|' | b'-' => (),  // Just keep walking
            b'A' ... b'|' => {
                                 letters.push(c as char);
                             },
            b'+' => {
                      // Change direction
                      if dy != 0 {
                          dy = 0;
                          if x > 0 && map[y as usize][(x-1) as usize] != b' ' {
                              dx = -1;
                          } else {
                              dx = 1;
                          }
                      } else {
                          dx = 0;
                          if y > 0 && map[(y-1) as usize][x as usize] != b' ' {
                              dy = -1;
                          } else {
                              dy = 1;
                          }
                      }
                    },
            b' ' => { return (letters, steps); },
            _ => panic!("Got to character [{}]", c),
        }
    }
}

fn main() {
    let input = aoc2017::get_input(19).unwrap();

    assert_eq!(solve1(r#"    |          
    |  +--+    
    A  |  C    
F---|----E|--+ 
    |  |  |  D 
    +B-+  +--+ 
"#), (String::from("ABCDEF"), 38));

    println!("Answer 1,2: {:?}", solve1(&input));
}
