use clap::{ArgMatches, Command};
use crate::lib;

pub mod subcommands;

pub fn get_matches() -> ArgMatches {
    Command::new("eaglemaster")
        .about("Tool for managing Eagle")
        .version("0.1.0")
        .author("Oleksii Luchnikov <oleksiiluchnikov@gmail.com>")
        .arg_required_else_help(true)

        .subcommand(subcommands::app::build())
        .subcommand(subcommands::folder::build())
        .subcommand(subcommands::item::build())
        .subcommand(subcommands::library::build())
        .get_matches()
}

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let matches = get_matches();
    let eagle_client = lib::client::EagleClient::new("localhost", 41595);

    // Handle rename subcommand
    match matches.subcommand() {
        Some(("app", app_matches)) => {
            subcommands::app::execute(&eagle_client, app_matches).await?;
        },
        Some(("folder", folder_matches)) => {
            subcommands::folder::execute(&eagle_client, folder_matches).await?;
        },
        // Some(("item", item_matches)) => {
        //     subcommands::item::execute(eagle_client, item_matches).await?;
        // },
        Some(("library", library_matches)) => {
            subcommands::library::execute(&eagle_client, library_matches).await?;
        },
        _ => {
            println!("No subcommand was used");
        }    }
    Ok(())
}
