use::git2::{Repository, BranchType};
use std::env;

fn main() {
    let cwd =  match env::current_dir() {
        Ok(r) => r,
        Err(e) => panic!("Can't get current directory: {}", e),
    };

    println!("{}", cwd.display());

    let repo = match Repository::open(cwd) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to get repo: {}", e), 
    };

    let branch = match repo.find_branch("master", BranchType::Remote) {

        Ok(b) => b,
        Err(e) => panic!("Failed to get branch: {}", e), 
    };

    println!("{:#?}", branch.name());

}
