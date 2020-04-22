use vfs_service::{run, SingleService};

mod gh_fs;
use gh_fs::GithubService;

fn main() {
    let gh = Box::new(GithubService {});
    let svcs: Vec<Box<dyn SingleService + Send>> = vec![gh];
    run(svcs);
}
