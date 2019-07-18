//
//           Copyright r6eve 2019 -
//  Distributed under the Boost Software License, Version 1.0.
//     (See accompanying file LICENSE_1_0.txt or copy at
//           https://www.boost.org/LICENSE_1_0.txt)
//

use errors::*;
use mio::net::TcpStream;
use mio::unix::EventedFd;
use mio::*;
use std::io;
use std::io::prelude::*;
use std::net;
use std::process;
use std::str;

pub struct Client<'a> {
    username: &'a str,
    ip: &'a str,
    port: u16,
}

impl<'a> Client<'a> {
    pub fn new(username: &'a str, ip: &'a str, port: u16) -> Client<'a> {
        Client { username, ip, port }
    }

    pub fn up_tcp_client(&self) {
        const STDIN: Token = Token(0);
        const CLIENT: Token = Token(1);

        let stream = net::TcpStream::connect((self.ip, self.port))
            .unwrap_or_else(|e| errorln!("connect error: {}", e));

        let mut stream = TcpStream::from_stream(stream)
            .unwrap_or_else(|e| errorln!("set non-blocking error: {}", e));

        let _ = stream
            .write(format!("JOIN {}", self.username).as_bytes())
            .unwrap_or_else(|e| errorln!("write JOIN error: {}", e));

        let poll = Poll::new().unwrap();

        let stdin_fd = 0;
        poll.register(
            &EventedFd(&stdin_fd),
            STDIN,
            Ready::readable(),
            PollOpt::edge(),
        )
        .unwrap();

        poll.register(&stream, CLIENT, Ready::readable(), PollOpt::edge())
            .unwrap();

        let mut events = Events::with_capacity(1024);
        loop {
            print_flush!("> ");
            poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                match event.token() {
                    STDIN => {
                        match stdin_action(&mut stream) {
                            Err(e) => errorln!("stdin error: {}", e),
                            Ok(true) => (),
                            Ok(false) => {
                                drop(stream); // close stream explicitly
                                process::exit(0);
                            }
                        }
                    }
                    CLIENT => {
                        match reader_action(&mut stream) {
                            Err(e) => errorln!("reader error: {}", e),
                            Ok(true) => (),
                            Ok(false) => {
                                println!("\nserver is down");
                                drop(stream); // close stream explicitly
                                process::exit(0);
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        // never reached
    }
}

fn stdin_action(stream: &mut TcpStream) -> Result<bool> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let s = line.trim_right();
    if s == "QUIT" {
        let _ = stream.write(s.as_bytes())?;
        return Ok(false);
    }
    let _ = stream.write(format!("POST {}", s).as_bytes())?;
    Ok(true)
}

fn reader_action(stream: &mut TcpStream) -> Result<(bool)> {
    let mut buf = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => bail!("{}", e),
            Ok(0) => return Ok(false),
            Ok(n) => {
                let buf = &buf[..n];
                info!("TCP_READ: [{:?}]", str::from_utf8(buf)?);
                if buf.starts_with(b"MESG ") {
                    println!("\n{}", str::from_utf8(&buf[5..n])?);
                }
                return Ok(true);
            }
        }
    }
}
