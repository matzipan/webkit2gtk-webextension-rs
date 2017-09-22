// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use Error;
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
    pub struct DOMNamedNodeMap(Object<ffi::WebKitDOMNamedNodeMap>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_named_node_map_get_type(),
    }
}

pub trait DOMNamedNodeMapExt {
    fn get_length(&self) -> libc::c_ulong;

    fn get_named_item(&self, name: &str) -> Option<DOMNode>;

    fn get_named_item_ns(&self, namespaceURI: &str, localName: &str) -> Option<DOMNode>;

    fn item(&self, index: libc::c_ulong) -> Option<DOMNode>;

    fn remove_named_item(&self, name: &str) -> Result<DOMNode, Error>;

    fn remove_named_item_ns(&self, namespaceURI: &str, localName: &str) -> Result<DOMNode, Error>;

    fn set_named_item<P: IsA<DOMNode>>(&self, node: &P) -> Result<DOMNode, Error>;

    fn set_named_item_ns<P: IsA<DOMNode>>(&self, node: &P) -> Result<DOMNode, Error>;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMNamedNodeMap> + IsA<glib::object::Object>> DOMNamedNodeMapExt for O {
    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_named_node_map_get_length(self.to_glib_none().0)
        }
    }

    fn get_named_item(&self, name: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_named_node_map_get_named_item(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_named_item_ns(&self, namespaceURI: &str, localName: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_named_node_map_get_named_item_ns(self.to_glib_none().0, namespaceURI.to_glib_none().0, localName.to_glib_none().0))
        }
    }

    fn item(&self, index: libc::c_ulong) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_named_node_map_item(self.to_glib_none().0, index))
        }
    }

    fn remove_named_item(&self, name: &str) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_named_node_map_remove_named_item(self.to_glib_none().0, name.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_named_item_ns(&self, namespaceURI: &str, localName: &str) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_named_node_map_remove_named_item_ns(self.to_glib_none().0, namespaceURI.to_glib_none().0, localName.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_named_item<P: IsA<DOMNode>>(&self, node: &P) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_named_node_map_set_named_item(self.to_glib_none().0, node.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_named_item_ns<P: IsA<DOMNode>>(&self, node: &P) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_named_node_map_set_named_item_ns(self.to_glib_none().0, node.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::length",
                transmute(notify_length_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_length_trampoline<P>(this: *mut ffi::WebKitDOMNamedNodeMap, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNamedNodeMap> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNamedNodeMap::from_glib_borrow(this).downcast_unchecked())
}
