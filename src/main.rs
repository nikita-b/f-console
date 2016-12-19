use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read, BufReader, BufRead};

fn handle_client(mut stream: TcpStream) {
    let mut rstream = BufReader::new(stream.try_clone().unwrap());
    let mut buffer = String::new();
    //try!(stream.read_to_end(&mut buffer));
    stream.write(b"Welcome to F-Console, Stranger!\nEnter password:").unwrap();
    rstream.read_line(&mut buffer).unwrap();
    //#stream.read()
    //let mut name : ~str = ss.read_line().unwrap();
    //stream.write(b"").unwrap();
    //stream.write_str(format!("Hello, {}!\n", name));
}

fn main() {


    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
             }
            Err(e) => { 
                println!("Connection failed: {}", e);
            }
        }
    }
}
