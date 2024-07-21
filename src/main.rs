use args::{Args, ArgsError};
use getopts::Occur;
use lib_notify::{AgentType, NotifyAgent};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

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

fn read_conf(file_name: String) -> AgentType {
    let file =
        File::open(&file_name).expect("Please, add configuration to /etc/pam_tbot_send.json");
    let mut br = BufReader::new(file);
    let mut config_json = String::new();
    br.read_to_string(&mut config_json)
        .expect("Unable to read from file");
    serde_json::from_str(&config_json).unwrap()
}

fn get_env() -> String {
    let mut s = String::new();
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
    let message = get_env();
    let app_name = format!("LoginBot v{}", env!("CARGO_PKG_VERSION"));
    let (na, _) = NotifyAgent::new(&config, &app_name, 10, 0);
    na.send_notify(&message);
}
