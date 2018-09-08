use errors::*;
use getopts::Options;

fn usage(program: &str, opts: &Options) -> String {
    opts.usage(&format!("Usage: {} [options]", program))
}

pub fn parse_opts(args: &[String]) -> Result<(String, u16)> {
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("u", "username", "set username", "NAME");
    opts.optopt("p", "port", "set port number (default: 8080)", "PORT");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print version");

    let matches = opts.parse(&args[1..])?;

    if matches.opt_present("h") {
        bail!(ErrorKind::Help(usage(&program, &opts)));
    }

    if matches.opt_present("v") {
        bail!(ErrorKind::Version);
    }

    let username = matches.opt_str("u").ok_or(ErrorKind::NoUserName)?;

    let port = match matches.opt_str("p") {
        None => 8080,
        Some(p) => match p.parse::<u16>() {
            Err(..) => bail!(ErrorKind::InvalidPort),
            Ok(p) => p,
        },
    };

    Ok((username, port))
}
