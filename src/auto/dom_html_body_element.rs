// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;
use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;

glib_wrapper! {
    pub struct DOMHTMLBodyElement(Object<webkit2_webextension_sys::WebKitDOMHTMLBodyElement, webkit2_webextension_sys::WebKitDOMHTMLBodyElementClass, DOMHTMLBodyElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_body_element_get_type(),
    }
}

pub const NONE_DOMHTML_BODY_ELEMENT: Option<&DOMHTMLBodyElement> = None;

pub trait DOMHTMLBodyElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_a_link(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_background(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_bg_color(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_link(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_text(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_v_link(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_a_link(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_background(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_bg_color(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_link(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_text(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_v_link(&self, value: &str);

    fn connect_property_a_link_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_link_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_v_link_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLBodyElement>> DOMHTMLBodyElementExt for O {
    fn get_a_link(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_body_element_get_a_link(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_background(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_body_element_get_background(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_bg_color(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_body_element_get_bg_color(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_link(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_body_element_get_link(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_body_element_get_text(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_v_link(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_body_element_get_v_link(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn set_a_link(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_body_element_set_a_link(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_background(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_body_element_set_background(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_bg_color(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_body_element_set_bg_color(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_link(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_body_element_set_link(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_text(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_body_element_set_text(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_v_link(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_body_element_set_v_link(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn connect_property_a_link_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_a_link_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLBodyElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLBodyElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLBodyElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::a-link\0".as_ptr() as *const _,
                Some(transmute(notify_a_link_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLBodyElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLBodyElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLBodyElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background\0".as_ptr() as *const _,
                Some(transmute(notify_background_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bg_color_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLBodyElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLBodyElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLBodyElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::bg-color\0".as_ptr() as *const _,
                Some(transmute(notify_bg_color_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_link_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_link_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLBodyElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLBodyElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLBodyElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::link\0".as_ptr() as *const _,
                Some(transmute(notify_link_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLBodyElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLBodyElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLBodyElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute(notify_text_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_v_link_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_v_link_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLBodyElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLBodyElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLBodyElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::v-link\0".as_ptr() as *const _,
                Some(transmute(notify_v_link_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLBodyElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLBodyElement")
    }
}
