// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use glib::translate::*;

use crate::Buffer;

glib::wrapper! {
	#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct MessageBody(Boxed<ffi::SoupMessageBody>);

	match fn {
		copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::soup_message_body_get_type(), ptr as *mut _) as *mut ffi::SoupMessageBody,
		free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::soup_message_body_get_type(), ptr as *mut _),
		type_ => || ffi::soup_message_body_get_type(),
	}
}

impl MessageBody {
	#[doc(alias = "soup_message_body_new")]
	pub fn new() -> MessageBody {
		crate::assert_initialized_main_thread!();
		unsafe { from_glib_full(ffi::soup_message_body_new()) }
	}

	#[doc(alias = "soup_message_body_append_buffer")]
	pub fn append_buffer(&mut self, buffer:&mut Buffer) {
		unsafe {
			ffi::soup_message_body_append_buffer(
				self.to_glib_none_mut().0,
				buffer.to_glib_none_mut().0,
			);
		}
	}

	#[cfg(any(feature = "v2_32", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
	#[doc(alias = "soup_message_body_append_take")]
	pub fn append_take(&mut self, data:&[u8]) {
		let length = data.len() as usize;
		unsafe {
			ffi::soup_message_body_append_take(
				self.to_glib_none_mut().0,
				data.to_glib_full(),
				length,
			);
		}
	}

	#[doc(alias = "soup_message_body_complete")]
	pub fn complete(&mut self) {
		unsafe {
			ffi::soup_message_body_complete(self.to_glib_none_mut().0);
		}
	}

	#[doc(alias = "soup_message_body_flatten")]
	pub fn flatten(&mut self) -> Option<Buffer> {
		unsafe { from_glib_full(ffi::soup_message_body_flatten(self.to_glib_none_mut().0)) }
	}

	#[cfg(any(feature = "v2_24", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
	#[doc(alias = "soup_message_body_get_accumulate")]
	#[doc(alias = "get_accumulate")]
	pub fn is_accumulate(&mut self) -> bool {
		unsafe { from_glib(ffi::soup_message_body_get_accumulate(self.to_glib_none_mut().0)) }
	}

	#[doc(alias = "soup_message_body_get_chunk")]
	#[doc(alias = "get_chunk")]
	pub fn chunk(&mut self, offset:i64) -> Option<Buffer> {
		unsafe {
			from_glib_full(ffi::soup_message_body_get_chunk(self.to_glib_none_mut().0, offset))
		}
	}

	#[cfg(any(feature = "v2_24", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
	#[doc(alias = "soup_message_body_got_chunk")]
	pub fn got_chunk(&mut self, chunk:&mut Buffer) {
		unsafe {
			ffi::soup_message_body_got_chunk(self.to_glib_none_mut().0, chunk.to_glib_none_mut().0);
		}
	}

	#[cfg(any(feature = "v2_24", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
	#[doc(alias = "soup_message_body_set_accumulate")]
	pub fn set_accumulate(&mut self, accumulate:bool) {
		unsafe {
			ffi::soup_message_body_set_accumulate(
				self.to_glib_none_mut().0,
				accumulate.into_glib(),
			);
		}
	}

	#[doc(alias = "soup_message_body_truncate")]
	pub fn truncate(&mut self) {
		unsafe {
			ffi::soup_message_body_truncate(self.to_glib_none_mut().0);
		}
	}

	#[cfg(any(feature = "v2_24", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
	#[doc(alias = "soup_message_body_wrote_chunk")]
	pub fn wrote_chunk(&mut self, chunk:&mut Buffer) {
		unsafe {
			ffi::soup_message_body_wrote_chunk(
				self.to_glib_none_mut().0,
				chunk.to_glib_none_mut().0,
			);
		}
	}
}

impl Default for MessageBody {
	fn default() -> Self { Self::new() }
}
