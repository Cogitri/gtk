// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use AppChooser;
use Box;
use Buildable;
use Container;
use Menu;
use Orientable;
use Widget;
use ffi;
use gio;
use gio_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct AppChooserWidget(Object<ffi::GtkAppChooserWidget, ffi::GtkAppChooserWidgetClass>): Box, Container, Widget, Buildable, Orientable, AppChooser;

    match fn {
        get_type => || ffi::gtk_app_chooser_widget_get_type(),
    }
}

impl AppChooserWidget {
    pub fn new(content_type: &str) -> AppChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_widget_new(content_type.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait AppChooserWidgetExt {
    fn get_default_text(&self) -> Option<String>;

    fn get_show_all(&self) -> bool;

    fn get_show_default(&self) -> bool;

    fn get_show_fallback(&self) -> bool;

    fn get_show_other(&self) -> bool;

    fn get_show_recommended(&self) -> bool;

    fn set_default_text(&self, text: &str);

    fn set_show_all(&self, setting: bool);

    fn set_show_default(&self, setting: bool);

    fn set_show_fallback(&self, setting: bool);

    fn set_show_other(&self, setting: bool);

    fn set_show_recommended(&self, setting: bool);

    fn connect_application_activated<F: Fn(&Self, &gio::AppInfo) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_application_selected<F: Fn(&Self, &gio::AppInfo) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_populate_popup<F: Fn(&Self, &Menu, &gio::AppInfo) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_all_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_default_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_other_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_recommended_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AppChooserWidget> + IsA<glib::object::Object>> AppChooserWidgetExt for O {
    fn get_default_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_widget_get_default_text(self.to_glib_none().0))
        }
    }

    fn get_show_all(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_all(self.to_glib_none().0))
        }
    }

    fn get_show_default(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_default(self.to_glib_none().0))
        }
    }

    fn get_show_fallback(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_fallback(self.to_glib_none().0))
        }
    }

    fn get_show_other(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_other(self.to_glib_none().0))
        }
    }

    fn get_show_recommended(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_recommended(self.to_glib_none().0))
        }
    }

    fn set_default_text(&self, text: &str) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_default_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_show_all(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_all(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_show_default(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_default(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_show_fallback(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_fallback(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_show_other(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_other(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_show_recommended(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_recommended(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn connect_application_activated<F: Fn(&Self, &gio::AppInfo) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gio::AppInfo) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "application-activated",
                transmute(application_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_application_selected<F: Fn(&Self, &gio::AppInfo) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gio::AppInfo) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "application-selected",
                transmute(application_selected_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_populate_popup<F: Fn(&Self, &Menu, &gio::AppInfo) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Menu, &gio::AppInfo) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "populate-popup",
                transmute(populate_popup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_default_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::default-text",
                transmute(notify_default_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_all_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-all",
                transmute(notify_show_all_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_default_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-default",
                transmute(notify_show_default_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-fallback",
                transmute(notify_show_fallback_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_other_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-other",
                transmute(notify_show_other_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_recommended_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-recommended",
                transmute(notify_show_recommended_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn application_activated_trampoline<P>(this: *mut ffi::GtkAppChooserWidget, application: *mut gio_ffi::GAppInfo, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &&(Fn(&P, &gio::AppInfo) + 'static) = transmute(f);
    f(&AppChooserWidget::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(application))
}

unsafe extern "C" fn application_selected_trampoline<P>(this: *mut ffi::GtkAppChooserWidget, application: *mut gio_ffi::GAppInfo, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &&(Fn(&P, &gio::AppInfo) + 'static) = transmute(f);
    f(&AppChooserWidget::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(application))
}

unsafe extern "C" fn populate_popup_trampoline<P>(this: *mut ffi::GtkAppChooserWidget, menu: *mut ffi::GtkMenu, application: *mut gio_ffi::GAppInfo, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &&(Fn(&P, &Menu, &gio::AppInfo) + 'static) = transmute(f);
    f(&AppChooserWidget::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(menu), &from_glib_borrow(application))
}

unsafe extern "C" fn notify_default_text_trampoline<P>(this: *mut ffi::GtkAppChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AppChooserWidget::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_all_trampoline<P>(this: *mut ffi::GtkAppChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AppChooserWidget::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_default_trampoline<P>(this: *mut ffi::GtkAppChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AppChooserWidget::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_fallback_trampoline<P>(this: *mut ffi::GtkAppChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AppChooserWidget::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_other_trampoline<P>(this: *mut ffi::GtkAppChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AppChooserWidget::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_recommended_trampoline<P>(this: *mut ffi::GtkAppChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserWidget> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AppChooserWidget::from_glib_borrow(this).downcast_unchecked())
}
