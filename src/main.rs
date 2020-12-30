use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt, Debug)]
#[structopt(name = "pier", about="pier related interaction")]
struct Pier {
    #[structopt(name = "repo", short = "r", parse(from_os_str))]
    repo: std::path::PathBuf,
    #[structopt(subcommand)]
    cmd: PierCmd,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "the stupid content tracker")]
enum PierCmd {
    #[structopt(name = "id")]
    Id(Id),
    #[structopt(name = "init")]
    Init(Init),
}

#[derive(Debug, StructOpt)]
struct Init {
    /// The repo path to init
    repo: Option<String>,
}

#[derive(Debug, StructOpt)]
struct Id {
    /// The repo path to get id
    repo: Option<String>,
}

fn main() -> Result<()> {
    let pier = Pier::from_args();
    let repo = pier.repo.into_os_string().into_string().unwrap();
    match &pier.cmd {
        PierCmd::Id (cmd) => getPierID(cmd, &repo)?,
        PierCmd::Init(cmd) => initPier(cmd, &repo)?,
        _ => {}
    }
    Ok(())
}

fn initPier(cmd: &Init, repo: &String) -> Result<()> {
    let cmdRepo = cmd.repo.as_ref().expect("wrong repo");
    println!("this is init function for repo {} and {}", repo, cmdRepo);
    Ok(())
}

fn getPierID(cmd: &Id, repo: &String) -> Result<()> {
    let cmdRepo = cmd.repo.as_ref().expect("wrong repo");
    println!("this is id function for repo {} and {}", repo, cmdRepo);
    Ok(())
}