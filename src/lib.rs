
use std::io;
use std::io::{BufRead, BufReader};

const LF: u8 = '\n' as u8;

pub fn count_lines<R: io::Read>(handle: R) -> Result<usize, io::Error> {
    let mut reader = BufReader::new(handle);
    let mut count = 0;
    let mut line: Vec<u8> = Vec::new();
    while match reader.read_until(LF, &mut line) {
        Ok(n) if n > 0 => true,
        Err(e) => return Err(e),
        _ => false,
    } {
        if *line.last().unwrap() == LF {
            count += 1;
        };
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_lines() {
        let f: &[u8] = b"some text\nwith\nfour\nlines\n";
        assert_eq!(count_lines(f).unwrap(), 4);
    }
}
