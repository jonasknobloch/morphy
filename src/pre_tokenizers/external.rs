use std::io;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use serde::{Serialize, Deserialize};
use tokenizers::{PreTokenizedString, PreTokenizer, SplitDelimiterBehavior};

#[derive(Clone, Debug, PartialEq, Eq)]
#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct External {
    pub path_to_socket: String,
    pub ignored_prefix: String,
    pub split_delimiter: char,
}

impl External {
    pub fn new(path_to_socket: &str, ignored_prefix: &str, split_delimiter: char) -> Self {
        Self {
            path_to_socket: path_to_socket.to_string(),
            ignored_prefix: ignored_prefix.to_string(),
            split_delimiter,
        }
    }

    fn socket(&self, message :&str) -> io::Result<String> {
        let mut stream = UnixStream::connect(self.path_to_socket.as_str())?;

        stream.write_all(message.as_bytes())?;

        let mut buffer = [0; 1024];

        let bytes_read = stream.read(&mut buffer)?;
        let response= String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        Ok(response)
    }
}

impl PreTokenizer for External {
    fn pre_tokenize(&self, pretokenized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        pretokenized.split(|_, mut normalized| {
            let form = normalized.get();

            let has_prefix = form.starts_with(self.ignored_prefix.as_str());

            let prefix = if has_prefix {self.ignored_prefix.as_str() } else { "" };
            let tail = if has_prefix { form.strip_prefix(prefix).unwrap() } else { form };

            if has_prefix & tail.is_empty() {
                return Ok(vec![normalized]);
            }

            let split  =  prefix.to_owned() + self.socket(tail)?.as_str();

            let mut new_chars: Vec<(char, isize)> = vec![];

            split.chars().for_each(|c| {
                if c == self.split_delimiter {
                    new_chars.push((c, 1));
                } else {
                    new_chars.push((c, 0));
                }
            });

            normalized.transform(new_chars, 0);
            normalized.split(self.split_delimiter, SplitDelimiterBehavior::Removed)
        })
    }
}
