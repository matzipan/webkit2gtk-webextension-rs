// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DOMElement;
use crate::DOMEventTarget;
use crate::DOMHTMLElement;
use crate::DOMNode;
use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct DOMHTMLHtmlElement(Object<ffi::WebKitDOMHTMLHtmlElement, ffi::WebKitDOMHTMLHtmlElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_html_element_get_type(),
    }
}

pub const NONE_DOMHTML_HTML_ELEMENT: Option<&DOMHTMLHtmlElement> = None;

pub trait DOMHTMLHtmlElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_html_element_get_version")]
    fn get_version(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_html_element_set_version")]
    fn set_version(&self, value: &str);

    fn connect_property_version_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLHtmlElement>> DOMHTMLHtmlElementExt for O {
    fn get_version(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_html_element_get_version(self.as_ref().to_glib_none().0))
        }
    }

    fn set_version(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_html_element_set_version(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_version_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_version_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLHtmlElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLHtmlElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLHtmlElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::version\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_version_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLHtmlElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLHtmlElement")
    }
}
