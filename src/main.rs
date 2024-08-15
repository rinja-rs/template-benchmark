use std::borrow::Cow;
use std::env::args;
use std::fmt;
use std::str::{from_utf8, Utf8Error};

use tmpls::{Benchmark, BigTable, Output, Teams};

fn main() -> Result<(), Error> {
    let mut args = args().fuse();
    let exe = args.next().unwrap_or_default();
    let case = args.next().unwrap_or_default();
    let tmpl = args.next().unwrap_or_default();
    let end = args.next();

    if case.is_empty() || tmpl.is_empty() || end.is_some() {
        return Err(Error::Usage(match exe.is_empty() {
            true => Cow::Borrowed("template-benchmark"),
            false => Cow::Owned(exe),
        }));
    }

    let tmpl = TMPLS
        .iter()
        .find_map(|&(name, func)| (name == tmpl).then_some(func))
        .ok_or(Error::Tmpl(tmpl.into()))?;
    tmpl(Case::try_from(case)?)
}

#[derive(thiserror::Error, pretty_error_debug::Debug)]
enum Error {
    #[error("
Usage: {} <case> <tmpl>
Where
    <case>    is a test case: <big-table | teams>
    <tmpl>    is a templating library: <{}>", .0, Tmpls(3))]
    Usage(Cow<'static, str>),

    #[error("
Unknown test case: <{case}>
Expected: <big-table | teams>", case = .0)]
    Case(Cow<'static, str>),

    #[error("
Unknown template engine: <{}>
Expected one of: <{}>", .0, Tmpls(usize::MAX))]
    Tmpl(Cow<'static, str>),

    #[error("template rendering failed")]
    Execution(#[source] Box<dyn std::error::Error + Send + 'static>),

    #[error("template rendering generator non-UTF-8 data")]
    Utf8(#[source] Utf8Error),
}

enum Case {
    BigTable(BigTable),
    Teams(Teams),
}

impl TryFrom<String> for Case {
    type Error = Error;

    fn try_from(case: String) -> Result<Self, Self::Error> {
        match case.as_str() {
            "big-table" => Ok(Case::BigTable(BigTable::default())),
            "teams" => Ok(Case::Teams(Teams::default())),
            _ => Err(Error::Case(case.into())),
        }
    }
}

const TMPLS: &[(&str, fn(Case) -> Result<(), Error>)] = &[
    #[cfg(feature = "askama")]
    ("askama", tmpl::<askama::Benchmark>),
    #[cfg(feature = "handlebars")]
    ("handlebars", tmpl::<handlebars::Benchmark>),
    #[cfg(feature = "horrorshow")]
    ("horrorshow", tmpl::<horrorshow::Benchmark>),
    #[cfg(feature = "markup")]
    ("markup", tmpl::<markup::Benchmark>),
    #[cfg(feature = "maud")]
    ("maud", tmpl::<maud::Benchmark>),
    #[cfg(feature = "minijinja")]
    ("minijinja", tmpl::<minijinja::Benchmark>),
    #[cfg(feature = "rinja")]
    ("rinja", tmpl::<rinja::Benchmark>),
    #[cfg(feature = "rinja_git")]
    ("rinja_git", tmpl::<rinja_git::Benchmark>),
    #[cfg(feature = "ructe")]
    ("ructe", tmpl::<ructe::Benchmark>),
    #[cfg(feature = "sailfish")]
    ("sailfish", tmpl::<sailfish::Benchmark>),
    #[cfg(feature = "tera")]
    ("tera", tmpl::<tera::Benchmark>),
    #[cfg(feature = "tinytemplate")]
    ("tinytemplate", tmpl::<tinytemplate::Benchmark>),
];

struct Tmpls(usize);

impl fmt::Display for Tmpls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, &(name, _)) in TMPLS.iter().enumerate() {
            if idx >= self.0 {
                return write!(f, "...");
            }
            match idx {
                0 => write!(f, "{name}"),
                _ => write!(f, " | {name}"),
            }?;
        }
        Ok(())
    }
}

fn tmpl<B: Benchmark>(case: Case) -> Result<(), Error> {
    let mut tmpl = B::default();
    let mut output = B::Output::default();
    let result = match case {
        Case::Teams(input) => tmpl.teams(&mut output, &input),
        Case::BigTable(input) => tmpl.big_table(&mut output, &input),
    };
    result.map_err(|err| Error::Execution(Box::new(err)))?;

    let bytes = from_utf8(output.as_bytes()).map_err(Error::Utf8)?;
    println!("{}", bytes);

    Ok(())
}
