use std::{collections::HashMap, fmt::Display, fs::OpenOptions, io::Write, path::PathBuf};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
enum Args {
    #[clap(name = "add", about = "Add a new item to the list")]
    Add { name: String, content: String },
    #[clap(name = "edit", about = "Edit an existing item in the list")]
    Edit { name: String, content: String },
    #[clap(name = "remove", about = "Remove an item from the list")]
    Remove { name: String },
    #[clap(name = "list", about = "List all items in the list")]
    List,
    #[clap(name = "find", about = "Find an item in the list")]
    Find { name: String },
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    content: String,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl Item {
    fn new(content: String) -> Self {
        Self { content }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Items {
    items: HashMap<String, Item>,
    save_path: PathBuf,
}

impl Items {
    fn on_args(&mut self, args: Args) {
        match args {
            Args::Add { name, content } => {
                match self.items.insert(name.clone(), Item::new(content)) {
                    Some(_) => {
                        println!("already exists: {name} ");
                    }
                    None => {
                        println!("added: {name} ");
                    }
                }
            }
            Args::Edit { name, content } => match self.items.get_mut(&name) {
                Some(item) => {
                    item.content = content;
                    println!("updated: {name}: {item}");
                }
                None => {
                    println!("not found: {name}");
                }
            },
            Args::Remove { name } => match self.items.remove(&name) {
                Some(item) => {
                    println!("removed: {name}: {item}");
                }
                None => {
                    println!("not found: {name}");
                    let mut iter = self.find(&name).peekable();
                    if let Some(_) = iter.peek() {
                        println!("Did you mean:");
                        for (index, (items_name, _)) in iter.enumerate() {
                            println!("{}: {}", index + 1, items_name);
                        }
                    }
                }
            },
            Args::List => {
                if self.items.is_empty() {
                    println!("No items in the list");
                    return;
                }
                for (name, item) in self.items.iter() {
                    println!("{}: {}", name, item.content);
                }
            }
            Args::Find { name } => {
                let mut iter = self.find(&name).peekable();
                match iter.peek() {
                    Some(_) => {
                        for (items_name, item) in iter {
                            println!("{}: {}", items_name, item.content);
                        }
                    }
                    None => {
                        println!("not found: {name}");
                    }
                }
            }
        }
    }
    fn find(&self, name: &str) -> impl Iterator<Item = (&String, &Item)> {
        let name = name.trim().to_lowercase();
        self.items
            .iter()
            .filter(move |(item_name, _)| item_name.trim().to_lowercase().contains(&name))
    }
    fn load_from_file(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        if path.exists() {
            let mut file = OpenOptions::new().read(true).open(&path)?;

            Ok(Self {
                items: bincode::serde::decode_from_std_read(
                    &mut file,
                    bincode::config::standard(),
                )?,
                save_path: path,
            })
        } else {
            Ok(Self {
                items: HashMap::new(),
                save_path: path,
            })
        }
    }
    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.save_path)?;
        let buffer = bincode::serde::encode_to_vec(self, bincode::config::standard())?;
        file.write(&buffer)?;
        Ok(())
    }
}

impl Drop for Items {
    fn drop(&mut self) {
        self.save_to_file().expect("Failed to save items to file");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut items: Items = Items::load_from_file("items.bin".into())?;
    items.on_args(args);
    Ok(())
}
