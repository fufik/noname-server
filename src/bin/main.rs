use regex::Regex;
use std::fs;
use std::path::Path;
use std::thread;
use std::io::prelude::*;
use std::time::Duration;
use std::net::TcpStream;
use std::net::TcpListener;
use noname_server::ThreadPool;
fn main() {
    println!("Rust server version 0.2 is up on port 7878");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("CON:Connection established with {}",stream.peer_addr().unwrap());
        pool.execute(||{
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();

    let mut answer = move |Code: (u16,&str), contents: String|{
        if Code.0!=200 
            {println!("ERR: {} {}.",Code.0,Code.1);}
        //else 
           // {println!("SUC!");}
        let response = format!("HTTP/1.1 {} {}\r\n\r\n{}", Code.0,Code.1, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("CON:Connection closed with {}",stream.peer_addr().unwrap());
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
                err_answer((400,"Bad request"));
                return;
            },
        }
        
        let mut contents = String::new();
        if address == "/"
        {
            println!("REQ: index.html");
            contents = fs::read_to_string("index.html").unwrap();
        } else {
            let fname = &address[1..]; 
            println!("REQ: {}",fname);
            if !Path::new(fname).exists(){
                err_answer((404,"Not found"));
                return
            }
            let ext = match Path::new(&fname).extension(){
                Some(osstr) => osstr.to_str().unwrap(),
                None        => "no_ext"
            };
            if(ext == "html"){
                let a = fs::read_to_string(fname);
                match a{
                    Ok(n)  => contents = n,
                    Err(_) => {
                        err_answer((403,"Forbidden"));
                        return;
                    },
                }
            } else{
                err_answer((501,"Not implemented"));
                return
            }//TODO images
        }
        answer((200,"OK"),contents);
    }
    

}
