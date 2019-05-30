// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::GString;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ContentDeserializer(Object<ffi::GdkContentDeserializer, ContentDeserializerClass>);

    match fn {
        get_type => || ffi::gdk_content_deserializer_get_type(),
    }
}

impl ContentDeserializer {
    //pub fn get_cancellable(&self) -> /*Ignored*/Option<gio::Cancellable> {
    //    unsafe { TODO: call ffi::gdk_content_deserializer_get_cancellable() }
    //}

    pub fn get_gtype(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gdk_content_deserializer_get_gtype(self.to_glib_none().0))
        }
    }

    //pub fn get_input_stream(&self) -> /*Ignored*/Option<gio::InputStream> {
    //    unsafe { TODO: call ffi::gdk_content_deserializer_get_input_stream() }
    //}

    pub fn get_mime_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gdk_content_deserializer_get_mime_type(self.to_glib_none().0))
        }
    }

    pub fn get_priority(&self) -> i32 {
        unsafe {
            ffi::gdk_content_deserializer_get_priority(self.to_glib_none().0)
        }
    }

    //pub fn get_task_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::gdk_content_deserializer_get_task_data() }
    //}

    //pub fn get_user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::gdk_content_deserializer_get_user_data() }
    //}

    //pub fn get_value(&self) -> /*Ignored*/Option<glib::Value> {
    //    unsafe { TODO: call ffi::gdk_content_deserializer_get_value() }
    //}

    //pub fn return_error(&self, error: /*Ignored*/&mut Error) {
    //    unsafe { TODO: call ffi::gdk_content_deserializer_return_error() }
    //}

    pub fn return_success(&self) {
        unsafe {
            ffi::gdk_content_deserializer_return_success(self.to_glib_none().0);
        }
    }

    //pub fn set_task_data(&self, data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gdk_content_deserializer_set_task_data() }
    //}
}

impl fmt::Display for ContentDeserializer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ContentDeserializer")
    }
}