use crate::IdentityFile;
use clap::Args;
use log::info;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use tsclientlib::Identity;

#[derive(Args, Debug)]
#[cfg(feature = "create")]
pub struct CreateIdentitySubcommand {
    #[clap(short, long)]
    pub level: Option<u8>,

    #[clap(short, long)]
    pub id: Option<String>,

    #[clap(short, long)]
    pub nickname: Option<String>,

    #[clap(short, long)]
    pub phonetic_nickname: Option<String>,

    #[clap(short, long)]
    pub save: Option<String>,
}

pub fn run(identity_subcommand_args: CreateIdentitySubcommand) {
    info!("{:?}", identity_subcommand_args);
    let mut identity = Identity::create();
    match identity_subcommand_args.level {
        Some(level) => identity.upgrade_level(level),
        None => (),
    }

    let mut identity_file = IdentityFile::from_identity(&identity);
    identity_file.id = identity_subcommand_args.id.clone();
    identity_file.nickname = identity_subcommand_args.nickname.clone();
    identity_file.phonetic_nickname = identity_subcommand_args.phonetic_nickname.clone();

    match identity_subcommand_args.save {
        None => {
            print!("{}", identity_file.to_toml_string());
        }
        Some(path_string) => {
            let path = Path::new(path_string.as_str());
            if path.exists() {
                eprintln!("{} already exists", path.display());
                exit(2);
            } else {
                match identity_file.write_to_file(&path) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Could not create {}: {}", path.display(), e);
                        exit(3);
                    }
                }
            }
        }
    }
}
