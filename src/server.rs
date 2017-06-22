use errors::*;
use mio::*;
use mio::net::{TcpStream, TcpListener};
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::net::UdpSocket;
use std::str;
use std::thread;

const MAX_SOCKETS: usize = 32;

pub struct Server { port: u16 }

impl Server {
    pub fn new(port: u16) -> Server { Server { port } }

    pub fn up_udp_server(&self) {
        let port = self.port;

        thread::spawn(move || {
            let socket = UdpSocket::bind(("0.0.0.0", port))
                .unwrap_or_else(|e| errorln!("UDP bind error: {}", e));

            let mut buf = [0; 4];
            loop {
                let (n, ref addr) = socket.recv_from(&mut buf)
                    .unwrap_or_else(|e| errorln!("recv_from error: {}", e));

                let buf = &buf[..n];

                info!("UDP_RECV_FROM: {:?} [{:?}]",
                      addr,
                      str::from_utf8(buf)
                          .unwrap_or_else(|e| errorln!("convert error: {}", e)));

                if buf == b"HELO" {
                    socket.send_to(b"HERE", addr)
                        .unwrap_or_else(|e| errorln!("send_to error: {}", e));
                }
                // ignore other messages
            }
        });
    }

    pub fn up_tcp_server(&self) {
        let port = self.port;

        thread::spawn(move || {
            const LISTENER: Token = Token(1024);

            let poll = Poll::new().unwrap();

            let inaddr_any = format!("0.0.0.0:{}", port).parse()
                .unwrap_or_else(|e| errorln!("parse INADDR_ANY error: {}", e));

            let listener = TcpListener::bind(&inaddr_any)
                .unwrap_or_else(|e| errorln!("TCP bind error: {}", e));

            poll.register(&listener,
                          LISTENER,
                          Ready::readable(),
                          PollOpt::edge()).unwrap();

            let mut sockets = HashMap::new();
            let mut logined = HashMap::new();
            let mut next_socket_index = 0;
            let mut events = Events::with_capacity(1024);
            loop {
                poll.poll(&mut events, None).unwrap();
                for event in events.iter() {
                    match event.token() {
                        LISTENER => {
                            listener_action(&listener, &poll, &mut next_socket_index, &mut sockets)
                                .unwrap_or_else(|e| errorln!("listener error: {}", e));
                        }
                        token => {
                            reader_action(&token, &mut sockets, &mut logined)
                                .unwrap_or_else(|e| errorln!("reader error: {}", e));
                        }
                    }
                }
            }
        });
    }
}

fn listener_action(listener: &TcpListener,
                   poll: &Poll,
                   next_socket_index: &mut usize,
                   sockets: &mut HashMap<Token, TcpStream>) -> Result<()>
{
    loop {
        match listener.accept() {
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => bail!("accept error: {}", e),
            Ok((socket, _addr)) => {
                if *next_socket_index == MAX_SOCKETS {
                    bail!("MAX SOCKETS. Shutdown a server.");
                }

                let token = Token(*next_socket_index);
                *next_socket_index += 1;

                poll.register(&socket,
                              token,
                              Ready::readable(),
                              PollOpt::edge()).unwrap();

                info!("INSERT: {:?} {:?}", token, socket);
                sockets.insert(token, socket);

                return Ok(());
            }
        }
    }
}

fn reader_action(token: &Token,
                 sockets: &mut HashMap<Token, TcpStream>,
                 logined: &mut HashMap<Token, String>) -> Result<()>
{
    let mut buf = [0; 512];

    loop {
        match sockets.get_mut(token).unwrap().read(&mut buf) {
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => bail!("{}", e),
            Ok(0) => {
                let _ = sockets.remove(token);
                match logined.remove(token) {
                    None => {
                        warn!("UNKOWN USER CONNECTED");
                        return Ok(());
                    }
                    Some(username) => {
                        let mesg = format!("MESG [system]{} is logout.", username);
                        let mesg = mesg.as_bytes();
                        for t in logined.keys() {
                            let _ = sockets.get_mut(t).unwrap().write(mesg)?;
                        }
                        return Ok(());
                    }
                }
            }
            Ok(n) => {
                let buf = &buf[..n];
                info!("TCP_READ: [{:?}]", str::from_utf8(buf)?);

                if buf.starts_with(b"QUIT") {
                    let _ = sockets.remove(token);
                    let username = match logined.remove(token) {
                        None => {
                            info!("Non-joined user sent QUIT");
                            return Ok(());
                        }
                        Some(u) => u,
                    };
                    let mesg = format!("MESG [system]{} is logout.", username);
                    let mesg = mesg.as_bytes();
                    for t in logined.keys() {
                        let _ = sockets.get_mut(t).unwrap().write(mesg)?;
                    }
                    return Ok(());
                } else if buf.starts_with(b"JOIN ") {
                    let username = str::from_utf8(&buf[5..n])?;
                    info!("JOIN: [{:?}]", username);
                    let mesg = format!("MESG [system]{} is logined.", username);
                    let mesg = mesg.as_bytes();
                    for t in logined.keys() {
                        let _ = sockets.get_mut(t).unwrap().write(mesg)?;
                    }
                    logined.insert(*token, username.to_string());
                    return Ok(());
                } else if buf.starts_with(b"POST ") {
                    match logined.get(token) {
                        None => {
                            warn!("UNKOWN USER CONNECTED");
                            return Ok(());
                        }
                        Some(username) => {
                            let mesg = format!("MESG [{}]{}", username, str::from_utf8(&buf[5..n])?);
                            let mesg = mesg.as_bytes();
                            for t in logined.keys() {
                                if *t == *token { continue }
                                let _ = sockets.get_mut(t).unwrap().write(mesg)?;
                            }
                            return Ok(());
                        }
                    }
                } else {
                    // ignore other messages
                    return Ok(());
                }
            }
        }
    }
}
