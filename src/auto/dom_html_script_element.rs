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
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct DOMHTMLScriptElement(Object<ffi::WebKitDOMHTMLScriptElement, ffi::WebKitDOMHTMLScriptElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_script_element_get_type(),
    }
}

pub const NONE_DOMHTML_SCRIPT_ELEMENT: Option<&DOMHTMLScriptElement> = None;

pub trait DOMHTMLScriptElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_get_charset")]
    fn get_charset(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_get_defer")]
    fn get_defer(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_get_event")]
    fn get_event(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_get_html_for")]
    fn get_html_for(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_get_src")]
    fn get_src(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_get_text")]
    fn get_text(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_get_type_attr")]
    fn get_type_attr(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_html_script_element_set_charset")]
    fn set_charset(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_set_defer")]
    fn set_defer(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_set_event")]
    fn set_event(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_set_html_for")]
    fn set_html_for(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_set_src")]
    fn set_src(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_set_text")]
    fn set_text(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_script_element_set_type_attr")]
    fn set_type_attr(&self, value: &str);

    fn set_property_charset(&self, charset: Option<&str>);

    fn get_property_type(&self) -> Option<glib::GString>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_defer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_event_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_html_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLScriptElement>> DOMHTMLScriptElementExt for O {
    fn get_charset(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_script_element_get_charset(self.as_ref().to_glib_none().0))
        }
    }

    fn get_defer(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_script_element_get_defer(self.as_ref().to_glib_none().0))
        }
    }

    fn get_event(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_script_element_get_event(self.as_ref().to_glib_none().0))
        }
    }

    fn get_html_for(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_script_element_get_html_for(self.as_ref().to_glib_none().0))
        }
    }

    fn get_src(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_script_element_get_src(self.as_ref().to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_script_element_get_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_script_element_get_type_attr(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_charset(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_charset(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_defer(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_defer(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_event(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_event(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_html_for(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_html_for(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_src(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_src(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_text(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_text(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_type_attr(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_property_charset(&self, charset: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"charset\0".as_ptr() as *const _, glib::Value::from(charset).to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `type` getter")
        }
    }

    fn set_property_type(&self, type_: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"type\0".as_ptr() as *const _, glib::Value::from(type_).to_glib_none().0);
        }
    }

    fn connect_property_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_charset_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLScriptElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::charset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_charset_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_defer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_defer_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLScriptElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::defer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_defer_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_event_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_event_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLScriptElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_event_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_html_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_html_for_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLScriptElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::html-for\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_html_for_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_src_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLScriptElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::src\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_src_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLScriptElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_text_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLScriptElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLScriptElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_type_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLScriptElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLScriptElement")
    }
}
