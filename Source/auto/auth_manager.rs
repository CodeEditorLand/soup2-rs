// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::{boxed::Box as Box_, fmt, mem::transmute};

use glib::{
	object::{Cast, IsA},
	signal::{connect_raw, SignalHandlerId},
	translate::*,
};

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use crate::SessionFeature;
#[cfg(any(feature = "v2_42", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_42")))]
use crate::URI;
use crate::{Auth, Message};

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
glib::wrapper! {
	#[doc(alias = "SoupAuthManager")]
	pub struct AuthManager(Object<ffi::SoupAuthManager, ffi::SoupAuthManagerClass>) @implements SessionFeature;

	match fn {
		type_ => || ffi::soup_auth_manager_get_type(),
	}
}

#[cfg(not(any(feature = "v2_24", feature = "dox")))]
glib::wrapper! {
	#[doc(alias = "SoupAuthManager")]
	pub struct AuthManager(Object<ffi::SoupAuthManager, ffi::SoupAuthManagerClass>);

	match fn {
		type_ => || ffi::soup_auth_manager_get_type(),
	}
}

pub const NONE_AUTH_MANAGER:Option<&AuthManager> = None;

pub trait AuthManagerExt: 'static {
	#[cfg(any(feature = "v2_58", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
	#[doc(alias = "soup_auth_manager_clear_cached_credentials")]
	fn clear_cached_credentials(&self);

	#[cfg(any(feature = "v2_42", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_42")))]
	#[doc(alias = "soup_auth_manager_use_auth")]
	fn use_auth(&self, uri:&mut URI, auth:&impl IsA<Auth>);

	#[doc(alias = "authenticate")]
	fn connect_authenticate<F:Fn(&Self, &Message, &Auth, bool) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId;
}

impl<O:IsA<AuthManager>> AuthManagerExt for O {
	#[cfg(any(feature = "v2_58", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
	fn clear_cached_credentials(&self) {
		unsafe {
			ffi::soup_auth_manager_clear_cached_credentials(self.as_ref().to_glib_none().0);
		}
	}

	#[cfg(any(feature = "v2_42", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_42")))]
	fn use_auth(&self, uri:&mut URI, auth:&impl IsA<Auth>) {
		unsafe {
			ffi::soup_auth_manager_use_auth(
				self.as_ref().to_glib_none().0,
				uri.to_glib_none_mut().0,
				auth.as_ref().to_glib_none().0,
			);
		}
	}

	fn connect_authenticate<F:Fn(&Self, &Message, &Auth, bool) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId {
		unsafe extern fn authenticate_trampoline<
			P:IsA<AuthManager>,
			F:Fn(&P, &Message, &Auth, bool) + 'static,
		>(
			this:*mut ffi::SoupAuthManager,
			msg:*mut ffi::SoupMessage,
			auth:*mut ffi::SoupAuth,
			retrying:glib::ffi::gboolean,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(
				AuthManager::from_glib_borrow(this).unsafe_cast_ref(),
				&from_glib_borrow(msg),
				&from_glib_borrow(auth),
				from_glib(retrying),
			)
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"authenticate\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					authenticate_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl fmt::Display for AuthManager {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("AuthManager") }
}
