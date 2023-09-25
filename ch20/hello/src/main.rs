use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use chrono::{DateTime, Utc, TimeZone};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    //logging
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
        Ok(n) => {
            let utc_time = UNIX_EPOCH + n;
            let datetime: DateTime<Utc> = DateTime::from(utc_time);
            println!("{:?} {:?} {} {}",n.as_secs(), datetime.format("%Y-%m-%d %H:%M:%S").to_string(), request_line, status_line)
        }
        Err(_) => panic!("Sytemtime error"),
    }
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
