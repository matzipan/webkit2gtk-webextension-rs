// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct Frame(Object<ffi::WebKitFrame, ffi::WebKitFrameClass>);

    match fn {
        get_type => || ffi::webkit_frame_get_type(),
    }
}

pub const NONE_FRAME: Option<&Frame> = None;

pub trait FrameExt: 'static {
    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    #[doc(alias = "webkit_frame_get_id")]
    fn get_id(&self) -> u64;

    //#[cfg_attr(feature = "v2_22", deprecated)]
    //#[cfg(any(feature = "v2_2", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    //#[doc(alias = "webkit_frame_get_javascript_context_for_script_world")]
    //fn get_javascript_context_for_script_world<P: IsA<ScriptWorld>>(&self, world: &P) -> /*Ignored*/Option<java_script_core::GlobalContextRef>;

    //#[cfg_attr(feature = "v2_22", deprecated)]
    //#[cfg(any(feature = "v2_2", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    //#[doc(alias = "webkit_frame_get_javascript_global_context")]
    //fn get_javascript_global_context(&self) -> /*Ignored*/Option<java_script_core::GlobalContextRef>;

    //#[cfg(any(feature = "v2_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    //#[doc(alias = "webkit_frame_get_js_context")]
    //fn get_js_context(&self) -> /*Ignored*/Option<java_script_core::Context>;

    //#[cfg(any(feature = "v2_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    //#[doc(alias = "webkit_frame_get_js_context_for_script_world")]
    //fn get_js_context_for_script_world<P: IsA<ScriptWorld>>(&self, world: &P) -> /*Ignored*/Option<java_script_core::Context>;

    //#[cfg(any(feature = "v2_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    //#[doc(alias = "webkit_frame_get_js_value_for_dom_object")]
    //fn get_js_value_for_dom_object<P: IsA<DOMObject>>(&self, dom_object: &P) -> /*Ignored*/Option<java_script_core::Value>;

    //#[cfg(any(feature = "v2_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    //#[doc(alias = "webkit_frame_get_js_value_for_dom_object_in_script_world")]
    //fn get_js_value_for_dom_object_in_script_world<P: IsA<DOMObject>, Q: IsA<ScriptWorld>>(&self, dom_object: &P, world: &Q) -> /*Ignored*/Option<java_script_core::Value>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_frame_get_uri")]
    fn get_uri(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_frame_is_main_frame")]
    fn is_main_frame(&self) -> bool;
}

impl<O: IsA<Frame>> FrameExt for O {
    #[cfg(any(feature = "v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    fn get_id(&self) -> u64 {
        unsafe {
            ffi::webkit_frame_get_id(self.as_ref().to_glib_none().0)
        }
    }

    //#[cfg(any(feature = "v2_2", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    //fn get_javascript_context_for_script_world<P: IsA<ScriptWorld>>(&self, world: &P) -> /*Ignored*/Option<java_script_core::GlobalContextRef> {
    //    unsafe { TODO: call ffi:webkit_frame_get_javascript_context_for_script_world() }
    //}

    //#[cfg(any(feature = "v2_2", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    //fn get_javascript_global_context(&self) -> /*Ignored*/Option<java_script_core::GlobalContextRef> {
    //    unsafe { TODO: call ffi:webkit_frame_get_javascript_global_context() }
    //}

    //#[cfg(any(feature = "v2_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    //fn get_js_context(&self) -> /*Ignored*/Option<java_script_core::Context> {
    //    unsafe { TODO: call ffi:webkit_frame_get_js_context() }
    //}

    //#[cfg(any(feature = "v2_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    //fn get_js_context_for_script_world<P: IsA<ScriptWorld>>(&self, world: &P) -> /*Ignored*/Option<java_script_core::Context> {
    //    unsafe { TODO: call ffi:webkit_frame_get_js_context_for_script_world() }
    //}

    //#[cfg(any(feature = "v2_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    //fn get_js_value_for_dom_object<P: IsA<DOMObject>>(&self, dom_object: &P) -> /*Ignored*/Option<java_script_core::Value> {
    //    unsafe { TODO: call ffi:webkit_frame_get_js_value_for_dom_object() }
    //}

    //#[cfg(any(feature = "v2_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    //fn get_js_value_for_dom_object_in_script_world<P: IsA<DOMObject>, Q: IsA<ScriptWorld>>(&self, dom_object: &P, world: &Q) -> /*Ignored*/Option<java_script_core::Value> {
    //    unsafe { TODO: call ffi:webkit_frame_get_js_value_for_dom_object_in_script_world() }
    //}

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn get_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_frame_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn is_main_frame(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_frame_is_main_frame(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Frame")
    }
}
