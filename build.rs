use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::OsString;
use std::process::{Command, Stdio};

use serde::Deserialize;
use serde_json::from_slice;

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=Cargo.lock");
    let output = Command::new(var_os("CARGO")?)
        .args(["metadata", "--locked", "--format-version=1"])
        .current_dir(var_os("CARGO_MANIFEST_DIR")?)
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

    let metadata: Metadata<'_> = from_slice(&output.stdout).map_err(Error::FromSlice)?;
    let packages = metadata
        .packages
        .iter()
        .map(|p| (&*p.name, p))
        .collect::<HashMap<_, _>>();
    for p in &metadata.packages {
        if p.source.is_some() {
            continue;
        }
        let Some(name_suffix) = p.name.strip_prefix("tmpl-") else {
            continue;
        };
        for dep in &p.dependencies {
            let (dep_prefix, _) = dep.name.split_once('_').unwrap_or((&dep.name, ""));
            if dep_prefix != name_suffix {
                continue;
            }
            let Some(dep) = packages.get(&*dep.name) else {
                break;
            };
            let Some(source) = dep.source.as_deref() else {
                break;
            };
            match source
                .strip_prefix("git+")
                .and_then(|s| s.rsplit_once('#')?.1.get(..8))
            {
                Some(shorthash) => println!(
                    r#"cargo::rustc-env=VERSION_{n}={n} v{v} (git-{h})"#,
                    n = name_suffix,
                    v = dep.version,
                    h = shorthash,
                ),
                None => println!(
                    r#"cargo::rustc-env=VERSION_{n}={n} v{v}"#,
                    n = name_suffix,
                    v = dep.version,
                ),
            }
        }
    }
    Ok(())
}

#[derive(thiserror::Error, pretty_error_debug::Debug)]
enum Error {
    #[error("environment variable {:?} not found", 1)]
    Var(&'static str),
    #[error("could not spawn `cargo metadata`")]
    Spawn(#[source] std::io::Error),
    #[error("could not wait for `cargo metadata`")]
    Wait(#[source] std::io::Error),
    #[error("subprocess `cargo metadata` exited with an error: {}", .0)]
    Status(std::process::ExitStatus),
    #[error("could not parse `cargo metadata`'s output")]
    FromSlice(#[source] serde_json::Error),
}

fn var_os(key: &'static str) -> Result<OsString, Error> {
    std::env::var_os(key).ok_or(Error::Var(key))
}

#[derive(Debug, Deserialize)]
struct Metadata<'a> {
    #[serde(borrow)]
    packages: Vec<Package<'a>>,
}

#[derive(Debug, Deserialize)]
struct Package<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(borrow)]
    version: Cow<'a, str>,
    #[serde(borrow)]
    source: Option<Cow<'a, str>>,
    #[serde(borrow)]
    dependencies: Vec<Dependency<'a>>,
}

#[derive(Debug, Deserialize)]
struct Dependency<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
}
