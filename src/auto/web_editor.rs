// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use std::mem::transmute;
use webkit2_webextension_sys;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use WebPage;

glib_wrapper! {
    pub struct WebEditor(Object<webkit2_webextension_sys::WebKitWebEditor, webkit2_webextension_sys::WebKitWebEditorClass, WebEditorClass>);

    match fn {
        get_type => || webkit2_webextension_sys::webkit_web_editor_get_type(),
    }
}

pub const NONE_WEB_EDITOR: Option<&WebEditor> = None;

pub trait WebEditorExt: 'static {
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_page(&self) -> Option<WebPage>;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebEditor>> WebEditorExt for O {
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_page(&self) -> Option<WebPage> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_web_editor_get_page(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn selection_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitWebEditor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<WebEditor>,
        {
            let f: &F = &*(f as *const F);
            f(&WebEditor::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selection-changed\0".as_ptr() as *const _,
                Some(transmute(selection_changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for WebEditor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WebEditor")
    }
}
