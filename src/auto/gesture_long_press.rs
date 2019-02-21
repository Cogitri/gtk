// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
use Widget;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureLongPress(Object<ffi::GtkGestureLongPress, ffi::GtkGestureLongPressClass, GestureLongPressClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_long_press_get_type(),
    }
}

impl GestureLongPress {
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureLongPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_long_press_new(widget.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_GESTURE_LONG_PRESS: Option<&GestureLongPress> = None;

pub trait GestureLongPressExt: 'static {
    fn get_property_delay_factor(&self) -> f64;

    fn set_property_delay_factor(&self, delay_factor: f64);

    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_pressed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_delay_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureLongPress>> GestureLongPressExt for O {
    fn get_property_delay_factor(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"delay-factor\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_delay_factor(&self, delay_factor: f64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"delay-factor\0".as_ptr() as *const _, Value::from(&delay_factor).to_glib_none().0);
        }
    }

    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"cancelled\0".as_ptr() as *const _,
                Some(transmute(cancelled_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_pressed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pressed\0".as_ptr() as *const _,
                Some(transmute(pressed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_delay_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::delay-factor\0".as_ptr() as *const _,
                Some(transmute(notify_delay_factor_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn cancelled_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkGestureLongPress, f: glib_ffi::gpointer)
where P: IsA<GestureLongPress> {
    let f: &F = transmute(f);
    f(&GestureLongPress::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn pressed_trampoline<P, F: Fn(&P, f64, f64) + 'static>(this: *mut ffi::GtkGestureLongPress, x: libc::c_double, y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureLongPress> {
    let f: &F = transmute(f);
    f(&GestureLongPress::from_glib_borrow(this).unsafe_cast(), x, y)
}

unsafe extern "C" fn notify_delay_factor_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkGestureLongPress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GestureLongPress> {
    let f: &F = transmute(f);
    f(&GestureLongPress::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for GestureLongPress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureLongPress")
    }
}
