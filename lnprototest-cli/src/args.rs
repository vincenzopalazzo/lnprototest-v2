use std::collections::HashMap;

use radicle_term as term;
use serde_json as json;

#[derive(Debug)]
pub struct ProtoTestCliArgs {
    pub datadir: String,
    pub method: String,
    pub args: HashMap<String, json::Value>,
}

struct Help {
    name: &'static str,
    description: &'static str,
    version: &'static str,
    usage: &'static str,
}

const HELP: Help = Help {
    name: "lnprototest-cli",
    description: "LN Prototest test command line program",
    version: env!("CARGO_PKG_VERSION"),
    usage: r#"
Usage

    lnprototest-cli [<option> ...] <method> [arg=value]

Options

    -d | --data-dir     Specify lampo data directory (used to get socket path)
    -n | --network      Set the network for lampo (default: testnet)
    -h | --help         Print help
"#,
};

pub fn parse_args() -> Result<ProtoTestCliArgs, lexopt::Error> {
    use lexopt::prelude::*;

    let mut data_dir: Option<String> = None;
    let mut _network: Option<String> = None;
    let mut method: Option<String> = None;
    let mut args = HashMap::<String, json::Value>::new();

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('d') | Long("data-dir") => {
                let val: String = parser.value()?.parse()?;
                data_dir = Some(val);
            }
            Short('n') | Long("network") => {
                let val: String = parser.value()?.parse()?;
                _network = Some(val);
            }
            Long("help") => {
                let _ = print_help();
                std::process::exit(0);
            }
            Long(val) => {
                if method.is_none() {
                    return Err(lexopt::Error::MissingValue {
                        option: Some("method is not specified".to_owned()),
                    });
                }
                log::debug!("look for args {:?}", val);
                match arg {
                    Long(val) => {
                        let key = val.to_string();
                        let val: String = parser.value()?.parse()?;
                        if let Ok(val) = val.parse::<u64>() {
                            let val = json::json!(val);
                            args.insert(key.clone(), val);
                        } else if let Ok(val) = val.parse::<bool>() {
                            let val = json::json!(val);
                            args.insert(key.clone(), val);
                        } else {
                            let val = json::json!(val);
                            args.insert(key, val);
                        }
                    }
                    _ => return Err(arg.unexpected()),
                }
            }
            Value(ref val) => {
                if args.is_empty() && method.is_none() {
                    method = Some(val.clone().string()?);
                    log::debug!("find a method {:?}", method);
                    continue;
                }
                return Err(arg.unexpected());
            }
            _ => return Err(arg.unexpected()),
        }
    }

    log::debug!("args parser are {:?} {:?}", method, args);
    Ok(ProtoTestCliArgs {
        datadir: data_dir.unwrap(),
        method: method.ok_or_else(|| lexopt::Error::MissingValue {
            option: Some(
                "Too few params, a method need to be specified. Try run `lampo-cli --help`"
                    .to_owned(),
            ),
        })?,
        args,
    })
}

// Print helps
pub fn print_help() {
    println!(
        "{}",
        term::format::secondary(
            "Common `lnprototest-cli` commands used to init the lnprototest deamon"
        )
    );
    println!(
        "\n{} {}",
        term::format::bold("Usage:"),
        term::format::dim("lnprototest [<option> ...] <method> [arg=value]")
    );
    println!();

    println!(
        "\t{} version {}",
        term::format::bold("lnprototest-cli"),
        term::format::dim(HELP.version)
    );
    println!(
        "\t{} {}",
        term::format::bold(format!("{:-12}", HELP.name)),
        term::format::dim(HELP.description)
    );
    println!("{}", term::format::bold(HELP.usage));
}
