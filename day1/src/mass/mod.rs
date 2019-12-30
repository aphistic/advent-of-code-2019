use std::io;
use std::io::BufRead;
use std::error::Error;

pub struct Reader<R> {
    r: io::BufReader<R>
}

impl<R: io::Read> Reader<R> {
    pub fn new(r: R) -> Reader<R> {
        Reader { r: io::BufReader::new(r) }
    }
}

impl<R: io::Read> Iterator for Reader<R> {
    type Item = Result<u32, String>;

    fn next(&mut self) -> Option<Result<u32, String>> {
        loop {
            let mut buf = String::new();
            match self.r.read_line(&mut buf) {
                Ok(0) => return None,
                Ok(_) => {
                    let trimmed_buf = buf.trim();
                    match trimmed_buf {
                        "" => continue,
                        _ => match buf.trim().parse::<u32>() {
                            Ok(val) => return Some(Ok(val)),
                            Err(error) => { return Some(Err(String::from(error.description()))) }
                        }
                    }
                }
                Err(error) => { return Some(Err(String::from(error.description()))) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod reader {
        use super::super::*;
        use std::io;

        #[test]
        fn iter_next_success() {
            let mut data = io::BufReader::new("14\n12".as_bytes());
            let mut r = Reader::new(data.get_mut());
            assert_eq!(Some(Ok(14)), r.next());
            assert_eq!(Some(Ok(12)), r.next());
            assert_eq!(None, r.next());
        }

        #[test]
        fn iter_next_empty_line() {
            let mut data = io::BufReader::new("14\n\n12".as_bytes());
            let mut r = Reader::new(data.get_mut());
            assert_eq!(Some(Ok(14)), r.next());
            assert_eq!(Some(Ok(12)), r.next());
            assert_eq!(None, r.next());
        }
    }
}