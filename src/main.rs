use clap::{Parser, Subcommand};
use libmc::accounts::authenticate;
use std::{error::Error, fs};

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
    Account(Account),
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
    Add,
    List,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ListMinecraftVersions => {
            let versions = libmc::launchermeta::get_minecraft_versions()?;
            versions
                .iter()
                .for_each(|version| println!("{} {}", version.r#type, version.id));
        }
        Commands::Instance(i) => match &i.command {
            InstanceCommand::List => {
                let instances = libmc::instances::get_instance_list()?;
                instances
                    .iter()
                    .for_each(|instance| println!("{}", instance));
            }
        },
        Commands::Config => {
            let _config = libmc::config::read()?;
            println!("config path: {:?}", libmc::config::CONFIG_PATH.as_path());
        }
        Commands::Account(a) => match &a.command {
            AccountCommand::Add => {
                let (device_code, user_code, verification_uri) =
                    libmc::accounts::authorize_device()?;
                println!("Go to: {}", verification_uri);
                println!("And enter this code: {}", user_code);
                authenticate(&device_code)?
            }
            AccountCommand::List => {
                let user_profiles = libmc::accounts::list_user_profiles()?;
                for user_profile in user_profiles {
                    println!("{}", user_profile.name);
                }
            }
        },
    }

    Ok(())
}
