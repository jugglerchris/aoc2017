use std::fs::File;
use std::io;
use std::io::{Read};

pub fn get_input(n: u32) -> io::Result<String> {
    let filename = format!("data/{}.txt", n);
    let mut f = try!(File::open(&filename));
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
