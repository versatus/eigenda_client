use clap::{command, Arg, Command};
use eigenda_client::status::BlobStatus;
use regex::Regex;
use std::str::FromStr;

fn parse_blob() -> Command {
    Command::new("parse-blob").arg(Arg::new("json").short('j').long("json").required(true))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .about("A cli interacting with the LASR network")
        .subcommand(parse_blob())
        .get_matches();

    match matches.subcommand() {
        Some(("parse-blob", children)) => {
            let json = children.get_one::<String>("json").expect("required");
            let re = Regex::new(r"(\\n|\\t|\n\t|\s\s+)").unwrap();
            let clean_json = re.replace_all(json, " ").to_string();
            println!("\n{:?}", clean_json);
            let blob_status: BlobStatus = BlobStatus::from_str(&clean_json)?;
            println!("{:?}", blob_status);
        }
        _ => {}
    }
    Ok(())
}
