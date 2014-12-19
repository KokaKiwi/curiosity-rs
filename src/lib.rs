extern crate semver;
extern crate serialize;
extern crate syntax;

pub use stats::Stats;
pub use parse::parse;
use semver::Version;

pub mod stats;
mod parse;

pub fn version() -> Version {
    use semver::AlphaNumeric;

    let major = from_str(env!("CARGO_PKG_VERSION_MAJOR")).unwrap();
    let minor = from_str(env!("CARGO_PKG_VERSION_MINOR")).unwrap();
    let patch = from_str(env!("CARGO_PKG_VERSION_PATCH")).unwrap();
    let pre = match option_env!("CARGO_PKG_VERSION_PRE") {
        Some(pre) => vec![AlphaNumeric(pre.into_string())],
        None => vec![],
    };

    Version {
        major: major,
        minor: minor,
        patch: patch,
        pre: pre,
        build: Vec::new(),
    }
}
