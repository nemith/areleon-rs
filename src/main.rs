use std::path::PathBuf;

use aerleon_rs::def::Definition;
use aerleon_rs::policy;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// aerleron configuration file to use
    #[arg(short, long, alias = "config_file")]
    config: Option<String>,

    #[arg(short, long, alias = "output_directory", default_value = ".")]
    output_dir: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    dbg!(args);

    let input = "
networks:
    RFC1918:
        values:
        - address: 10.0.0.0/8
        - address: 172.16.0.0/12
        - address: 192.168.0.0/16
    WEB_SERVERS:
        values:
        - address: 10.0.0.1/32
          comment: Web Server 1
        - address: 10.0.0.2/32
          comment: Web Server 2
    MAIL_SERVERS:
        values:
        - address: 10.0.0.3/32
          comment: Mail Server 1
        - address: 10.0.0.4/32
          comment: Mail Server 2
    ALL_SERVERS:
        values:
        - WEB_SERVERS
        - MAIL_SERVERS
services:
    HTTP:
        - protocol: tcp
          port: 80
    HTTPS:
        - protocol: tcp
          port: 443
    WEB:
        - HTTP
        - HTTPS
    HIGH_PORTS:
        - port: 1024-65535
          protocol: tcp
        - port: 1024-65535
          protocol: udp
        ";

    let def = Definition::from_yaml_str(&input)?;
    dbg!(def);

    println!("{}", std::mem::size_of::<Option<String>>());
    println!("{}", std::mem::size_of::<String>());

    let typ: policy::Icmp4Types = 55.into();
    dbg!(typ);

    Ok(())
}
