// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::fmt;

use glib::{object::Cast, translate::*};

use crate::Session;

glib::wrapper! {
	#[doc(alias = "SoupSessionAsync")]
	pub struct SessionAsync(Object<ffi::SoupSessionAsync, ffi::SoupSessionAsyncClass>) @extends Session;

	match fn {
		type_ => || ffi::soup_session_async_get_type(),
	}
}

impl SessionAsync {
	#[doc(alias = "soup_session_async_new")]
	pub fn new() -> SessionAsync {
		crate::assert_initialized_main_thread!();

		unsafe { Session::from_glib_full(ffi::soup_session_async_new()).unsafe_cast() }
	}

	//#[doc(alias = "soup_session_async_new_with_options")]
	//#[doc(alias = "new_with_options")]
	// pub fn with_options(optname1: &str, : /*Unknown
	// conversion*//*Unimplemented*/Fundamental: VarArgs) -> SessionAsync {
	//    unsafe { TODO: call ffi:soup_session_async_new_with_options() }
	//}
}

impl Default for SessionAsync {
	fn default() -> Self { Self::new() }
}

pub const NONE_SESSION_ASYNC:Option<&SessionAsync> = None;

impl fmt::Display for SessionAsync {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("SessionAsync") }
}
