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
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();

    let mut answer = move |Code: (u16,&str), contents: String|{
        let response = format!("HTTP/1.1 {} {}\r\n\r\n{}", Code.0,Code.1, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    };

    let mut err_answer = |Code: (u16,&str)|{
        let contents = fs::read_to_string(format!("errors/{}.html",Code.0)).unwrap();
        answer(Code,contents);
    };

    if !buffer.starts_with(b"GET")
    {
        //...Something I don't get, 
        err_answer((400,"Bad request"));
        return;
    } else {
        let re = Regex::new(r"GET (?P<address>([0-9A-Za-z]|/|\.|_)+)\??(?P<params>([0-9A-Za-z]|=|\+|%|:|-|_)+)? ").unwrap();
        let buffer = String::from_utf8_lossy(&buffer);
        let capture = &re.captures(&buffer);
        let (mut address,mut params) = ("/","");
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
                err_answer((400,"Bad request"));
                return;
            },
        }
        
        let mut contents = String::new();
        if address == "/"
        {
            contents = fs::read_to_string("index.html").unwrap();
        } else {
            let fname = &address[1..];
            //println!("{}",fname);
            let a = fs::read_to_string(fname);
            match a{
                Ok(n)  => contents = n,
                Err(_) => {
                    println!("404 Not Found");
                    err_answer((404,"Not found"));
                    return;
                },
            }
        }
        answer((200,"OK"),contents);
    }
    

}
