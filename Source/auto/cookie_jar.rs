// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::{boxed::Box as Box_, fmt, mem::transmute};

use glib::{
	StaticType,
	object::{Cast, IsA},
	signal::{SignalHandlerId, connect_raw},
	translate::*,
};

use crate::Cookie;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
use crate::CookieJarAcceptPolicy;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use crate::SessionFeature;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use crate::URI;

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
glib::wrapper! {
	#[doc(alias = "SoupCookieJar")]
	pub struct CookieJar(Object<ffi::SoupCookieJar, ffi::SoupCookieJarClass>) @implements SessionFeature;

	match fn {
		type_ => || ffi::soup_cookie_jar_get_type(),
	}
}

#[cfg(not(any(feature = "v2_24", feature = "dox")))]
glib::wrapper! {
	#[doc(alias = "SoupCookieJar")]
	pub struct CookieJar(Object<ffi::SoupCookieJar, ffi::SoupCookieJarClass>);

	match fn {
		type_ => || ffi::soup_cookie_jar_get_type(),
	}
}

impl CookieJar {
	#[cfg(any(feature = "v2_24", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
	#[doc(alias = "soup_cookie_jar_new")]
	pub fn new() -> CookieJar {
		crate::assert_initialized_main_thread!();

		unsafe { from_glib_full(ffi::soup_cookie_jar_new()) }
	}
}

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
impl Default for CookieJar {
	fn default() -> Self { Self::new() }
}

pub const NONE_COOKIE_JAR:Option<&CookieJar> = None;

pub trait CookieJarExt: 'static {
	#[cfg(any(feature = "v2_26", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	#[doc(alias = "soup_cookie_jar_all_cookies")]
	fn all_cookies(&self) -> Vec<Cookie>;

	#[cfg(any(feature = "v2_26", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	#[doc(alias = "soup_cookie_jar_delete_cookie")]
	fn delete_cookie(&self, cookie:&mut Cookie);

	#[cfg(any(feature = "v2_30", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
	#[doc(alias = "soup_cookie_jar_get_accept_policy")]
	#[doc(alias = "get_accept_policy")]
	fn accept_policy(&self) -> CookieJarAcceptPolicy;

	#[cfg(any(feature = "v2_40", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
	#[doc(alias = "soup_cookie_jar_get_cookie_list")]
	#[doc(alias = "get_cookie_list")]
	fn cookie_list(&self, uri:&mut URI, for_http:bool) -> Vec<Cookie>;

	#[cfg(any(feature = "v2_24", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
	#[doc(alias = "soup_cookie_jar_get_cookies")]
	#[doc(alias = "get_cookies")]
	fn cookies(&self, uri:&mut URI, for_http:bool) -> Option<glib::GString>;

	#[cfg(any(feature = "v2_40", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
	#[doc(alias = "soup_cookie_jar_is_persistent")]
	fn is_persistent(&self) -> bool;

	#[cfg(any(feature = "v2_24", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
	#[doc(alias = "soup_cookie_jar_save")]
	fn save(&self);

	#[cfg(any(feature = "v2_30", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
	#[doc(alias = "soup_cookie_jar_set_accept_policy")]
	fn set_accept_policy(&self, policy:CookieJarAcceptPolicy);

	#[cfg(any(feature = "v2_24", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
	#[doc(alias = "soup_cookie_jar_set_cookie")]
	fn set_cookie(&self, uri:&mut URI, cookie:&str);

	#[cfg(any(feature = "v2_30", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
	#[doc(alias = "soup_cookie_jar_set_cookie_with_first_party")]
	fn set_cookie_with_first_party(&self, uri:&mut URI, first_party:&mut URI, cookie:&str);

	#[doc(alias = "read-only")]
	fn is_read_only(&self) -> bool;

	#[doc(alias = "changed")]
	fn connect_changed<F:Fn(&Self, &Cookie, &Cookie) + 'static>(&self, f:F) -> SignalHandlerId;

	#[cfg(any(feature = "v2_30", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
	#[doc(alias = "accept-policy")]
	fn connect_accept_policy_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;
}

impl<O:IsA<CookieJar>> CookieJarExt for O {
	#[cfg(any(feature = "v2_26", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	fn all_cookies(&self) -> Vec<Cookie> {
		unsafe {
			FromGlibPtrContainer::from_glib_full(ffi::soup_cookie_jar_all_cookies(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[cfg(any(feature = "v2_26", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	fn delete_cookie(&self, cookie:&mut Cookie) {
		unsafe {
			ffi::soup_cookie_jar_delete_cookie(
				self.as_ref().to_glib_none().0,
				cookie.to_glib_none_mut().0,
			);
		}
	}

	#[cfg(any(feature = "v2_30", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
	fn accept_policy(&self) -> CookieJarAcceptPolicy {
		unsafe { from_glib(ffi::soup_cookie_jar_get_accept_policy(self.as_ref().to_glib_none().0)) }
	}

	#[cfg(any(feature = "v2_40", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
	fn cookie_list(&self, uri:&mut URI, for_http:bool) -> Vec<Cookie> {
		unsafe {
			FromGlibPtrContainer::from_glib_full(ffi::soup_cookie_jar_get_cookie_list(
				self.as_ref().to_glib_none().0,
				uri.to_glib_none_mut().0,
				for_http.into_glib(),
			))
		}
	}

	#[cfg(any(feature = "v2_24", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
	fn cookies(&self, uri:&mut URI, for_http:bool) -> Option<glib::GString> {
		unsafe {
			from_glib_full(ffi::soup_cookie_jar_get_cookies(
				self.as_ref().to_glib_none().0,
				uri.to_glib_none_mut().0,
				for_http.into_glib(),
			))
		}
	}

	#[cfg(any(feature = "v2_40", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
	fn is_persistent(&self) -> bool {
		unsafe { from_glib(ffi::soup_cookie_jar_is_persistent(self.as_ref().to_glib_none().0)) }
	}

	#[cfg(any(feature = "v2_24", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
	fn save(&self) {
		unsafe {
			ffi::soup_cookie_jar_save(self.as_ref().to_glib_none().0);
		}
	}

	#[cfg(any(feature = "v2_30", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
	fn set_accept_policy(&self, policy:CookieJarAcceptPolicy) {
		unsafe {
			ffi::soup_cookie_jar_set_accept_policy(
				self.as_ref().to_glib_none().0,
				policy.into_glib(),
			);
		}
	}

	#[cfg(any(feature = "v2_24", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
	fn set_cookie(&self, uri:&mut URI, cookie:&str) {
		unsafe {
			ffi::soup_cookie_jar_set_cookie(
				self.as_ref().to_glib_none().0,
				uri.to_glib_none_mut().0,
				cookie.to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v2_30", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
	fn set_cookie_with_first_party(&self, uri:&mut URI, first_party:&mut URI, cookie:&str) {
		unsafe {
			ffi::soup_cookie_jar_set_cookie_with_first_party(
				self.as_ref().to_glib_none().0,
				uri.to_glib_none_mut().0,
				first_party.to_glib_none_mut().0,
				cookie.to_glib_none().0,
			);
		}
	}

	fn is_read_only(&self) -> bool {
		unsafe {
			let mut value = glib::Value::from_type(<bool as StaticType>::static_type());

			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"read-only\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);

			value.get().expect("Return Value for property `read-only` getter")
		}
	}

	fn connect_changed<F:Fn(&Self, &Cookie, &Cookie) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn changed_trampoline<
			P:IsA<CookieJar>,
			F:Fn(&P, &Cookie, &Cookie) + 'static,
		>(
			this:*mut ffi::SoupCookieJar,
			old_cookie:*mut ffi::SoupCookie,
			new_cookie:*mut ffi::SoupCookie,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);

			f(
				CookieJar::from_glib_borrow(this).unsafe_cast_ref(),
				&from_glib_borrow(old_cookie),
				&from_glib_borrow(new_cookie),
			)
		}

		unsafe {
			let f:Box_<F> = Box_::new(f);

			connect_raw(
				self.as_ptr() as *mut _,
				b"changed\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					changed_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg(any(feature = "v2_30", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
	fn connect_accept_policy_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_accept_policy_trampoline<P:IsA<CookieJar>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupCookieJar,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);

			f(CookieJar::from_glib_borrow(this).unsafe_cast_ref())
		}

		unsafe {
			let f:Box_<F> = Box_::new(f);

			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::accept-policy\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_accept_policy_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl fmt::Display for CookieJar {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("CookieJar") }
}
