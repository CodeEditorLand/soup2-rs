// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::{boxed::Box as Box_, fmt, mem::transmute};

use glib::{
	object::{Cast, IsA},
	signal::{connect_raw, SignalHandlerId},
	translate::*,
	StaticType,
	ToValue,
};

use crate::Address;

glib::wrapper! {
	#[doc(alias = "SoupSocket")]
	pub struct Socket(Object<ffi::SoupSocket, ffi::SoupSocketClass>);

	match fn {
		type_ => || ffi::soup_socket_get_type(),
	}
}

impl Socket {
	//#[doc(alias = "soup_socket_new")]
	// pub fn new(optname1: &str, : /*Unknown
	// conversion*//*Unimplemented*/Fundamental: VarArgs) -> Socket {    unsafe
	// { TODO: call ffi:soup_socket_new() }
	//}
}

pub const NONE_SOCKET:Option<&Socket> = None;

pub trait SocketExt: 'static {
	#[doc(alias = "soup_socket_connect_async")]
	fn connect_async<P:FnOnce(&Socket, u32) + 'static>(
		&self,
		cancellable:Option<&impl IsA<gio::Cancellable>>,
		callback:P,
	);

	#[doc(alias = "soup_socket_connect_sync")]
	fn connect_sync(&self, cancellable:Option<&impl IsA<gio::Cancellable>>) -> u32;

	#[doc(alias = "soup_socket_disconnect")]
	fn disconnect(&self);

	#[doc(alias = "soup_socket_get_fd")]
	#[doc(alias = "get_fd")]
	fn fd(&self) -> i32;

	#[doc(alias = "soup_socket_get_local_address")]
	#[doc(alias = "get_local_address")]
	fn local_address(&self) -> Option<Address>;

	#[doc(alias = "soup_socket_get_remote_address")]
	#[doc(alias = "get_remote_address")]
	fn remote_address(&self) -> Option<Address>;

	#[doc(alias = "soup_socket_is_connected")]
	fn is_connected(&self) -> bool;

	#[doc(alias = "soup_socket_is_ssl")]
	fn is_ssl(&self) -> bool;

	#[doc(alias = "soup_socket_listen")]
	fn listen(&self) -> bool;

	//#[doc(alias = "soup_socket_read_until")]
	// fn read_until(&self, buffer: &[u8], boundary:
	// /*Unimplemented*/Option<Fundamental: Pointer>, boundary_len: usize,
	// got_boundary: bool, cancellable: Option<&impl IsA<gio::Cancellable>>) ->
	// Result<(SocketIOStatus, usize), glib::Error>;

	#[doc(alias = "soup_socket_start_proxy_ssl")]
	fn start_proxy_ssl(
		&self,
		ssl_host:&str,
		cancellable:Option<&impl IsA<gio::Cancellable>>,
	) -> bool;

	#[doc(alias = "soup_socket_start_ssl")]
	fn start_ssl(&self, cancellable:Option<&impl IsA<gio::Cancellable>>) -> bool;

	//#[doc(alias = "async-context")]
	// fn async_context(&self) -> /*Unimplemented*/Fundamental: Pointer;

	#[doc(alias = "ipv6-only")]
	fn is_ipv6_only(&self) -> bool;

	#[doc(alias = "ipv6-only")]
	fn set_ipv6_only(&self, ipv6_only:bool);

	#[doc(alias = "is-server")]
	fn is_server(&self) -> bool;

	#[doc(alias = "non-blocking")]
	fn is_non_blocking(&self) -> bool;

	#[doc(alias = "non-blocking")]
	fn set_non_blocking(&self, non_blocking:bool);

	//#[doc(alias = "ssl-creds")]
	// fn ssl_creds(&self) -> /*Unimplemented*/Fundamental: Pointer;

	//#[doc(alias = "ssl-creds")]
	// fn set_ssl_creds(&self, ssl_creds: /*Unimplemented*/Fundamental:
	// Pointer);

	#[doc(alias = "ssl-fallback")]
	fn is_ssl_fallback(&self) -> bool;

	#[doc(alias = "ssl-strict")]
	fn is_ssl_strict(&self) -> bool;

	fn timeout(&self) -> u32;

	fn set_timeout(&self, timeout:u32);

	#[doc(alias = "tls-certificate")]
	fn tls_certificate(&self) -> Option<gio::TlsCertificate>;

	#[doc(alias = "tls-errors")]
	fn tls_errors(&self) -> gio::TlsCertificateFlags;

	#[doc(alias = "trusted-certificate")]
	fn is_trusted_certificate(&self) -> bool;

	#[cfg(any(feature = "v2_38", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
	#[doc(alias = "use-thread-context")]
	fn uses_thread_context(&self) -> bool;

	#[doc(alias = "disconnected")]
	fn connect_disconnected<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[cfg(any(feature = "v2_38", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
	#[doc(alias = "event")]
	fn connect_event<F:Fn(&Self, gio::SocketClientEvent, &gio::IOStream) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId;

	#[doc(alias = "new-connection")]
	fn connect_new_connection<F:Fn(&Self, &Socket) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "readable")]
	fn connect_readable<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "writable")]
	fn connect_writable<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "ipv6-only")]
	fn connect_ipv6_only_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "is-server")]
	fn connect_is_server_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "non-blocking")]
	fn connect_non_blocking_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "ssl-creds")]
	fn connect_ssl_creds_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "timeout")]
	fn connect_timeout_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "tls-certificate")]
	fn connect_tls_certificate_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "tls-errors")]
	fn connect_tls_errors_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[doc(alias = "trusted-certificate")]
	fn connect_trusted_certificate_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;
}

impl<O:IsA<Socket>> SocketExt for O {
	fn connect_async<P:FnOnce(&Socket, u32) + 'static>(
		&self,
		cancellable:Option<&impl IsA<gio::Cancellable>>,
		callback:P,
	) {
		let callback_data:Box_<P> = Box_::new(callback);
		unsafe extern fn callback_func<P:FnOnce(&Socket, u32) + 'static>(
			sock:*mut ffi::SoupSocket,
			status:libc::c_uint,
			user_data:glib::ffi::gpointer,
		) {
			let sock = from_glib_borrow(sock);
			let callback:Box_<P> = Box_::from_raw(user_data as *mut _);
			(*callback)(&sock, status);
		}
		let callback = Some(callback_func::<P> as _);
		let super_callback0:Box_<P> = callback_data;
		unsafe {
			ffi::soup_socket_connect_async(
				self.as_ref().to_glib_none().0,
				cancellable.map(|p| p.as_ref()).to_glib_none().0,
				callback,
				Box_::into_raw(super_callback0) as *mut _,
			);
		}
	}

	fn connect_sync(&self, cancellable:Option<&impl IsA<gio::Cancellable>>) -> u32 {
		unsafe {
			ffi::soup_socket_connect_sync(
				self.as_ref().to_glib_none().0,
				cancellable.map(|p| p.as_ref()).to_glib_none().0,
			)
		}
	}

	fn disconnect(&self) {
		unsafe {
			ffi::soup_socket_disconnect(self.as_ref().to_glib_none().0);
		}
	}

	fn fd(&self) -> i32 { unsafe { ffi::soup_socket_get_fd(self.as_ref().to_glib_none().0) } }

	fn local_address(&self) -> Option<Address> {
		unsafe {
			from_glib_none(ffi::soup_socket_get_local_address(self.as_ref().to_glib_none().0))
		}
	}

	fn remote_address(&self) -> Option<Address> {
		unsafe {
			from_glib_none(ffi::soup_socket_get_remote_address(self.as_ref().to_glib_none().0))
		}
	}

	fn is_connected(&self) -> bool {
		unsafe { from_glib(ffi::soup_socket_is_connected(self.as_ref().to_glib_none().0)) }
	}

	fn is_ssl(&self) -> bool {
		unsafe { from_glib(ffi::soup_socket_is_ssl(self.as_ref().to_glib_none().0)) }
	}

	fn listen(&self) -> bool {
		unsafe { from_glib(ffi::soup_socket_listen(self.as_ref().to_glib_none().0)) }
	}

	// fn read_until(&self, buffer: &[u8], boundary:
	// /*Unimplemented*/Option<Fundamental: Pointer>, boundary_len: usize,
	// got_boundary: bool, cancellable: Option<&impl IsA<gio::Cancellable>>) ->
	// Result<(SocketIOStatus, usize), glib::Error> {    unsafe { TODO: call
	// ffi:soup_socket_read_until() }
	//}

	fn start_proxy_ssl(
		&self,
		ssl_host:&str,
		cancellable:Option<&impl IsA<gio::Cancellable>>,
	) -> bool {
		unsafe {
			from_glib(ffi::soup_socket_start_proxy_ssl(
				self.as_ref().to_glib_none().0,
				ssl_host.to_glib_none().0,
				cancellable.map(|p| p.as_ref()).to_glib_none().0,
			))
		}
	}

	fn start_ssl(&self, cancellable:Option<&impl IsA<gio::Cancellable>>) -> bool {
		unsafe {
			from_glib(ffi::soup_socket_start_ssl(
				self.as_ref().to_glib_none().0,
				cancellable.map(|p| p.as_ref()).to_glib_none().0,
			))
		}
	}

	// fn async_context(&self) -> /*Unimplemented*/Fundamental: Pointer {
	//    unsafe {
	//        let mut value = glib::Value::from_type(</*Unknown type*/ as
	// StaticType>::static_type());
	//        glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as
	// *mut glib::gobject_ffi::GObject, b"async-context\0".as_ptr() as *const _,
	// value.to_glib_none_mut().0);        value.get().expect("Return Value for
	// property `async-context` getter")    }
	//}

	fn is_ipv6_only(&self) -> bool {
		unsafe {
			let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"ipv6-only\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `ipv6-only` getter")
		}
	}

	fn set_ipv6_only(&self, ipv6_only:bool) {
		unsafe {
			glib::gobject_ffi::g_object_set_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"ipv6-only\0".as_ptr() as *const _,
				ipv6_only.to_value().to_glib_none().0,
			);
		}
	}

	fn is_server(&self) -> bool {
		unsafe {
			let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"is-server\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `is-server` getter")
		}
	}

	fn is_non_blocking(&self) -> bool {
		unsafe {
			let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"non-blocking\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `non-blocking` getter")
		}
	}

	fn set_non_blocking(&self, non_blocking:bool) {
		unsafe {
			glib::gobject_ffi::g_object_set_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"non-blocking\0".as_ptr() as *const _,
				non_blocking.to_value().to_glib_none().0,
			);
		}
	}

	// fn ssl_creds(&self) -> /*Unimplemented*/Fundamental: Pointer {
	//    unsafe {
	//        let mut value = glib::Value::from_type(</*Unknown type*/ as
	// StaticType>::static_type());
	//        glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as
	// *mut glib::gobject_ffi::GObject, b"ssl-creds\0".as_ptr() as *const _,
	// value.to_glib_none_mut().0);        value.get().expect("Return Value for
	// property `ssl-creds` getter")    }
	//}

	// fn set_ssl_creds(&self, ssl_creds: /*Unimplemented*/Fundamental: Pointer)
	// {    unsafe {
	//        glib::gobject_ffi::g_object_set_property(self.to_glib_none().0 as
	// *mut glib::gobject_ffi::GObject, b"ssl-creds\0".as_ptr() as *const _,
	// ssl_creds.to_value().to_glib_none().0);    }
	//}

	fn is_ssl_fallback(&self) -> bool {
		unsafe {
			let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"ssl-fallback\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `ssl-fallback` getter")
		}
	}

	fn is_ssl_strict(&self) -> bool {
		unsafe {
			let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"ssl-strict\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `ssl-strict` getter")
		}
	}

	fn timeout(&self) -> u32 {
		unsafe {
			let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"timeout\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `timeout` getter")
		}
	}

	fn set_timeout(&self, timeout:u32) {
		unsafe {
			glib::gobject_ffi::g_object_set_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"timeout\0".as_ptr() as *const _,
				timeout.to_value().to_glib_none().0,
			);
		}
	}

	fn tls_certificate(&self) -> Option<gio::TlsCertificate> {
		unsafe {
			let mut value =
				glib::Value::from_type(<gio::TlsCertificate as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"tls-certificate\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `tls-certificate` getter")
		}
	}

	fn tls_errors(&self) -> gio::TlsCertificateFlags {
		unsafe {
			let mut value =
				glib::Value::from_type(<gio::TlsCertificateFlags as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"tls-errors\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `tls-errors` getter")
		}
	}

	fn is_trusted_certificate(&self) -> bool {
		unsafe {
			let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"trusted-certificate\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `trusted-certificate` getter")
		}
	}

	#[cfg(any(feature = "v2_38", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
	fn uses_thread_context(&self) -> bool {
		unsafe {
			let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"use-thread-context\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `use-thread-context` getter")
		}
	}

	fn connect_disconnected<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn disconnected_trampoline<P:IsA<Socket>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupSocket,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Socket::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"disconnected\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					disconnected_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg(any(feature = "v2_38", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
	fn connect_event<F:Fn(&Self, gio::SocketClientEvent, &gio::IOStream) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId {
		unsafe extern fn event_trampoline<
			P:IsA<Socket>,
			F:Fn(&P, gio::SocketClientEvent, &gio::IOStream) + 'static,
		>(
			this:*mut ffi::SoupSocket,
			event:gio::ffi::GSocketClientEvent,
			connection:*mut gio::ffi::GIOStream,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(
				Socket::from_glib_borrow(this).unsafe_cast_ref(),
				from_glib(event),
				&from_glib_borrow(connection),
			)
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"event\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(event_trampoline::<Self, F> as *const ())),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_new_connection<F:Fn(&Self, &Socket) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn new_connection_trampoline<P:IsA<Socket>, F:Fn(&P, &Socket) + 'static>(
			this:*mut ffi::SoupSocket,
			new:*mut ffi::SoupSocket,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Socket::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(new))
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"new-connection\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					new_connection_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_readable<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn readable_trampoline<P:IsA<Socket>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupSocket,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Socket::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"readable\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					readable_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_writable<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn writable_trampoline<P:IsA<Socket>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupSocket,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Socket::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"writable\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					writable_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_ipv6_only_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_ipv6_only_trampoline<P:IsA<Socket>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupSocket,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Socket::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::ipv6-only\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_ipv6_only_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_is_server_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_is_server_trampoline<P:IsA<Socket>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupSocket,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Socket::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::is-server\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_is_server_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_non_blocking_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_non_blocking_trampoline<P:IsA<Socket>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupSocket,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Socket::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::non-blocking\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_non_blocking_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_ssl_creds_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_ssl_creds_trampoline<P:IsA<Socket>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupSocket,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Socket::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::ssl-creds\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_ssl_creds_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_timeout_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_timeout_trampoline<P:IsA<Socket>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupSocket,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Socket::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::timeout\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_timeout_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_tls_certificate_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_tls_certificate_trampoline<P:IsA<Socket>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupSocket,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Socket::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::tls-certificate\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_tls_certificate_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_tls_errors_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_tls_errors_trampoline<P:IsA<Socket>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupSocket,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Socket::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::tls-errors\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_tls_errors_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_trusted_certificate_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_trusted_certificate_trampoline<
			P:IsA<Socket>,
			F:Fn(&P) + 'static,
		>(
			this:*mut ffi::SoupSocket,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Socket::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::trusted-certificate\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_trusted_certificate_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl fmt::Display for Socket {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("Socket") }
}
