use clap::{App, load_yaml};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if matches.is_present("list-minecraft-versions") {
        let versions = libmc::launchermeta::get_minecraft_versions()?;
        versions.into_iter().for_each(|version| println!("{} {}", version.r#type, version.id));
    }

    Ok(())
}
