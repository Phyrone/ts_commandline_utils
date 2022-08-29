use crate::IdentityFile;
use clap::Args;
use clap::{Parser, ValueEnum};
use std::borrow::Borrow;
use std::path::Path;
use std::process::exit;

#[cfg(feature = "view")]
#[derive(Args, Debug)]
pub struct ViewIdentityArgs {
    #[clap(value_parser)]
    file: String,

    #[clap(short, long, arg_enum)]
    format: Option<Format>,

    #[clap(short, long, arg_enum)]
    data: Vec<Data>,
}

#[derive(Debug, ValueEnum, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Format {
    NewLine,
}
#[derive(Debug, ValueEnum, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Data {
    UID,
    LEVEL,

    NICKNAME,
    PhoneticNickname,

    ID,

    PrivateKey,
}

#[cfg(feature = "view")]
pub fn run(identity_subcommand_args: ViewIdentityArgs) {
    let path = Path::new(identity_subcommand_args.file.as_str());
    if path.exists() {
        match IdentityFile::read_from_open_file(path) {
            Ok(identity_file) => {
                let identity = identity_file.to_identity().unwrap(); //TODO error handling
                let format = match identity_subcommand_args.format {
                    Some(format) => format,
                    None => Format::NewLine,
                };
                match format {
                    Format::NewLine => {
                        for datum in identity_subcommand_args.data {
                            match datum {
                                Data::UID => {
                                    println!("{}", identity.key().to_pub().get_uid())
                                }
                                Data::LEVEL => {
                                    println!("{}", identity.level())
                                }
                                Data::ID => {
                                    println!(
                                        "{}",
                                        identity_file.clone().id.unwrap_or("".to_string())
                                    )
                                }
                                Data::NICKNAME => {
                                    println!(
                                        "{}",
                                        identity_file.clone().nickname.unwrap_or("".to_string())
                                    )
                                }
                                Data::PhoneticNickname => {
                                    println!(
                                        "{}",
                                        identity_file
                                            .clone()
                                            .phonetic_nickname
                                            .unwrap_or("".to_string())
                                    )
                                }
                                Data::PrivateKey => {
                                    println!("{}", identity.key().to_ts())
                                }
                            }
                        }
                    }
                };
            }
            Err(e) => {
                eprintln!("Could not open {}: {}", path.display(), e);
                exit(2);
            }
        }
    } else {
        eprintln!("{} does not exist", path.display());
        exit(2);
    }
}
