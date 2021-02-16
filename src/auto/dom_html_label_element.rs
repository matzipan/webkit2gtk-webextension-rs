// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DOMElement;
use crate::DOMEventTarget;
use crate::DOMHTMLElement;
use crate::DOMHTMLFormElement;
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
    pub struct DOMHTMLLabelElement(Object<ffi::WebKitDOMHTMLLabelElement, ffi::WebKitDOMHTMLLabelElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_label_element_get_type(),
    }
}

pub const NONE_DOMHTML_LABEL_ELEMENT: Option<&DOMHTMLLabelElement> = None;

pub trait DOMHTMLLabelElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_label_element_get_form")]
    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_label_element_get_html_for")]
    fn get_html_for(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_label_element_set_html_for")]
    fn set_html_for(&self, value: &str);

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_html_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLLabelElement>> DOMHTMLLabelElementExt for O {
    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_label_element_get_form(self.as_ref().to_glib_none().0))
        }
    }

    fn get_html_for(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_label_element_get_html_for(self.as_ref().to_glib_none().0))
        }
    }

    fn set_html_for(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_label_element_set_html_for(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_form_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLLabelElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLLabelElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLLabelElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::form\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_form_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_html_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_html_for_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLLabelElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLLabelElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLLabelElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::html-for\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_html_for_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLLabelElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLLabelElement")
    }
}
