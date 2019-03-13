//
//           Copyright r6eve 2019 -
//  Distributed under the Boost Software License, Version 1.0.
//     (See accompanying file LICENSE_1_0.txt or copy at
//           https://www.boost.org/LICENSE_1_0.txt)
//

#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate getopts;
extern crate mio;

use std::env;
use std::io::prelude::*;
use std::process;

mod errors;
#[macro_use]
mod util;
mod argv;
mod client;
mod server;

use client::Client;
use errors::*;
use server::Server;

fn main() {
    env_logger::init();

    let (username, port) = match argv::parse_opts(&env::args().collect::<Vec<_>>()) {
        Ok(x) => x,
        Err(ref e @ Error(ErrorKind::Help(_), _)) => {
            print_flush!("{}", e);
            process::exit(2);
        }
        Err(ref e @ Error(ErrorKind::Version, _)) => {
            println!("{}", e);
            process::exit(2);
        }
        Err(ref e) => errorln!("{}", e),
    };

    let ip = match util::search_server(port) {
        Err(ref e) => errorln!("{}", e),
        Ok(Some(ip)) => ip.to_string(),
        Ok(None) => {
            println!("{}, You're a Server.", username);
            let server = Server::new(port);
            server.up_udp_server();
            server.up_tcp_server();
            "localhost".to_string()
        }
    };

    // XXX: use `&str` type to pass "localhost"
    let client = Client::new(&username, &ip, port);
    client.up_tcp_client();

    // never reached
}
