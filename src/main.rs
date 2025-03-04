use std::str::FromStr;
use std::{env, fmt::Arguments, net::IpAddr};

struct Args {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Args {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Args {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!("Usage:\n-j: select threads\n-h: show help message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("too many args");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("invalid IP"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread no."),
                };
                return Ok(Args {
                    threads,
                    flag,
                    ipaddr,
                });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for i in &args {
        println!("{}", i);
    }

    println!("{:?}", args);
    let program = args[0].clone();
}
