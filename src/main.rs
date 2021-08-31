use::git2::{Repository};
use std::env;
use regex::Regex;
use open;

const BASE_URL: &str = "https://ramseysolutions.atlassian.net/browse/";

fn main() {
    let cwd =  match env::current_dir() {
        Ok(r) => r,
        Err(e) => panic!("Can't get current directory: {}", e),
    };

    let repo = match Repository::open(cwd) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to get repo: {}", e), 
    };

   let head = match repo.head() {
       Ok(h) => h,
       Err(e) => panic!("Erorr getting head: {}", e)
   };

   let head_name = match head.name() {
       Some(br) => br,
       None => panic!("Can't get branch ref")
   };

   let re = Regex::new(r"refs/heads/").unwrap();

   let branch_name = re.replace(head_name, "");

   println!("{}", branch_name);

   open::that(BASE_URL).unwrap();
}
