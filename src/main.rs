use std::{path::Path, process::Command, thread::sleep, time::Duration};

use notify::{RecursiveMode, Result, Watcher};

fn main() -> Result<()> {
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
           Ok(_) => {
            println!("adding to repository");
            Command::new("git").args(["add","-A"]).output().expect("failed to add to repository");
            Command::new("git").args(["commit","-m"]).output().expect("failed to commit to repository");
            Command::new("git").arg("push").output().expect("failed to push to remote");// no way to spawn hundrends of threads if it requires password :3
           },
           Err(e) => println!("watch error: {:?}", e),
        }
    })?;
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;
    sleep(Duration::MAX);
    
    Ok(())
}