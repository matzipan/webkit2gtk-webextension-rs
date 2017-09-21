// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Frame(Object<ffi::WebKitFrame>);

    match fn {
        get_type => || ffi::webkit_frame_get_type(),
    }
}

pub trait FrameExt {
    //#[cfg(feature = "v2_2")]
    //fn get_javascript_context_for_script_world(&self, world: &ScriptWorld) -> /*Ignored*/Option<java_script_core::GlobalContext>;

    //#[cfg(feature = "v2_2")]
    //fn get_javascript_global_context(&self) -> /*Ignored*/Option<java_script_core::GlobalContext>;

    #[cfg(feature = "v2_2")]
    fn get_uri(&self) -> Option<String>;

    #[cfg(feature = "v2_2")]
    fn is_main_frame(&self) -> bool;
}

impl<O: IsA<Frame>> FrameExt for O {
    //#[cfg(feature = "v2_2")]
    //fn get_javascript_context_for_script_world(&self, world: &ScriptWorld) -> /*Ignored*/Option<java_script_core::GlobalContext> {
    //    unsafe { TODO: call ffi::webkit_frame_get_javascript_context_for_script_world() }
    //}

    //#[cfg(feature = "v2_2")]
    //fn get_javascript_global_context(&self) -> /*Ignored*/Option<java_script_core::GlobalContext> {
    //    unsafe { TODO: call ffi::webkit_frame_get_javascript_global_context() }
    //}

    #[cfg(feature = "v2_2")]
    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_frame_get_uri(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    fn is_main_frame(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_frame_is_main_frame(self.to_glib_none().0))
        }
    }
}
