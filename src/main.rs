use clap::{Parser, Subcommand};
use libmc::accounts::authenticate;
use std::{error::Error, path::Path};

#[derive(Parser)]
#[clap(about, version, author)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ListMinecraftVersions,
    Instance(Instance),
    Config,
    Account(Account)
}

#[derive(Parser)]
struct Instance {
    #[clap(subcommand)]
    command: InstanceCommand,
}

#[derive(Subcommand)]
enum InstanceCommand {
    List,
}

#[derive(Parser)]
struct Account {
    #[clap(subcommand)]
    command: AccountCommand,
}

#[derive(Subcommand)]
enum AccountCommand {
    Add
}

fn main() -> Result<(), Box<dyn Error>> {
    let base_dir = libmc::util::get_base_dir()?;
    std::fs::create_dir_all(base_dir)?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::ListMinecraftVersions => {
            let versions = libmc::launchermeta::get_minecraft_versions()?;
            versions
                .iter()
                .for_each(|version| println!("{} {}", version.r#type, version.id));
        },
        Commands::Instance(i) => match &i.command {
            InstanceCommand::List => {
                let instances = libmc::instances::get_instance_list()?;
                instances
                    .iter()
                    .for_each(|instance| println!("{}", instance));
            }
        },
        Commands::Config => {
            let config_path = libmc::config::get_config_path()?;
            if !Path::is_file(&config_path) {
                libmc::config::new()?;
            }
            println!("config path: {:?}", &config_path);
        },
        Commands::Account(a) => match &a.command {
            AccountCommand::Add => {
                let (device_code, user_code, verification_uri) = libmc::accounts::authorize_device()?;
                println!("Go to: {}", verification_uri);
                println!("And enter this code: {}", user_code);
                authenticate(&device_code)?
            },
        }
    }

    Ok(())
}
