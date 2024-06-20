// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::Date;
#[cfg(any(feature = "v2_70", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_70")))]
use crate::SameSitePolicy;
use crate::URI;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Cookie(Boxed<ffi::SoupCookie>);

    match fn {
        copy => |ptr| ffi::soup_cookie_copy(mut_override(ptr)),
        free => |ptr| ffi::soup_cookie_free(ptr),
        type_ => || ffi::soup_cookie_get_type(),
    }
}

impl Cookie {
    #[doc(alias = "soup_cookie_new")]
    pub fn new(name: &str, value: &str, domain: &str, path: &str, max_age: i32) -> Cookie {
        crate::assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::soup_cookie_new(name.to_glib_none().0, value.to_glib_none().0, domain.to_glib_none().0, path.to_glib_none().0, max_age))
        }
    }

    #[doc(alias = "soup_cookie_applies_to_uri")]
    pub fn applies_to_uri(&mut self, uri: &mut URI) -> bool {
        unsafe {
            from_glib(ffi::soup_cookie_applies_to_uri(self.to_glib_none_mut().0, uri.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "soup_cookie_domain_matches")]
    pub fn domain_matches(&mut self, host: &str) -> bool {
        unsafe {
            from_glib(ffi::soup_cookie_domain_matches(self.to_glib_none_mut().0, host.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_cookie_get_domain")]
    #[doc(alias = "get_domain")]
    pub fn domain(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_cookie_get_domain(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_cookie_get_expires")]
    #[doc(alias = "get_expires")]
    pub fn expires(&mut self) -> Option<Date> {
        unsafe {
            from_glib_none(ffi::soup_cookie_get_expires(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_cookie_get_http_only")]
    #[doc(alias = "get_http_only")]
    pub fn is_http_only(&mut self) -> bool {
        unsafe {
            from_glib(ffi::soup_cookie_get_http_only(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_cookie_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_cookie_get_name(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_cookie_get_path")]
    #[doc(alias = "get_path")]
    pub fn path(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_cookie_get_path(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_70", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_70")))]
    #[doc(alias = "soup_cookie_get_same_site_policy")]
    #[doc(alias = "get_same_site_policy")]
    pub fn same_site_policy(&mut self) -> SameSitePolicy {
        unsafe {
            from_glib(ffi::soup_cookie_get_same_site_policy(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_cookie_get_secure")]
    #[doc(alias = "get_secure")]
    pub fn is_secure(&mut self) -> bool {
        unsafe {
            from_glib(ffi::soup_cookie_get_secure(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_cookie_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_cookie_get_value(self.to_glib_none_mut().0))
        }
    }

    #[doc(alias = "soup_cookie_set_domain")]
    pub fn set_domain(&mut self, domain: &str) {
        unsafe {
            ffi::soup_cookie_set_domain(self.to_glib_none_mut().0, domain.to_glib_none().0);
        }
    }

    #[doc(alias = "soup_cookie_set_expires")]
    pub fn set_expires(&mut self, expires: &mut Date) {
        unsafe {
            ffi::soup_cookie_set_expires(self.to_glib_none_mut().0, expires.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "soup_cookie_set_http_only")]
    pub fn set_http_only(&mut self, http_only: bool) {
        unsafe {
            ffi::soup_cookie_set_http_only(self.to_glib_none_mut().0, http_only.into_glib());
        }
    }

    #[doc(alias = "soup_cookie_set_max_age")]
    pub fn set_max_age(&mut self, max_age: i32) {
        unsafe {
            ffi::soup_cookie_set_max_age(self.to_glib_none_mut().0, max_age);
        }
    }

    #[doc(alias = "soup_cookie_set_name")]
    pub fn set_name(&mut self, name: &str) {
        unsafe {
            ffi::soup_cookie_set_name(self.to_glib_none_mut().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "soup_cookie_set_path")]
    pub fn set_path(&mut self, path: &str) {
        unsafe {
            ffi::soup_cookie_set_path(self.to_glib_none_mut().0, path.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_70", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_70")))]
    #[doc(alias = "soup_cookie_set_same_site_policy")]
    pub fn set_same_site_policy(&mut self, policy: SameSitePolicy) {
        unsafe {
            ffi::soup_cookie_set_same_site_policy(self.to_glib_none_mut().0, policy.into_glib());
        }
    }

    #[doc(alias = "soup_cookie_set_secure")]
    pub fn set_secure(&mut self, secure: bool) {
        unsafe {
            ffi::soup_cookie_set_secure(self.to_glib_none_mut().0, secure.into_glib());
        }
    }

    #[doc(alias = "soup_cookie_set_value")]
    pub fn set_value(&mut self, value: &str) {
        unsafe {
            ffi::soup_cookie_set_value(self.to_glib_none_mut().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "soup_cookie_to_cookie_header")]
    pub fn to_cookie_header(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::soup_cookie_to_cookie_header(self.to_glib_none_mut().0))
        }
    }

    #[doc(alias = "soup_cookie_to_set_cookie_header")]
    pub fn to_set_cookie_header(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::soup_cookie_to_set_cookie_header(self.to_glib_none_mut().0))
        }
    }

    #[doc(alias = "soup_cookie_parse")]
    pub fn parse(header: &str, origin: &mut URI) -> Option<Cookie> {
        crate::assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::soup_cookie_parse(header.to_glib_none().0, origin.to_glib_none_mut().0))
        }
    }
}
