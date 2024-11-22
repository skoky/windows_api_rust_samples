use std::io::{Read, Write};
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    let pipe_name = r".socket_file";
    let mut pipe = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(pipe_name)?;

    // Write to the pipe
    let message = b"Hello from Windows!";
    pipe.write_all(message)?;

    // Read from the pipe
    let mut buffer = [0u8; 100];
    pipe.read(&mut buffer)?;

    // Process the received data
    println!("Received: {:?}", &buffer[..buffer.len()]);

    Ok(())
}