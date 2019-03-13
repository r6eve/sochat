//
//           Copyright r6eve 2019 -
//  Distributed under the Boost Software License, Version 1.0.
//     (See accompanying file LICENSE_1_0.txt or copy at
//           https://www.boost.org/LICENSE_1_0.txt)
//

use errors::*;
use std::io;
use std::net::{IpAddr, UdpSocket};
use std::str;
use std::time::Duration;

#[macro_export]
macro_rules! errorln {
    () => ({
        eprintln!("error");
        ::std::process::exit(1);
    });
    ($fmt:expr) => ({
        eprintln!($fmt);
        ::std::process::exit(1);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        eprintln!($fmt, $($arg)*);
        ::std::process::exit(1);
    });
}

#[macro_export]
macro_rules! print_flush {
    () => (());
    ($fmt:expr) => ({
        print!($fmt);
        ::std::io::stdout().flush().unwrap();
    });
    ($fmt:expr, $($arg:tt)*) => ({
        print!($fmt, $($arg)*);
        ::std::io::stdout().flush().unwrap();
    });
}

pub fn search_server(port: u16) -> Result<Option<IpAddr>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    socket.set_read_timeout(Some(Duration::from_secs(3)))?;

    let mut buf = [0; 4];
    for n_send in 0..3 {
        socket.send_to(b"HELO", ("255.255.255.255", port))?;

        match socket.recv_from(&mut buf) {
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                info!("UDP_SEND: count={}", n_send);
            }
            Err(e) => bail!("{}", e),
            Ok((n, addr)) => {
                let buf = &buf[..n];
                let s = str::from_utf8(buf).unwrap();
                info!("GOT: {:?} [{:?}]", addr, s);
                if s == "HERE" {
                    return Ok(Some(addr.ip()));
                }
            }
        }
    }

    Ok(None)
}
