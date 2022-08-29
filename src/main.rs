use clap::{Parser, Subcommand};
use log::debug;
use pretty_env_logger::init as init_logger;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};
use tsclientlib::Identity;

mod create;
mod view;

use crate::create::CreateIdentitySubcommand;
use crate::view::ViewIdentityArgs;

fn main() {
    init_logger();

    let args: CommandlineArgs = CommandlineArgs::parse();
    debug!("{:?}", args);

    match args.subcommand {
        CategorySubcommand::CREATE(create_args) => create::run(create_args),
        CategorySubcommand::VIEW(view_args) => view::run(view_args),
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CommandlineArgs {
    #[clap(subcommand)]
    pub subcommand: CategorySubcommand,
}

#[derive(Subcommand, Debug)]
pub enum CategorySubcommand {
    #[cfg(feature = "create")]
    #[clap(about = "create a new identity teamspeak identity")]
    CREATE(CreateIdentitySubcommand),
    #[cfg(feature = "view")]
    #[clap(about = "view an existing teamspeak identity")]
    VIEW(ViewIdentityArgs),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct IdentityFile {
    id: Option<String>,
    identity: String,
    nickname: Option<String>,
    phonetic_nickname: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct IdentityFileWrapper {
    #[serde(rename(serialize = "Identity", deserialize = "Identity"))]
    payload: IdentityFile,
}

impl IdentityFileWrapper {
    fn wrap(identity_file: IdentityFile) -> Self {
        IdentityFileWrapper {
            payload: identity_file,
        }
    }
    fn unwrap(self) -> IdentityFile {
        self.payload
    }
}

impl IdentityFile {
    #[cfg(feature = "write")]
    fn from_identity(identity: &Identity) -> IdentityFile {
        let mut exp = identity.counter().to_string();
        exp.push_str("V");
        exp.push_str(identity.key().to_ts_obfuscated().as_str());
        IdentityFile {
            id: None,
            identity: exp,
            nickname: None,
            phonetic_nickname: None,
        }
    }

    #[cfg(feature = "read")]
    fn to_identity(&self) -> Result<Identity, ()> {
        match Identity::new_from_ts_str(self.identity.as_str()) {
            Ok(i) => Ok(i),
            Err(_) => Err(()), //TODO error handling
        }
    }

    #[cfg(feature = "read")]
    fn read_from_open_file(file: &Path) -> Result<IdentityFile, io::Error> {
        let file_content_string = fs::read_to_string(file)?;
        let identity_file =
            toml::from_str::<IdentityFileWrapper>(file_content_string.as_str())?.unwrap();
        Ok(identity_file)
    }

    #[cfg(feature = "write")]
    fn to_toml_string(&self) -> String {
        toml::to_string(&IdentityFileWrapper::wrap(self.clone())).unwrap()
    }

    #[cfg(feature = "write")]
    fn write_to_file(&self, path: &Path) -> Result<(), io::Error> {
        let mut file = File::create(path)?;
        let tml = IdentityFile::to_toml_string(self);
        file.write_all(tml.as_bytes())?;
        Ok(())
    }
}
