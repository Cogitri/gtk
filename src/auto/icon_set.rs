// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use IconSource;
use StyleContext;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use cairo;
use ffi;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use gdk;
use gdk_pixbuf;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct IconSet(Shared<ffi::GtkIconSet>);

    match fn {
        ref => |ptr| ffi::gtk_icon_set_ref(ptr),
        unref => |ptr| ffi::gtk_icon_set_unref(ptr),
        get_type => || ffi::gtk_icon_set_get_type(),
    }
}

impl IconSet {
    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn new() -> IconSet {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_icon_set_new())
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn new_from_pixbuf(pixbuf: &gdk_pixbuf::Pixbuf) -> IconSet {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_icon_set_new_from_pixbuf(pixbuf.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn add_source(&self, source: &IconSource) {
        unsafe {
            ffi::gtk_icon_set_add_source(self.to_glib_none().0, source.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn copy(&self) -> Option<IconSet> {
        unsafe {
            from_glib_full(ffi::gtk_icon_set_copy(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn get_sizes(&self) -> Vec<i32> {
        unsafe {
            let mut sizes = ptr::null_mut();
            let mut n_sizes = mem::uninitialized();
            ffi::gtk_icon_set_get_sizes(self.to_glib_none().0, &mut sizes, &mut n_sizes);
            FromGlibContainer::from_glib_full_num(sizes, n_sizes as usize)
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn render_icon_pixbuf(&self, context: &StyleContext, size: i32) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_full(ffi::gtk_icon_set_render_icon_pixbuf(self.to_glib_none().0, context.to_glib_none().0, size))
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn render_icon_surface<'a, P: Into<Option<&'a gdk::Window>>>(&self, context: &StyleContext, size: i32, scale: i32, for_window: P) -> Option<cairo::Surface> {
        let for_window = for_window.into();
        let for_window = for_window.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_icon_set_render_icon_surface(self.to_glib_none().0, context.to_glib_none().0, size, scale, for_window.0))
        }
    }
}

#[cfg_attr(feature = "v3_10", deprecated)]
impl Default for IconSet {
    fn default() -> Self {
        Self::new()
    }
}
