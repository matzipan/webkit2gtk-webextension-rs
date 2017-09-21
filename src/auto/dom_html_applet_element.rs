// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
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
    pub struct DOMHTMLAppletElement(Object<ffi::WebKitDOMHTMLAppletElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_applet_element_get_type(),
    }
}

pub trait DOMHTMLAppletElementExt {
    fn get_align(&self) -> Option<String>;

    fn get_alt(&self) -> Option<String>;

    fn get_archive(&self) -> Option<String>;

    fn get_code(&self) -> Option<String>;

    fn get_code_base(&self) -> Option<String>;

    fn get_height(&self) -> Option<String>;

    fn get_hspace(&self) -> libc::c_long;

    fn get_name(&self) -> Option<String>;

    fn get_object(&self) -> Option<String>;

    fn get_vspace(&self) -> libc::c_long;

    fn get_width(&self) -> Option<String>;

    fn set_align(&self, value: &str);

    fn set_alt(&self, value: &str);

    fn set_archive(&self, value: &str);

    fn set_code(&self, value: &str);

    fn set_code_base(&self, value: &str);

    fn set_height(&self, value: &str);

    fn set_hspace(&self, value: libc::c_long);

    fn set_name(&self, value: &str);

    fn set_object(&self, value: &str);

    fn set_vspace(&self, value: libc::c_long);

    fn set_width(&self, value: &str);

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_alt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_archive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_code_base_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_object_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLAppletElement> + IsA<glib::object::Object>> DOMHTMLAppletElementExt for O {
    fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_applet_element_get_align(self.to_glib_none().0))
        }
    }

    fn get_alt(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_applet_element_get_alt(self.to_glib_none().0))
        }
    }

    fn get_archive(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_applet_element_get_archive(self.to_glib_none().0))
        }
    }

    fn get_code(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_applet_element_get_code(self.to_glib_none().0))
        }
    }

    fn get_code_base(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_applet_element_get_code_base(self.to_glib_none().0))
        }
    }

    fn get_height(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_applet_element_get_height(self.to_glib_none().0))
        }
    }

    fn get_hspace(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_applet_element_get_hspace(self.to_glib_none().0)
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_applet_element_get_name(self.to_glib_none().0))
        }
    }

    fn get_object(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_applet_element_get_object(self.to_glib_none().0))
        }
    }

    fn get_vspace(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_applet_element_get_vspace(self.to_glib_none().0)
        }
    }

    fn get_width(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_applet_element_get_width(self.to_glib_none().0))
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_applet_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_alt(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_applet_element_set_alt(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_archive(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_applet_element_set_archive(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_code(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_applet_element_set_code(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_code_base(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_applet_element_set_code_base(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_height(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_applet_element_set_height(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_hspace(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_applet_element_set_hspace(self.to_glib_none().0, value);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_applet_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_object(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_applet_element_set_object(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_vspace(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_applet_element_set_vspace(self.to_glib_none().0, value);
        }
    }

    fn set_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_applet_element_set_width(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::align",
                transmute(notify_align_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_alt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::alt",
                transmute(notify_alt_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_archive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::archive",
                transmute(notify_archive_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::code",
                transmute(notify_code_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_code_base_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::code-base",
                transmute(notify_code_base_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::height",
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hspace",
                transmute(notify_hspace_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_object_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::object",
                transmute(notify_object_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_vspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::vspace",
                transmute(notify_vspace_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::width",
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_align_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAppletElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAppletElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAppletElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_alt_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAppletElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAppletElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAppletElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_archive_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAppletElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAppletElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAppletElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_code_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAppletElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAppletElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAppletElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_code_base_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAppletElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAppletElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAppletElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAppletElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAppletElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAppletElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hspace_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAppletElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAppletElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAppletElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAppletElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAppletElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAppletElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_object_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAppletElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAppletElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAppletElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_vspace_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAppletElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAppletElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAppletElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAppletElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAppletElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAppletElement::from_glib_borrow(this).downcast_unchecked())
}
