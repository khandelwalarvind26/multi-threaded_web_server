use std::{
    net::{
        TcpListener, 
        TcpStream
    }, 
    io::{
        BufReader, 
        BufRead, 
        Write
    }, 
    fs, 
    thread, 
    time
};
use hello::ThreadPool;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);


    for stream in listner.incoming() {
        let stream = stream.unwrap();
        
        pool.execute(|| handle_connection(stream));
    }

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (status_line, filename) = match &http_request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK","/home/arvind/Desktop/Dev/Rust/multi-threaded_web_server/html/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(time::Duration::from_secs(5));
            ("HTTP/1.1 200 OK","/home/arvind/Desktop/Dev/Rust/multi-threaded_web_server/html/hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND","/home/arvind/Desktop/Dev/Rust/multi-threaded_web_server/html/404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
    
}