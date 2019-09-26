extern crate failure;

use std::{process, io::{stderr, stdout, Write}};
use failure::Error;

/// Given an `Error`, print all the causes to `w`
pub fn print_causes(e: impl Into<Error>, mut w: impl Write) {
    let e = e.into();
    let causes = e.causes().collect::<Vec<_>>();
    let num_causes = causes.len();
    for (index, cause) in causes.iter().enumerate() {
        if index == 0 {
            writeln!(w, "{}", cause).ok();
            if num_causes > 1 {
                writeln!(w, "Caused by: ").ok();
            }
        } else {
            writeln!(w, " {}: {}", num_causes - index, cause).ok();
        }
    }
}

/// If the `Result` is `Ok(v)`, return `v`. Otherwise `print_causes()`
/// and exit with exit code 1.
pub fn ok_or_exit<T, E>(r: Result<T, E>) -> T
where
    E: Into<Error>,
{
    match r {
        Ok(r) => r,
        Err(e) => {
            stdout().flush().ok();
            write!(stderr(), "error: ").ok();
            print_causes(e, stderr());
            process::exit(1);
        }
    }
}
