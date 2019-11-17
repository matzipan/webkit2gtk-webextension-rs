// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use DOMXPathResult;
use glib;
use glib::object::IsA;
use glib::translate::*;
use libc;
use std::fmt;
use std::ptr;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMXPathExpression(Object<webkit2_webextension_sys::WebKitDOMXPathExpression, webkit2_webextension_sys::WebKitDOMXPathExpressionClass, DOMXPathExpressionClass>) @extends DOMObject;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_xpath_expression_get_type(),
    }
}

pub const NONE_DOMX_PATH_EXPRESSION: Option<&DOMXPathExpression> = None;

pub trait DOMXPathExpressionExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn evaluate<P: IsA<DOMNode>, Q: IsA<DOMXPathResult>>(&self, contextNode: &P, type_: libc::c_ushort, inResult: &Q) -> Result<DOMXPathResult, glib::Error>;
}

impl<O: IsA<DOMXPathExpression>> DOMXPathExpressionExt for O {
    fn evaluate<P: IsA<DOMNode>, Q: IsA<DOMXPathResult>>(&self, contextNode: &P, type_: libc::c_ushort, inResult: &Q) -> Result<DOMXPathResult, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_xpath_expression_evaluate(self.as_ref().to_glib_none().0, contextNode.as_ref().to_glib_none().0, type_, inResult.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for DOMXPathExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMXPathExpression")
    }
}
