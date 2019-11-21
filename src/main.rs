use git2::{BranchType, Repository, ObjectType, Blob};
use regex::Regex;
use std::io::{self, Write};
use serde_json;
use std::collections::HashMap;
use std::fs;
use termion::color::{self, Fg};

const CONFIG_FILE: &str = "rules.json";

fn main() {

    let info: String = format!("{}[INFO]{}", Fg(color::Green), Fg(color::Reset));

    // Get config string
    let conf_str = fs::read_to_string(CONFIG_FILE).unwrap();

    // Make a hashmap of uncompiled regex expressions
    let conf: HashMap<String, String> = serde_json::from_str(&conf_str).unwrap();

    // Get path to git repo via command line args or assume current directory
    let repo_root = std::env::args().nth(1).unwrap_or(".".to_string());

    // Open git repo
    let repo = Repository::open(repo_root.as_str()).expect("Couldn't open repository");

    println!("{} checking {} key templates", info, conf.len());

    for branch in repo.branches(Some(BranchType::Local)).unwrap() {
        // This is not what rust code should look like
        println!(
            "{} Scanning branch {}",
            info,
            branch.unwrap().0.name().unwrap().unwrap()
        );
    }

    // Print the current start of the git repo
    println!("{} {} state={:?}", info, repo.path().display(), repo.state());

    let odb = repo.odb().unwrap();
    odb.foreach(|oid| {
        // println!("{}",oid);
        let obj = repo.revparse_single(&oid.to_string()).unwrap();
        // println!("{} {}\n--", obj.kind().unwrap().str(), obj.id());
        match obj.kind() {
            Some(ObjectType::Blob) => {
                show_blob(obj.as_blob().unwrap());
            }
            _ => () // only care about the blobs so ignore anything else.
        }
        true
    })
    .unwrap();
}

fn is_bad(maybe: &str, bads: &HashMap<String, String>) -> Option<String> {
    for (key, val) in bads {
        let re = Regex::new(val).unwrap();
        if re.is_match(maybe) {
            return Some(key.to_string())
        }
    }

    None
}

fn show_blob(blob: &Blob) {
    io::stdout().write_all(blob.content()).unwrap();
}

