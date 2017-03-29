use std::env;              //args()
use std::io::Write;        //writeln!()
use std::process;          //exit()
use std::net::TcpListener; //TcpListener::*()
use std::str::from_utf8;   //from_utf8()
use std::string::String;   //as_bytes()


//Uses u8 buffer array, noting that ASCII and utf8 share the same memory alignment but not nessisarily the same code-point sizes.
const BUF_SIZE : usize = 256; //inlined numeric constant


fn main() {
    //get port string from argv
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
         writeln!( &mut std::io::stderr(), "ERROR, no port provided\n").unwrap(); //write to stderr, panic if it fails.
         process::exit(0); //explicit exit value, and terminates the process without calling destructors.
    }

    //verify port is itype
    let _ : isize = args[1].parse().unwrap_or_else(|_| {
        writeln!( &mut std::io::stderr(), "ERROR, port must be an integer\n").unwrap();
        process::exit(0);
        });
    let ip = "127.0.0.1".to_string();

    //open socket, get ip/port from string, and bind.
    let listener = TcpListener::bind(ip + ":" + args[1].as_str()).unwrap_or_else(|_| {
        writeln!( &mut std::io::stderr(), "ERROR, socket bind failed\n").unwrap();
        process::exit(1);
        });
    
    //listen to socket
    let mut acceptor = listener.listen(); //set listener properties


    //open socket contents
    let mut stream = acceptor.accept().unwrap_or_else(|_| {
        writeln!( &mut std::io::stderr(), "ERROR, stream accept failed\n").unwrap();
        process::exit(1);
    });

    //display data
    let mut buffer = [0u8; BUF_SIZE];
    stream.read(buffer);
    let msg = from_utf8( &buffer ).unwrap_or_else(|_| {
        writeln!( &mut std::io::stderr(), "ERROR, unable to decode utf8 in received message");
        process::exit(1);
    }); //assumes utf8 input, returns &str
    println!("Here is the message: {}\n", msg);

    //respond
    buffer = [0u8; BUF_SIZE];
    buffer.copy_from_slice( "I got your message".as_bytes() ); //stores utf8 in bytes (bytes!("msg") is depreciated)
    stream.write(buffer);
    
    //disconnect (implicit)
    //exit
    process::exit(0);
}
