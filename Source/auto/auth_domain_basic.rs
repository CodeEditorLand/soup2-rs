// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::{boxed::Box as Box_, fmt, mem::transmute};

use glib::{
	object::{Cast, IsA},
	signal::{connect_raw, SignalHandlerId},
	translate::*,
};

use crate::{AuthDomain, Message};

glib::wrapper! {
	#[doc(alias = "SoupAuthDomainBasic")]
	pub struct AuthDomainBasic(Object<ffi::SoupAuthDomainBasic, ffi::SoupAuthDomainBasicClass>) @extends AuthDomain;

	match fn {
		type_ => || ffi::soup_auth_domain_basic_get_type(),
	}
}

impl AuthDomainBasic {
	//#[doc(alias = "soup_auth_domain_basic_new")]
	// pub fn new(optname1: &str, : /*Unknown
	// conversion*//*Unimplemented*/Fundamental: VarArgs) -> AuthDomainBasic {
	//    unsafe { TODO: call ffi:soup_auth_domain_basic_new() }
	//}
}

pub const NONE_AUTH_DOMAIN_BASIC:Option<&AuthDomainBasic> = None;

pub trait AuthDomainBasicExt: 'static {
	#[doc(alias = "soup_auth_domain_basic_set_auth_callback")]
	fn set_auth_callback<P:Fn(&AuthDomainBasic, &Message, &str, &str) -> bool + 'static>(
		&self,
		callback:P,
	);

	//#[doc(alias = "auth-data")]
	// fn auth_data(&self) -> /*Unimplemented*/Fundamental: Pointer;

	//#[doc(alias = "auth-data")]
	// fn set_auth_data(&self, auth_data: /*Unimplemented*/Fundamental: Pointer);

	#[doc(alias = "auth-data")]
	fn connect_auth_data_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;
}

impl<O:IsA<AuthDomainBasic>> AuthDomainBasicExt for O {
	fn set_auth_callback<P:Fn(&AuthDomainBasic, &Message, &str, &str) -> bool + 'static>(
		&self,
		callback:P,
	) {
		let callback_data:Box_<P> = Box_::new(callback);

		unsafe extern fn callback_func<
			P:Fn(&AuthDomainBasic, &Message, &str, &str) -> bool + 'static,
		>(
			domain:*mut ffi::SoupAuthDomainBasic,
			msg:*mut ffi::SoupMessage,
			username:*const libc::c_char,
			password:*const libc::c_char,
			user_data:glib::ffi::gpointer,
		) -> glib::ffi::gboolean {
			let domain = from_glib_borrow(domain);

			let msg = from_glib_borrow(msg);

			let username:Borrowed<glib::GString> = from_glib_borrow(username);

			let password:Borrowed<glib::GString> = from_glib_borrow(password);

			let callback:&P = &*(user_data as *mut _);

			let res = (*callback)(&domain, &msg, username.as_str(), password.as_str());

			res.into_glib()
		}

		let callback = Some(callback_func::<P> as _);

		unsafe extern fn dnotify_func<
			P:Fn(&AuthDomainBasic, &Message, &str, &str) -> bool + 'static,
		>(
			data:glib::ffi::gpointer,
		) {
			let _callback:Box_<P> = Box_::from_raw(data as *mut _);
		}

		let destroy_call3 = Some(dnotify_func::<P> as _);

		let super_callback0:Box_<P> = callback_data;

		unsafe {
			ffi::soup_auth_domain_basic_set_auth_callback(
				self.as_ref().to_glib_none().0,
				callback,
				Box_::into_raw(super_callback0) as *mut _,
				destroy_call3,
			);
		}
	}

	// fn auth_data(&self) -> /*Unimplemented*/Fundamental: Pointer {
	//    unsafe {
	//        let mut value = glib::Value::from_type(</*Unknown type*/ as
	// StaticType>::static_type());
	//        glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut
	// glib::gobject_ffi::GObject, b"auth-data\0".as_ptr() as *const _,
	// value.to_glib_none_mut().0);        value.get().expect("Return Value for
	// property `auth-data` getter")    }
	//}

	// fn set_auth_data(&self, auth_data: /*Unimplemented*/Fundamental: Pointer) {
	//    unsafe {
	//        glib::gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut
	// glib::gobject_ffi::GObject, b"auth-data\0".as_ptr() as *const _,
	// auth_data.to_value().to_glib_none().0);    }
	//}

	fn connect_auth_data_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_auth_data_trampoline<P:IsA<AuthDomainBasic>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupAuthDomainBasic,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);

			f(AuthDomainBasic::from_glib_borrow(this).unsafe_cast_ref())
		}

		unsafe {
			let f:Box_<F> = Box_::new(f);

			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::auth-data\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_auth_data_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl fmt::Display for AuthDomainBasic {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("AuthDomainBasic") }
}
