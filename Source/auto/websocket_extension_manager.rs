// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::fmt;

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use crate::SessionFeature;

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
glib::wrapper! {
	#[doc(alias = "SoupWebsocketExtensionManager")]
	pub struct WebsocketExtensionManager(Object<ffi::SoupWebsocketExtensionManager, ffi::SoupWebsocketExtensionManagerClass>) @implements SessionFeature;

	match fn {
		type_ => || ffi::soup_websocket_extension_manager_get_type(),
	}
}

#[cfg(not(any(feature = "v2_24", feature = "dox")))]
glib::wrapper! {
	#[doc(alias = "SoupWebsocketExtensionManager")]
	pub struct WebsocketExtensionManager(Object<ffi::SoupWebsocketExtensionManager, ffi::SoupWebsocketExtensionManagerClass>);

	match fn {
		type_ => || ffi::soup_websocket_extension_manager_get_type(),
	}
}

impl WebsocketExtensionManager {}

pub const NONE_WEBSOCKET_EXTENSION_MANAGER:Option<&WebsocketExtensionManager> = None;

impl fmt::Display for WebsocketExtensionManager {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("WebsocketExtensionManager") }
}
