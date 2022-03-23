use anyhow::Result;
use clap::{Parser, Subcommand};

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
    Remove { name: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ListMinecraftVersions => {
            let versions = mc::launchermeta::get_minecraft_versions()?;
            versions
                .iter()
                .for_each(|version| println!("{} {}", version.r#type, version.id));
        }
        Commands::Instance(i) => match &i.command {
            InstanceCommand::List => {
                let instances = mc::instances::get_instance_list()?;
                instances
                    .iter()
                    .for_each(|instance| println!("{}", instance));
            }
        },
        Commands::Config => {
            let _ = mc::config::read()?;
            println!("config path: {:?}", mc::config::CONFIG_PATH.as_path());
        }
        Commands::Account(a) => match &a.command {
            AccountCommand::Add => {
                let auth_url = mc::accounts::get_auth_url()?;
                println!("Go to: {}", auth_url);

                mc::accounts::add()?;
            }
            AccountCommand::List => {
                for account in mc::accounts::list()? {
                    println!("{}", account);
                }
            }
            AccountCommand::Remove { name } => {
                mc::accounts::remove(name)?;
            }
        },
    }

    Ok(())
}
