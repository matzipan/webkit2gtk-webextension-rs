// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLCollection;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLFormElement(Object<ffi::WebKitDOMHTMLFormElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_form_element_get_type(),
    }
}

pub trait DOMHTMLFormElementExt {
    fn get_accept_charset(&self) -> Option<String>;

    fn get_action(&self) -> Option<String>;

    fn get_elements(&self) -> Option<DOMHTMLCollection>;

    fn get_encoding(&self) -> Option<String>;

    fn get_enctype(&self) -> Option<String>;

    fn get_length(&self) -> libc::c_long;

    fn get_method(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    fn get_target(&self) -> Option<String>;

    fn reset(&self);

    fn set_accept_charset(&self, value: &str);

    fn set_action(&self, value: &str);

    fn set_encoding(&self, value: &str);

    fn set_enctype(&self, value: &str);

    fn set_method(&self, value: &str);

    fn set_name(&self, value: &str);

    fn set_target(&self, value: &str);

    fn submit(&self);

    fn connect_property_accept_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_elements_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_enctype_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_method_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLFormElement> + IsA<glib::object::Object>> DOMHTMLFormElementExt for O {
    fn get_accept_charset(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_accept_charset(self.to_glib_none().0))
        }
    }

    fn get_action(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_action(self.to_glib_none().0))
        }
    }

    fn get_elements(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_elements(self.to_glib_none().0))
        }
    }

    fn get_encoding(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_encoding(self.to_glib_none().0))
        }
    }

    fn get_enctype(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_enctype(self.to_glib_none().0))
        }
    }

    fn get_length(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_form_element_get_length(self.to_glib_none().0)
        }
    }

    fn get_method(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_method(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_name(self.to_glib_none().0))
        }
    }

    fn get_target(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_target(self.to_glib_none().0))
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::webkit_dom_html_form_element_reset(self.to_glib_none().0);
        }
    }

    fn set_accept_charset(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_accept_charset(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_action(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_action(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_encoding(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_encoding(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_enctype(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_enctype(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_method(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_method(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_target(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_target(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn submit(&self) {
        unsafe {
            ffi::webkit_dom_html_form_element_submit(self.to_glib_none().0);
        }
    }

    fn connect_property_accept_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accept-charset",
                transmute(notify_accept_charset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::action",
                transmute(notify_action_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_elements_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::elements",
                transmute(notify_elements_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::encoding",
                transmute(notify_encoding_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_enctype_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::enctype",
                transmute(notify_enctype_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::length",
                transmute(notify_length_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_method_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::method",
                transmute(notify_method_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::target",
                transmute(notify_target_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_accept_charset_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFormElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFormElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFormElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_action_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFormElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFormElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFormElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_elements_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFormElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFormElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFormElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_encoding_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFormElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFormElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFormElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_enctype_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFormElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFormElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFormElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_length_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFormElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFormElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFormElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_method_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFormElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFormElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFormElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFormElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFormElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFormElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_target_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFormElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFormElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFormElement::from_glib_borrow(this).downcast_unchecked())
}
