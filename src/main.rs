use clap::Parser;
use std::error::Error;

#[derive(Parser)]
#[clap(
    version = "0.1.0",
    author = "Manuel Quarneti <manuelquarneti@gmail.com>"
)]
struct Opts {
    #[clap(short, long)]
    list_minecraft_versions: bool,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Instance(Instance),
}

/// A subcommand for controlling testing
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
    let opts: Opts = Opts::parse();

    if opts.list_minecraft_versions {
        let versions = libmc::launchermeta::get_minecraft_versions()?;
        versions
            .iter()
            .for_each(|version| println!("{} {}", version.r#type, version.id));
        return Ok(());
    }

    match opts.subcmd {
        SubCommand::Instance(i) => match i.subcmd {
            InstanceSubCommand::List => {
                let instances = libmc::instances::get_instance_list()?;
                instances
                    .iter()
                    .for_each(|instance| println!("{}", instance));
            }
        },
    }

    Ok(())
}
