use regex::Regex;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    println!("Rust server version 0.1 is up on port 7878");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Connection established");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let re = Regex::new(r"GET (?P<address>([0-9A-Za-z]|/|\.|_)+)\??(?P<params>([0-9A-Za-z]|=|\+|%|:|-|_)+)? ").unwrap();
    let mut buffer = [0;512];
    let mut contents = String::new();
    let (mut address,mut params,mut code) = ("/","","200 OK");
    stream.read(&mut buffer).unwrap();
    let buffer = String::from_utf8_lossy(&buffer);
    let capture = &re.captures(&buffer);
    match capture{
        Some(a) => {
                    address = match a.name("address"){
                        Some(b) => b.as_str(),
                        None    => "/",
                    };
                    params  = match a.name("params"){
                        Some(b) => b.as_str(),
                        None    => "",
                    };
        },
        None    =>  {
                     println!("400 Bad request");
                     code="400 Bad request";
        },
    }
    println!("{}",address);
    if address == "/"{
        contents = fs::read_to_string("index.html").unwrap();
    } else {
        let fname = &address[1..];
        //println!("{}",fname);
        let a = fs::read_to_string(fname);
        match a{
            Ok(n)  => contents = n,
            Err(_) => {
                       println!("404 Not Found");
                       code = "404 Not Found";
            },
        }
    }
    if code != "200 OK"{
        let a = &code[0..3];
        contents = fs::read_to_string(format!("errors/{}.html",a)).unwrap();
    }
    let response = format!("HTTP/1.1 {}\r\n\r\n{}", code, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    //println!("Request: {}",String::from_utf8_lossy(&buffer[..]));
}
