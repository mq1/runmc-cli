use clap::Parser;
use std::{error::Error, path::Path};

#[derive(Parser)]
#[clap(
    version = "0.1.0",
    author = "Manuel Quarneti <manuelquarneti@gmail.com>"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    ListMinecraftVersions,
    Instance(Instance),
    Config,
}

#[derive(Parser)]
struct Instance {
    #[clap(subcommand)]
    subcmd: InstanceSubCommand,
}

#[derive(Parser)]
enum InstanceSubCommand {
    List,
}

fn main() -> Result<(), Box<dyn Error>> {
    let base_dir = libmc::util::get_base_dir()?;
    std::fs::create_dir_all(base_dir)?;

    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::ListMinecraftVersions => {
            let versions = libmc::launchermeta::get_minecraft_versions()?;
            versions
                .iter()
                .for_each(|version| println!("{} {}", version.r#type, version.id));
        },
        SubCommand::Instance(i) => match i.subcmd {
            InstanceSubCommand::List => {
                let instances = libmc::instances::get_instance_list()?;
                instances
                    .iter()
                    .for_each(|instance| println!("{}", instance));
            }
        },
        SubCommand::Config => {
            let config_path = libmc::config::get_config_path()?;
            if !Path::is_file(&config_path) {
                libmc::config::new()?;
            }
            open::that(config_path)?;
        }
    }

    Ok(())
}
