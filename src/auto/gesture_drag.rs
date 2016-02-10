// This file was generated by gir (17af302) from gir-files (11e0e6d)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(feature = "3.14")]
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
#[cfg(feature = "3.14")]
use std::mem;

glib_wrapper! {
    pub struct GestureDrag(Object<ffi::GtkGestureDrag>): EventController, Gesture, GestureSingle;

    match fn {
        get_type => || ffi::gtk_gesture_drag_get_type(),
    }
}

impl GestureDrag {
    #[cfg(feature = "3.14")]
    pub fn new<T: IsA<Widget>>(widget: &T) -> GestureDrag {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_drag_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait GestureDragExt {
    #[cfg(feature = "3.14")]
    fn get_offset(&self) -> Option<(f64, f64)>;

    #[cfg(feature = "3.14")]
    fn get_start_point(&self) -> Option<(f64, f64)>;
}

impl<O: IsA<GestureDrag>> GestureDragExt for O {
    #[cfg(feature = "3.14")]
    fn get_offset(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_drag_get_offset(self.to_glib_none().0, &mut x, &mut y));
            if ret { Some((x, y)) } else { None }
        }
    }

    #[cfg(feature = "3.14")]
    fn get_start_point(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_drag_get_start_point(self.to_glib_none().0, &mut x, &mut y));
            if ret { Some((x, y)) } else { None }
        }
    }
}
