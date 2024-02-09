use std::fs::File;
use chrono::{Local, DateTime};
use gethostname::gethostname;
use std::io::{BufReader, Read};
use serde::Deserialize;
use std::env;
use rustelebot::{create_instance, send_message};
use rustelebot::types::{SendMessageOption, SendMessageParseMode};
use args::{Args, ArgsError};
use getopts::Occur;
use std::process::exit;

#[derive(Debug)]
#[derive(Deserialize)]
struct Config {
    token: String,
    chat: String,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

//=================================================================================
fn parse() -> Result<String, ArgsError> {
    let input_args: Vec<String> = env::args().collect();
    let mut args = Args::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_DESCRIPTION"));
    args.flag("h", "help", "Print the usage menu");
    args.flag("v", "version", "Print version");
    args.option(
        "c",
        "config",
        "The name of json config file with telegram token and chat_id, \
        /etc/pam_tbot_send.json by default",
        "/etc/pam_tbot_send.json",
        Occur::Optional,
        Some(String::from("/etc/pam_tbot_send.json")),
    );

    args.parse(input_args)?;

    let help = args.value_of("help")?;
    if help {
        println!("{}", args.full_usage());
        exit(0);
    }
    let version = args.value_of("version")?;
    if version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        exit(0);
    }
    let conf = args.value_of("config")?;
    Ok(conf)
}

fn read_conf(file_name: String) -> Config {
    let file =
        File::open(&file_name).expect("Please, add configuration to /etc/pam_tbot_send.json");
    let mut br = BufReader::new(file);
    let mut config_json = String::new();
    br.read_to_string(&mut config_json).expect(
        "Unable to read from file",
    );
    serde_json::from_str(&config_json).unwrap()
}

fn get_env() -> String {
    let mut s = String::from(format!(
        "*LoginBot v.{}* on `{:?}`\n",
        VERSION,
        gethostname()
    ));
    let now: DateTime<Local> = Local::now();
    s = s + &format!("`{}`\n", now);
    let env_vars = ["SERVICE", "TTY", "USER", "TYPE", "RHOST"];
    for v in env_vars {
        match env::var_os(format!("PAM_{}", v)) {
            Some(val) => s = s + &String::from(format!("*{}:* `{:?}`\n", v, val)),
            None => (),
        };
    }
    s
}

fn main() {
    let config_file = match parse() {
        Ok(d) => d,
        Err(e) => panic!("Unable to parse arguments: {:?}", e),
    };
    let config = read_conf(config_file);
    let message = get_env().replace(".", r"\.");
    let instance = create_instance(&config.token, &config.chat);
    let option = SendMessageOption { parse_mode: Some(SendMessageParseMode::MarkdownV2) };
    match send_message(&instance, &message, Some(option)) {
        Ok(_) => (),
        Err(e) => panic!("Unable to send message: {} - {}", e, message),
    };
}
