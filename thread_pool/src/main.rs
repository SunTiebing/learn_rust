use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use thread_pool::ThreadPool;

fn main() {
    let linstener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in linstener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // println!("{}", String::from_utf8_lossy(&buffer));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 404 NOT FUND", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let resp = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
