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
    pub struct DOMHTMLParamElement(Object<ffi::WebKitDOMHTMLParamElement, ffi::WebKitDOMHTMLParamElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_param_element_get_type(),
    }
}

pub const NONE_DOMHTML_PARAM_ELEMENT: Option<&DOMHTMLParamElement> = None;

pub trait DOMHTMLParamElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_param_element_get_name")]
    fn get_name(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_param_element_get_type_attr")]
    fn get_type_attr(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_param_element_get_value")]
    fn get_value(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_param_element_get_value_type")]
    fn get_value_type(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_param_element_set_name")]
    fn set_name(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_param_element_set_type_attr")]
    fn set_type_attr(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_param_element_set_value")]
    fn set_value(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_param_element_set_value_type")]
    fn set_value_type(&self, value: &str);

    fn get_property_type(&self) -> Option<glib::GString>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLParamElement>> DOMHTMLParamElementExt for O {
    fn get_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_param_element_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_param_element_get_type_attr(self.as_ref().to_glib_none().0))
        }
    }

    fn get_value(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_param_element_get_value(self.as_ref().to_glib_none().0))
        }
    }

    fn get_value_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_param_element_get_value_type(self.as_ref().to_glib_none().0))
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_param_element_set_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_param_element_set_type_attr(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_param_element_set_value(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_value_type(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_param_element_set_value_type(self.as_ref().to_glib_none().0, value.to_glib_none().0);
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

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLParamElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLParamElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLParamElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLParamElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLParamElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLParamElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_type_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLParamElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLParamElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLParamElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_value_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_value_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMHTMLParamElement, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMHTMLParamElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLParamElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_value_type_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLParamElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLParamElement")
    }
}
