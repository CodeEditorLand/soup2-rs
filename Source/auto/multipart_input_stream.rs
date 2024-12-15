// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::fmt;
#[cfg(any(feature = "v2_40", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
use std::ptr;

use glib::{StaticType, object::IsA, translate::*};

use crate::Message;
#[cfg(any(feature = "v2_40", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
use crate::MessageHeaders;

glib::wrapper! {
	#[doc(alias = "SoupMultipartInputStream")]
	pub struct MultipartInputStream(Object<ffi::SoupMultipartInputStream, ffi::SoupMultipartInputStreamClass>) @extends gio::InputStream;

	match fn {
		type_ => || ffi::soup_multipart_input_stream_get_type(),
	}
}

impl MultipartInputStream {
	#[cfg(any(feature = "v2_40", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
	#[doc(alias = "soup_multipart_input_stream_new")]
	pub fn new(
		msg:&impl IsA<Message>,
		base_stream:&impl IsA<gio::InputStream>,
	) -> MultipartInputStream {
		crate::skip_assert_initialized!();

		unsafe {
			from_glib_full(ffi::soup_multipart_input_stream_new(
				msg.as_ref().to_glib_none().0,
				base_stream.as_ref().to_glib_none().0,
			))
		}
	}
}

pub const NONE_MULTIPART_INPUT_STREAM:Option<&MultipartInputStream> = None;

pub trait MultipartInputStreamExt: 'static {
	#[cfg(any(feature = "v2_40", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
	#[doc(alias = "soup_multipart_input_stream_get_headers")]
	#[doc(alias = "get_headers")]
	fn headers(&self) -> Option<MessageHeaders>;

	#[cfg(any(feature = "v2_40", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
	#[doc(alias = "soup_multipart_input_stream_next_part")]
	fn next_part(
		&self,
		cancellable:Option<&impl IsA<gio::Cancellable>>,
	) -> Result<Option<gio::InputStream>, glib::Error>;

	fn message(&self) -> Option<Message>;
}

impl<O:IsA<MultipartInputStream>> MultipartInputStreamExt for O {
	#[cfg(any(feature = "v2_40", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
	fn headers(&self) -> Option<MessageHeaders> {
		unsafe {
			from_glib_none(ffi::soup_multipart_input_stream_get_headers(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[cfg(any(feature = "v2_40", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
	fn next_part(
		&self,
		cancellable:Option<&impl IsA<gio::Cancellable>>,
	) -> Result<Option<gio::InputStream>, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();

			let ret = ffi::soup_multipart_input_stream_next_part(
				self.as_ref().to_glib_none().0,
				cancellable.map(|p| p.as_ref()).to_glib_none().0,
				&mut error,
			);

			if error.is_null() {
				Ok(from_glib_full(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn message(&self) -> Option<Message> {
		unsafe {
			let mut value = glib::Value::from_type(<Message as StaticType>::static_type());

			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"message\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);

			value.get().expect("Return Value for property `message` getter")
		}
	}
}

impl fmt::Display for MultipartInputStream {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("MultipartInputStream") }
}
