// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMHTMLDirectoryElement(Object<webkit2_webextension_sys::WebKitDOMHTMLDirectoryElement, webkit2_webextension_sys::WebKitDOMHTMLDirectoryElementClass, DOMHTMLDirectoryElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_directory_element_get_type(),
    }
}

pub const NONE_DOMHTML_DIRECTORY_ELEMENT: Option<&DOMHTMLDirectoryElement> = None;

pub trait DOMHTMLDirectoryElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_compact(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_compact(&self, value: bool);

    fn connect_property_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLDirectoryElement>> DOMHTMLDirectoryElementExt for O {
    fn get_compact(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_html_directory_element_get_compact(self.as_ref().to_glib_none().0))
        }
    }

    fn set_compact(&self, value: bool) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_directory_element_set_compact(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn connect_property_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_compact_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLDirectoryElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLDirectoryElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLDirectoryElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::compact\0".as_ptr() as *const _,
                Some(transmute(notify_compact_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLDirectoryElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLDirectoryElement")
    }
}
