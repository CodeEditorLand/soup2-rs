// Generated by gir (https://github.com/gtk-rs/gir @ 05fe12c0b7e7)
// from ../gir-files (@ 7182204ef108)
// DO NOT EDIT

#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
	if let Err(s) = system_deps::Config::new().probe() {
		println!("cargo:warning={}", s);

		process::exit(1);
	}
}
