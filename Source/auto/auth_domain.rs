// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::{boxed::Box as Box_, fmt, mem::transmute};

use glib::{
	StaticType,
	ToValue,
	object::{Cast, IsA},
	signal::{SignalHandlerId, connect_raw},
	translate::*,
};

use crate::Message;

glib::wrapper! {
	#[doc(alias = "SoupAuthDomain")]
	pub struct AuthDomain(Object<ffi::SoupAuthDomain, ffi::SoupAuthDomainClass>);

	match fn {
		type_ => || ffi::soup_auth_domain_get_type(),
	}
}

pub const NONE_AUTH_DOMAIN:Option<&AuthDomain> = None;

pub trait AuthDomainExt: 'static {
	#[doc(alias = "soup_auth_domain_accepts")]
	fn accepts(&self, msg:&impl IsA<Message>) -> Option<glib::GString>;

	#[doc(alias = "soup_auth_domain_add_path")]
	fn add_path(&self, path:&str);

	#[doc(alias = "soup_auth_domain_challenge")]
	fn challenge(&self, msg:&impl IsA<Message>);

	#[doc(alias = "soup_auth_domain_check_password")]
	fn check_password(&self, msg:&impl IsA<Message>, username:&str, password:&str) -> bool;

	#[doc(alias = "soup_auth_domain_covers")]
	fn covers(&self, msg:&impl IsA<Message>) -> bool;

	#[doc(alias = "soup_auth_domain_get_realm")]
	#[doc(alias = "get_realm")]
	fn realm(&self) -> Option<glib::GString>;

	#[doc(alias = "soup_auth_domain_remove_path")]
	fn remove_path(&self, path:&str);

	#[doc(alias = "soup_auth_domain_set_filter")]
	fn set_filter<P:Fn(&AuthDomain, &Message) -> bool + 'static>(&self, filter:P);

	#[doc(alias = "soup_auth_domain_set_generic_auth_callback")]
	fn set_generic_auth_callback<P:Fn(&AuthDomain, &Message, &str) -> bool + 'static>(
		&self,
		auth_callback:P,
	);

	#[doc(alias = "soup_auth_domain_try_generic_auth_callback")]
	fn try_generic_auth_callback(&self, msg:&impl IsA<Message>, username:&str) -> bool;

	#[doc(alias = "add-path")]
	fn set_add_path(&self, add_path:Option<&str>);

	//#[doc(alias = "filter-data")]
	// fn filter_data(&self) -> /*Unimplemented*/Fundamental: Pointer;

	//#[doc(alias = "filter-data")]
	// fn set_filter_data(&self, filter_data: /*Unimplemented*/Fundamental:
	// Pointer);

	//#[doc(alias = "generic-auth-data")]
	// fn generic_auth_data(&self) -> /*Unimplemented*/Fundamental: Pointer;

	//#[doc(alias = "generic-auth-data")]
	// fn set_generic_auth_data(&self, generic_auth_data:
	// /*Unimplemented*/Fundamental: Pointer);

	fn is_proxy(&self) -> bool;

	#[doc(alias = "remove-path")]
	fn set_remove_path(&self, remove_path:Option<&str>);

	#[doc(alias = "add-path")]
	fn connect_add_path_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "filter-data")]
	fn connect_filter_data_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "generic-auth-data")]
	fn connect_generic_auth_data_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "remove-path")]
	fn connect_remove_path_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;
}

impl<O:IsA<AuthDomain>> AuthDomainExt for O {
	fn accepts(&self, msg:&impl IsA<Message>) -> Option<glib::GString> {
		unsafe {
			from_glib_full(ffi::soup_auth_domain_accepts(
				self.as_ref().to_glib_none().0,
				msg.as_ref().to_glib_none().0,
			))
		}
	}

	fn add_path(&self, path:&str) {
		unsafe {
			ffi::soup_auth_domain_add_path(self.as_ref().to_glib_none().0, path.to_glib_none().0);
		}
	}

	fn challenge(&self, msg:&impl IsA<Message>) {
		unsafe {
			ffi::soup_auth_domain_challenge(
				self.as_ref().to_glib_none().0,
				msg.as_ref().to_glib_none().0,
			);
		}
	}

	fn check_password(&self, msg:&impl IsA<Message>, username:&str, password:&str) -> bool {
		unsafe {
			from_glib(ffi::soup_auth_domain_check_password(
				self.as_ref().to_glib_none().0,
				msg.as_ref().to_glib_none().0,
				username.to_glib_none().0,
				password.to_glib_none().0,
			))
		}
	}

	fn covers(&self, msg:&impl IsA<Message>) -> bool {
		unsafe {
			from_glib(ffi::soup_auth_domain_covers(
				self.as_ref().to_glib_none().0,
				msg.as_ref().to_glib_none().0,
			))
		}
	}

	fn realm(&self) -> Option<glib::GString> {
		unsafe { from_glib_none(ffi::soup_auth_domain_get_realm(self.as_ref().to_glib_none().0)) }
	}

	fn remove_path(&self, path:&str) {
		unsafe {
			ffi::soup_auth_domain_remove_path(
				self.as_ref().to_glib_none().0,
				path.to_glib_none().0,
			);
		}
	}

	fn set_filter<P:Fn(&AuthDomain, &Message) -> bool + 'static>(&self, filter:P) {
		let filter_data:Box_<P> = Box_::new(filter);

		unsafe extern fn filter_func<P:Fn(&AuthDomain, &Message) -> bool + 'static>(
			domain:*mut ffi::SoupAuthDomain,
			msg:*mut ffi::SoupMessage,
			user_data:glib::ffi::gpointer,
		) -> glib::ffi::gboolean {
			let domain = from_glib_borrow(domain);

			let msg = from_glib_borrow(msg);

			let callback:&P = &*(user_data as *mut _);

			let res = (*callback)(&domain, &msg);

			res.into_glib()
		}

		let filter = Some(filter_func::<P> as _);

		unsafe extern fn dnotify_func<P:Fn(&AuthDomain, &Message) -> bool + 'static>(
			data:glib::ffi::gpointer,
		) {
			let _callback:Box_<P> = Box_::from_raw(data as *mut _);
		}

		let destroy_call3 = Some(dnotify_func::<P> as _);

		let super_callback0:Box_<P> = filter_data;

		unsafe {
			ffi::soup_auth_domain_set_filter(
				self.as_ref().to_glib_none().0,
				filter,
				Box_::into_raw(super_callback0) as *mut _,
				destroy_call3,
			);
		}
	}

	fn set_generic_auth_callback<P:Fn(&AuthDomain, &Message, &str) -> bool + 'static>(
		&self,
		auth_callback:P,
	) {
		let auth_callback_data:Box_<P> = Box_::new(auth_callback);

		unsafe extern fn auth_callback_func<P:Fn(&AuthDomain, &Message, &str) -> bool + 'static>(
			domain:*mut ffi::SoupAuthDomain,
			msg:*mut ffi::SoupMessage,
			username:*const libc::c_char,
			user_data:glib::ffi::gpointer,
		) -> glib::ffi::gboolean {
			let domain = from_glib_borrow(domain);

			let msg = from_glib_borrow(msg);

			let username:Borrowed<glib::GString> = from_glib_borrow(username);

			let callback:&P = &*(user_data as *mut _);

			let res = (*callback)(&domain, &msg, username.as_str());

			res.into_glib()
		}

		let auth_callback = Some(auth_callback_func::<P> as _);

		unsafe extern fn dnotify_func<P:Fn(&AuthDomain, &Message, &str) -> bool + 'static>(
			data:glib::ffi::gpointer,
		) {
			let _callback:Box_<P> = Box_::from_raw(data as *mut _);
		}

		let destroy_call3 = Some(dnotify_func::<P> as _);

		let super_callback0:Box_<P> = auth_callback_data;

		unsafe {
			ffi::soup_auth_domain_set_generic_auth_callback(
				self.as_ref().to_glib_none().0,
				auth_callback,
				Box_::into_raw(super_callback0) as *mut _,
				destroy_call3,
			);
		}
	}

	fn try_generic_auth_callback(&self, msg:&impl IsA<Message>, username:&str) -> bool {
		unsafe {
			from_glib(ffi::soup_auth_domain_try_generic_auth_callback(
				self.as_ref().to_glib_none().0,
				msg.as_ref().to_glib_none().0,
				username.to_glib_none().0,
			))
		}
	}

	fn set_add_path(&self, add_path:Option<&str>) {
		unsafe {
			glib::gobject_ffi::g_object_set_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"add-path\0".as_ptr() as *const _,
				add_path.to_value().to_glib_none().0,
			);
		}
	}

	// fn filter_data(&self) -> /*Unimplemented*/Fundamental: Pointer {
	//    unsafe {
	//        let mut value = glib::Value::from_type(</*Unknown type*/ as
	// StaticType>::static_type());
	//        glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut
	// glib::gobject_ffi::GObject, b"filter-data\0".as_ptr() as *const _,
	// value.to_glib_none_mut().0);        value.get().expect("Return Value for
	// property `filter-data` getter")    }
	//}

	// fn set_filter_data(&self, filter_data: /*Unimplemented*/Fundamental: Pointer)
	// {    unsafe {
	//        glib::gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut
	// glib::gobject_ffi::GObject, b"filter-data\0".as_ptr() as *const _,
	// filter_data.to_value().to_glib_none().0);    }
	//}

	// fn generic_auth_data(&self) -> /*Unimplemented*/Fundamental: Pointer {
	//    unsafe {
	//        let mut value = glib::Value::from_type(</*Unknown type*/ as
	// StaticType>::static_type());
	//        glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut
	// glib::gobject_ffi::GObject, b"generic-auth-data\0".as_ptr() as *const _,
	// value.to_glib_none_mut().0);        value.get().expect("Return Value for
	// property `generic-auth-data` getter")    }
	//}

	// fn set_generic_auth_data(&self, generic_auth_data:
	// /*Unimplemented*/Fundamental: Pointer) {    unsafe {
	//        glib::gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut
	// glib::gobject_ffi::GObject, b"generic-auth-data\0".as_ptr() as *const _,
	// generic_auth_data.to_value().to_glib_none().0);    }
	//}

	fn is_proxy(&self) -> bool {
		unsafe {
			let mut value = glib::Value::from_type(<bool as StaticType>::static_type());

			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"proxy\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);

			value.get().expect("Return Value for property `proxy` getter")
		}
	}

	fn set_remove_path(&self, remove_path:Option<&str>) {
		unsafe {
			glib::gobject_ffi::g_object_set_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"remove-path\0".as_ptr() as *const _,
				remove_path.to_value().to_glib_none().0,
			);
		}
	}

	fn connect_add_path_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_add_path_trampoline<P:IsA<AuthDomain>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupAuthDomain,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);

			f(AuthDomain::from_glib_borrow(this).unsafe_cast_ref())
		}

		unsafe {
			let f:Box_<F> = Box_::new(f);

			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::add-path\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_add_path_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_filter_data_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_filter_data_trampoline<P:IsA<AuthDomain>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupAuthDomain,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);

			f(AuthDomain::from_glib_borrow(this).unsafe_cast_ref())
		}

		unsafe {
			let f:Box_<F> = Box_::new(f);

			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::filter-data\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_filter_data_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_generic_auth_data_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_generic_auth_data_trampoline<
			P:IsA<AuthDomain>,
			F:Fn(&P) + 'static,
		>(
			this:*mut ffi::SoupAuthDomain,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);

			f(AuthDomain::from_glib_borrow(this).unsafe_cast_ref())
		}

		unsafe {
			let f:Box_<F> = Box_::new(f);

			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::generic-auth-data\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_generic_auth_data_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_remove_path_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_remove_path_trampoline<P:IsA<AuthDomain>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupAuthDomain,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);

			f(AuthDomain::from_glib_borrow(this).unsafe_cast_ref())
		}

		unsafe {
			let f:Box_<F> = Box_::new(f);

			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::remove-path\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_remove_path_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl fmt::Display for AuthDomain {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("AuthDomain") }
}
