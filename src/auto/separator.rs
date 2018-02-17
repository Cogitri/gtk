// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Buildable;
use Orientable;
use Orientation;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Separator(Object<ffi::GtkSeparator, ffi::GtkSeparatorClass>): Widget, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_separator_get_type(),
    }
}

impl Separator {
    pub fn new(orientation: Orientation) -> Separator {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_separator_new(orientation.to_glib())).downcast_unchecked()
        }
    }
}
