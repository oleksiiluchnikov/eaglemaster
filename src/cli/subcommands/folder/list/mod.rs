use crate::lib::client::EagleClient;
use clap::ArgMatches;
use crate::lib::types::Child;

// Arguments
pub mod args;

pub struct ListCommand {
    root: String,
    options: ListOptions,
}

pub struct ListOptions {
    recursive: bool,
    tree: bool,
    nesting_level: u8,
}

impl ListOptions {
    pub fn new() -> Self {
        ListOptions {
            recursive: false,
            tree: false,
            nesting_level: 0,
        }
    }
}

pub async fn execute(
    client: &EagleClient,
    matches: &ArgMatches,
    ) -> Result<(), Box<dyn std::error::Error>> {

    let data: Vec<Child> = client.folder().list().await?.data;

    if matches.get_flag("tree") {
        args::tree::execute(&data, &ListOptions {
            recursive: matches.get_flag("recursive"),
            tree: matches.get_flag("tree"),
            nesting_level: 0,
        })?;
        return Ok(());
    }

    if matches.get_flag("recursive") {
        // let nesting_level = matches.get_one::<u8>("nesting-level")?;
        print_recursive(&data, 0);
        return Ok(());
    }

    Ok(())
}

fn print_recursive(data: &Vec<Child>, mut nesting_level: u8) {
    for child in data {
        println!("{}", child.name);
        if child.children.len() > 0 {
            nesting_level += 1;
            print_recursive(&child.children, nesting_level);
        }
    }
}


// Recursive function to find duplicate folder names among siblings (having the same parent)
fn find_duplicates(data: &Vec<Child>, duplicate_folder_names: &mut Vec<String>) {
    todo!()
}

