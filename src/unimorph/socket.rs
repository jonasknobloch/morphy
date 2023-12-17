use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;

pub fn split(message :&str) -> io::Result<String> {
    let mut stream = UnixStream::connect("/tmp/unimorph.sock")?;

    stream.write_all(message.as_bytes())?;

    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer)?;
    let response= String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

    Ok(response)
}
