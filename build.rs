use std::ffi::{OsStr, OsString};
use std::process::{Command, Stdio};

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=Cargo.lock");

    let cargo = var_os("CARGO")?;
    let root = var_os("CARGO_MANIFEST_DIR")?;
    for &(benchmark_name, _) in &cargo_tree(&cargo, &root, "template-benchmark")? {
        let Some(engine_name) = benchmark_name.strip_prefix("tmpl-") else {
            continue;
        };
        let bare_engine_name = engine_name.strip_suffix("_git").unwrap_or(engine_name);

        for &(name, info) in &cargo_tree(&cargo, &root, benchmark_name)? {
            if name != bare_engine_name {
                continue;
            }
            if let Some((version, hash)) = info
                .split_once(' ')
                .and_then(|(v, s)| Some((v, s.rsplit_once('#')?)))
                .and_then(|(v, (_, s))| Some((v, s.strip_suffix(')')?)))
            {
                println!(
                    r#"cargo::rustc-env=VERSION_{p}={n} {v} (git-{h})"#,
                    p = engine_name,
                    n = name,
                    v = version,
                    h = hash,
                )
            } else {
                println!(
                    r#"cargo::rustc-env=VERSION_{p}={n} {v}"#,
                    p = engine_name,
                    n = name,
                    v = info,
                )
            }
            break;
        }
    }
    Ok(())
}

fn var_os(key: &'static str) -> Result<OsString, Error> {
    std::env::var_os(key).ok_or(Error::Var(key))
}

fn cargo_tree(cargo: &OsStr, root: &OsStr, package: &str) -> Result<CargoTreeOutput, Error> {
    let output = Command::new(cargo)
        .args([
            "tree",
            "--locked",
            "--edges=normal",
            "--depth=1",
            "--charset=ascii",
            "--all-features",
            "--package",
            package,
        ])
        .current_dir(root)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(Error::Spawn)?
        .wait_with_output()
        .map_err(Error::Wait)?;
    if !output.status.success() {
        return Err(Error::Status(output.status));
    }

    Ok(CargoTreeOutput::new(
        String::from_utf8(output.stdout).map_err(|_| Error::Utf8)?,
        |s| {
            s.lines()
                .filter_map(|l| (l.get(1..4)? == "-- ").then(|| l.get(4..)?.split_once(' '))?)
                .collect()
        },
    ))
}

self_cell::self_cell! {
    struct CargoTreeOutput {
        owner: String,
        #[covariant]
        dependent: CargoTreeOutputLines,
    }
}

impl<'a> IntoIterator for &'a CargoTreeOutput {
    type Item = <&'a CargoTreeOutputLines<'a> as IntoIterator>::Item;
    type IntoIter = <&'a CargoTreeOutputLines<'a> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.borrow_dependent().iter()
    }
}

type CargoTreeOutputLines<'a> = Vec<(&'a str, &'a str)>;

#[derive(thiserror::Error, pretty_error_debug::Debug)]
enum Error {
    #[error("environment variable {:?} not found", 1)]
    Var(&'static str),
    #[error("could not spawn `cargo`")]
    Spawn(#[source] std::io::Error),
    #[error("could not wait for `cargo`")]
    Wait(#[source] std::io::Error),
    #[error("`cargo` exited with an error: {}", .0)]
    Status(std::process::ExitStatus),
    #[error("`cargo` returned non-UTF-8 data")]
    Utf8,
}
