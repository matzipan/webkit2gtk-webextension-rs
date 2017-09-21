// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMDocument;
use DOMEventTarget;
use DOMHTMLCollection;
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
    pub struct DOMHTMLDocument(Object<ffi::WebKitDOMHTMLDocument>): DOMDocument, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_document_get_type(),
    }
}

pub trait DOMHTMLDocumentExt {
    fn capture_events(&self);

    fn clear(&self);

    fn close(&self);

    fn get_alink_color(&self) -> Option<String>;

    fn get_bg_color(&self) -> Option<String>;

    #[cfg(not(feature = "v2_14"))]
    fn get_compat_mode(&self) -> Option<String>;

    #[cfg(not(feature = "v2_14"))]
    fn get_design_mode(&self) -> Option<String>;

    #[cfg(not(feature = "v2_16"))]
    fn get_dir(&self) -> Option<String>;

    #[cfg(not(feature = "v2_14"))]
    fn get_embeds(&self) -> Option<DOMHTMLCollection>;

    fn get_fg_color(&self) -> Option<String>;

    fn get_height(&self) -> libc::c_long;

    fn get_link_color(&self) -> Option<String>;

    #[cfg(not(feature = "v2_14"))]
    fn get_plugins(&self) -> Option<DOMHTMLCollection>;

    #[cfg(not(feature = "v2_14"))]
    fn get_scripts(&self) -> Option<DOMHTMLCollection>;

    fn get_vlink_color(&self) -> Option<String>;

    fn get_width(&self) -> libc::c_long;

    fn release_events(&self);

    fn set_alink_color(&self, value: &str);

    fn set_bg_color(&self, value: &str);

    #[cfg(not(feature = "v2_14"))]
    fn set_design_mode(&self, value: &str);

    #[cfg(not(feature = "v2_16"))]
    fn set_dir(&self, value: &str);

    fn set_fg_color(&self, value: &str);

    fn set_link_color(&self, value: &str);

    fn set_vlink_color(&self, value: &str);

    fn connect_property_alink_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_dir_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_link_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vlink_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLDocument> + IsA<glib::object::Object>> DOMHTMLDocumentExt for O {
    fn capture_events(&self) {
        unsafe {
            ffi::webkit_dom_html_document_capture_events(self.to_glib_none().0);
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::webkit_dom_html_document_clear(self.to_glib_none().0);
        }
    }

    fn close(&self) {
        unsafe {
            ffi::webkit_dom_html_document_close(self.to_glib_none().0);
        }
    }

    fn get_alink_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_alink_color(self.to_glib_none().0))
        }
    }

    fn get_bg_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_bg_color(self.to_glib_none().0))
        }
    }

    #[cfg(not(feature = "v2_14"))]
    fn get_compat_mode(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_compat_mode(self.to_glib_none().0))
        }
    }

    #[cfg(not(feature = "v2_14"))]
    fn get_design_mode(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_design_mode(self.to_glib_none().0))
        }
    }

    #[cfg(not(feature = "v2_16"))]
    fn get_dir(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_dir(self.to_glib_none().0))
        }
    }

    #[cfg(not(feature = "v2_14"))]
    fn get_embeds(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_embeds(self.to_glib_none().0))
        }
    }

    fn get_fg_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_fg_color(self.to_glib_none().0))
        }
    }

    fn get_height(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_document_get_height(self.to_glib_none().0)
        }
    }

    fn get_link_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_link_color(self.to_glib_none().0))
        }
    }

    #[cfg(not(feature = "v2_14"))]
    fn get_plugins(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_plugins(self.to_glib_none().0))
        }
    }

    #[cfg(not(feature = "v2_14"))]
    fn get_scripts(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_scripts(self.to_glib_none().0))
        }
    }

    fn get_vlink_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_vlink_color(self.to_glib_none().0))
        }
    }

    fn get_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_document_get_width(self.to_glib_none().0)
        }
    }

    fn release_events(&self) {
        unsafe {
            ffi::webkit_dom_html_document_release_events(self.to_glib_none().0);
        }
    }

    fn set_alink_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_alink_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_bg_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_bg_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(not(feature = "v2_14"))]
    fn set_design_mode(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_design_mode(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(not(feature = "v2_16"))]
    fn set_dir(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_dir(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_fg_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_fg_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_link_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_link_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_vlink_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_vlink_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_alink_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::alink-color",
                transmute(notify_alink_color_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::bg-color",
                transmute(notify_bg_color_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_dir_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::dir",
                transmute(notify_dir_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_fg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::fg-color",
                transmute(notify_fg_color_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::height",
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_link_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::link-color",
                transmute(notify_link_color_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_vlink_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::vlink-color",
                transmute(notify_vlink_color_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
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

unsafe extern "C" fn notify_alink_color_trampoline<P>(this: *mut ffi::WebKitDOMHTMLDocument, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLDocument> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLDocument::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_bg_color_trampoline<P>(this: *mut ffi::WebKitDOMHTMLDocument, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLDocument> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLDocument::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_dir_trampoline<P>(this: *mut ffi::WebKitDOMHTMLDocument, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLDocument> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLDocument::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_fg_color_trampoline<P>(this: *mut ffi::WebKitDOMHTMLDocument, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLDocument> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLDocument::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::WebKitDOMHTMLDocument, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLDocument> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLDocument::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_link_color_trampoline<P>(this: *mut ffi::WebKitDOMHTMLDocument, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLDocument> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLDocument::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_vlink_color_trampoline<P>(this: *mut ffi::WebKitDOMHTMLDocument, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLDocument> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLDocument::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLDocument, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLDocument> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLDocument::from_glib_borrow(this).downcast_unchecked())
}
