//
//           Copyright r6eve 2019 -
//  Distributed under the Boost Software License, Version 1.0.
//     (See accompanying file LICENSE_1_0.txt or copy at
//           https://www.boost.org/LICENSE_1_0.txt)
//

error_chain!{
    foreign_links {
        Getopts(::getopts::Fail);
        IO(::std::io::Error);
        Str(::std::str::Utf8Error);
    }

    errors {
        Help(t: String) {
            display("{}", t)
        }
        Version {
            display("{}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
        }
        NoUserName {
            display("set username")
        }
        InvalidPort {
            display("set port number as an unsigned 16 bit integer")
        }
    }
}
