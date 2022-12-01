use std::process::{Command, Stdio};
use clap::{Arg, App};
use std::string::String;

fn main() {

    let matches = cli().get_matches();

    let fetch_package = matches.value_of("install").unwrap();

    let fmt_pkg = format!("https://aur.archlinux.org/{}.git", fetch_package);
    let out_dir = "/tmp/otter-aur";
    fetch(fmt_pkg, out_dir).expect("Cant run install phase");

    install().expect("Cant run install phase");

    post_remove(out_dir).expect("Cant run install phase");
}

fn fetch(pkg: String, dir: &str) -> Result<(), Box<dyn std::error::Error>> {

    let mut command = Command::new("git");

    command
        .args(["clone", &pkg, dir])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let output = command.output()?;

    if !output.status.success() {
        return Err("fetch failed to run".into());
    }

    Ok(())

}

fn install() -> Result<(), Box<dyn std::error::Error>>  {
    let mut command = Command::new("makepkg");

    command
        .args(["-si"])
        .current_dir("/tmp/otter-aur")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let output = command.output()?;

    if !output.status.success() {
        return Err("instalation failed to run".into());
    }

    Ok(())
}

fn post_remove(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::new("rm");

    command
        .args(["-rf", "otter-aur"])
        .current_dir(dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let output = command.output()?;

    if !output.status.success() {
        return Err("post-remove failed to run".into());
    }

    Ok(())
}

fn cli<'help>() -> App<'help> {
    App::new("Otter aur helper")
        .version("0.1-alpha")
        .author("Guilherme Menezes <thechibbis@protonmail.com>")
        .about("otter is a simple aur helper")
        .arg(Arg::with_name("install")
            .short('S')
            .long("install")
            .value_name("PACKAGE")
            .help("Fetch a package"))
}
