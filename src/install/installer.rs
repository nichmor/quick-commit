use miette::{self, IntoDiagnostic};
use std::{fs, io::Write, os::unix::fs::OpenOptionsExt, process::Command};

use minijinja::{context, Environment};

pub fn get_python_exec() -> miette::Result<String> {
    let output = Command::new("python3")
        .args(["-c", "import sys; print(sys.executable)"])
        .output()
        .expect("failed to execute process");
    let stringify = String::from_utf8(output.stdout).into_diagnostic()?;
    Ok(stringify)
}

pub fn get_current_git_dir() -> miette::Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-common-dir"])
        .current_dir(".")
        .output()
        .expect("failed to execute process");

    let stringify = String::from_utf8(output.stdout).into_diagnostic()?;

    Ok(stringify)
}

pub fn install_hook() -> miette::Result<()> {
    let python_loc = get_python_exec()?;

    let mut env = Environment::new();

    let contents = fs::read_to_string("src/templates/hook.sh.tmpl")
        .expect("Should have been able to read the file");

    env.add_template("hook_tmpl", &contents).unwrap();

    // env.get_template(name)
    let tmpl = env.get_template("hook_tmpl").unwrap();
    let rendered_tmpl = tmpl
        .render(context!(python_location => python_loc, hook_args => "some_args"))
        .into_diagnostic()?;

    let git_location = get_current_git_dir()?;

    let git_location_trimmed = git_location.trim();

    let pre_commit_path = format!("{git_location_trimmed}/hooks/pre-commit");

    println!("pre-commit-path {}", pre_commit_path);

    let mut pre_commit_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o770)
        .open(pre_commit_path)
        .into_diagnostic()?;

    pre_commit_file
        .write_all(rendered_tmpl.as_bytes())
        .into_diagnostic()?;

    println!("Hook installed!");

    Ok(())
}
