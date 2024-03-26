use std::io;
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::exit;

use json::{to_writer, Deserializer};
use radicle_term as term;
use serde_json as json;

use crate::args::ProtoTestCliArgs;

mod args;
mod logger;

fn main() -> io::Result<()> {
    logger::init("debug", None);
    let args = match args::parse_args() {
        Ok(args) => args,
        Err(err) => {
            term::error(format!("{err}"));
            exit(1);
        }
    };
    let resp = run(args);
    match resp {
        Ok(resp) => {
            term::print(json::to_string_pretty(&resp)?);
        }
        Err(err) => {
            term::error(format!("{err}"));
        }
    }
    Ok(())
}

fn run(args: ProtoTestCliArgs) -> io::Result<json::Value> {
    let mut client = UnixStream::connect(&format!("{}/lnprototest.sock", args.datadir))?;
    to_writer(&mut client, &json::json!({
        "id": "lnprototest/1",
        "method": args.method,
        "params": args.args,
    }))?;
    let response: json::Value = Deserializer::from_reader(&mut client)
            .into_iter()
            .next().unwrap()?;
    Ok(response)
}
