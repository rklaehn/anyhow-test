use anyhow::{self, Context}; // 1.0.4
use derive_more::{Display, Error};

fn fnord() -> anyhow::Result<()> {
    anyhow::bail!("FNORD!");
}

fn bar() -> anyhow::Result<()> {
    fnord()
}

fn foo() -> anyhow::Result<()> {
    bar()
}

// the wrapper approach
#[derive(Debug, Display)]
struct WrappingError(anyhow::Error);

impl std::error::Error for WrappingError {}

// the tag approach
#[derive(Debug, Display, Error)]
struct TaggingError;

fn main() {
    let res1: anyhow::Error = foo().context(TaggingError).unwrap_err();
    let res2: anyhow::Error = foo().map_err(WrappingError).map_err(Into::into).unwrap_err();
    assert!(res1.downcast_ref::<TaggingError>().is_some());
    assert!(res2.downcast_ref::<WrappingError>().is_some());
    println!("{}", res1.backtrace());
    println!("{}", res2.backtrace());
}