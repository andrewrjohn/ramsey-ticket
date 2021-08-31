use::git2::{Repository, BranchType};
use std::env;
use std::process::Command;

fn main() {
    // let output = Command::new("git")
    // .arg("rev-parse")
    // .arg("--abbrev-ref")
    // .arg("HEAD")
    // .output()
    // .expect("failed");

    // let current_branch = String::from_utf8_lossy(&output.stdout);

    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));

    let cwd =  match env::current_dir() {
        Ok(r) => r,
        Err(e) => panic!("Can't get current directory: {}", e),
    };


    let repo = match Repository::open(cwd) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to get repo: {}", e), 
    };


   let rev_parse = match repo.revparse_single("HEAD") {
       Ok(rp) => println!("{:#?}", rp),
       Err(err) => println!("Error: {}", err)
   };




    // let branch = match repo.find_branch("master", BranchType::Local) {

    //     Ok(b) => b,
    //     Err(e) => panic!("Failed to get branch: {}", e), 
    // };
    
    // let branches = match repo.branches(Some(BranchType::Local)) {

    //     Ok(b) => b,
    //     Err(e) => panic!("Failed to get branch: {}", e), 
    // };

    // let trees = match repo.worktrees() {
    //     Ok(t) => t,
    //     Err(e) => panic!("Error: {}", e)
    // };

    // for t in trees.iter() {

    //     println!("{}", t[0]);
    // }


    // for b in branches {
    //     match b {
    //         Ok(b) => println!("{:?}", b.name()),

    //     };
    // };

    // let last_branch =  match branches.last() {
    //     Some(lb) => lb,
    //     // None => println!("")
    // };

    // match last_branch {
    //     Ok(b) => println!("{}", b[0])
    // }
    // println!("{:#?}", branch.);
    // println!("{:?}", last_branch.name());

}
