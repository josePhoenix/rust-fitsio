use sys::ffgerr;
use libc::{c_char, c_int};
use std::string::FromUtf8Error;

/// Helper function converting a C string pointer to Rust String
pub fn buf_to_string(buffer: &[c_char]) -> Result<String, FromUtf8Error> {
    String::from_utf8(buffer.iter()
        .map(|&x| x as u8)
        .filter(|&x| x != 0)
        .collect())
}

/// Internal function to get the fits error description from a status code
pub fn status_to_string(status: c_int) -> Option<String> {
    match status {
        0 => None,
        status => {
            let mut buffer: Vec<c_char> = vec![0; 31];
            unsafe {
                ffgerr(status, buffer.as_mut_ptr());
            }
            let result_str = buf_to_string(&buffer).unwrap();
            Some(result_str)
        }
    }
}

#[cfg(test)]
mod test {
    use super::status_to_string;

    #[test]
    fn returning_error_messages() {
        assert_eq!(status_to_string(105).unwrap(),
                   "couldn't create the named file");
    }
}
