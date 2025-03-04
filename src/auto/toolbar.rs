// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Align;
use Buildable;
use Container;
use IconSize;
use Orientable;
use Orientation;
use ResizeMode;
use ToolItem;
use ToolShell;
use ToolbarStyle;
use Widget;

glib_wrapper! {
    pub struct Toolbar(Object<gtk_sys::GtkToolbar, gtk_sys::GtkToolbarClass, ToolbarClass>) @extends Container, Widget, @implements Buildable, Orientable, ToolShell;

    match fn {
        get_type => || gtk_sys::gtk_toolbar_get_type(),
    }
}

impl Toolbar {
    pub fn new() -> Toolbar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_toolbar_new()).unsafe_cast() }
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ToolbarBuilder {
    icon_size: Option<IconSize>,
    icon_size_set: Option<bool>,
    show_arrow: Option<bool>,
    toolbar_style: Option<ToolbarStyle>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    //style: /*Unknown type*/,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl ToolbarBuilder {
    pub fn new() -> Self {
        Self {
            icon_size: None,
            icon_size_set: None,
            show_arrow: None,
            toolbar_style: None,
            border_width: None,
            child: None,
            resize_mode: None,
            app_paintable: None,
            can_default: None,
            can_focus: None,
            events: None,
            expand: None,
            #[cfg(any(feature = "v3_20", feature = "dox"))]
            focus_on_click: None,
            halign: None,
            has_default: None,
            has_focus: None,
            has_tooltip: None,
            height_request: None,
            hexpand: None,
            hexpand_set: None,
            is_focus: None,
            margin: None,
            margin_bottom: None,
            margin_end: None,
            margin_start: None,
            margin_top: None,
            name: None,
            no_show_all: None,
            opacity: None,
            parent: None,
            receives_default: None,
            sensitive: None,
            tooltip_markup: None,
            tooltip_text: None,
            valign: None,
            vexpand: None,
            vexpand_set: None,
            visible: None,
            width_request: None,
        }
    }

    pub fn build(self) -> Toolbar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref icon_size) = self.icon_size {
            properties.push(("icon-size", icon_size));
        }
        if let Some(ref icon_size_set) = self.icon_size_set {
            properties.push(("icon-size-set", icon_size_set));
        }
        if let Some(ref show_arrow) = self.show_arrow {
            properties.push(("show-arrow", show_arrow));
        }
        if let Some(ref toolbar_style) = self.toolbar_style {
            properties.push(("toolbar-style", toolbar_style));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new(Toolbar::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = Some(icon_size);
        self
    }

    pub fn icon_size_set(mut self, icon_size_set: bool) -> Self {
        self.icon_size_set = Some(icon_size_set);
        self
    }

    pub fn show_arrow(mut self, show_arrow: bool) -> Self {
        self.show_arrow = Some(show_arrow);
        self
    }

    pub fn toolbar_style(mut self, toolbar_style: ToolbarStyle) -> Self {
        self.toolbar_style = Some(toolbar_style);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child(mut self, child: &Widget) -> Self {
        self.child = Some(child.clone());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent(mut self, parent: &Container) -> Self {
        self.parent = Some(parent.clone());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_TOOLBAR: Option<&Toolbar> = None;

pub trait ToolbarExt: 'static {
    fn get_drop_index(&self, x: i32, y: i32) -> i32;

    fn get_item_index<P: IsA<ToolItem>>(&self, item: &P) -> i32;

    fn get_n_items(&self) -> i32;

    fn get_nth_item(&self, n: i32) -> Option<ToolItem>;

    fn get_show_arrow(&self) -> bool;

    fn insert<P: IsA<ToolItem>>(&self, item: &P, pos: i32);

    fn set_drop_highlight_item<P: IsA<ToolItem>>(&self, tool_item: Option<&P>, index_: i32);

    fn set_icon_size(&self, icon_size: IconSize);

    fn set_show_arrow(&self, show_arrow: bool);

    fn set_style(&self, style: ToolbarStyle);

    fn unset_icon_size(&self);

    fn unset_style(&self);

    fn get_property_icon_size_set(&self) -> bool;

    fn set_property_icon_size_set(&self, icon_size_set: bool);

    fn get_property_toolbar_style(&self) -> ToolbarStyle;

    fn set_property_toolbar_style(&self, toolbar_style: ToolbarStyle);

    fn get_item_expand<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_item_expand<T: IsA<Widget>>(&self, item: &T, expand: bool);

    fn get_item_homogeneous<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_item_homogeneous<T: IsA<Widget>>(&self, item: &T, homogeneous: bool);

    fn connect_focus_home_or_end<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_focus_home_or_end(&self, focus_home: bool) -> bool;

    fn connect_orientation_changed<F: Fn(&Self, Orientation) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_popup_context_menu<F: Fn(&Self, i32, i32, i32) -> Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_style_changed<F: Fn(&Self, ToolbarStyle) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_size_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_show_arrow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_toolbar_style_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Toolbar>> ToolbarExt for O {
    fn get_drop_index(&self, x: i32, y: i32) -> i32 {
        unsafe { gtk_sys::gtk_toolbar_get_drop_index(self.as_ref().to_glib_none().0, x, y) }
    }

    fn get_item_index<P: IsA<ToolItem>>(&self, item: &P) -> i32 {
        unsafe {
            gtk_sys::gtk_toolbar_get_item_index(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
            )
        }
    }

    fn get_n_items(&self) -> i32 {
        unsafe { gtk_sys::gtk_toolbar_get_n_items(self.as_ref().to_glib_none().0) }
    }

    fn get_nth_item(&self, n: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(gtk_sys::gtk_toolbar_get_nth_item(
                self.as_ref().to_glib_none().0,
                n,
            ))
        }
    }

    fn get_show_arrow(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_toolbar_get_show_arrow(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert<P: IsA<ToolItem>>(&self, item: &P, pos: i32) {
        unsafe {
            gtk_sys::gtk_toolbar_insert(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
                pos,
            );
        }
    }

    fn set_drop_highlight_item<P: IsA<ToolItem>>(&self, tool_item: Option<&P>, index_: i32) {
        unsafe {
            gtk_sys::gtk_toolbar_set_drop_highlight_item(
                self.as_ref().to_glib_none().0,
                tool_item.map(|p| p.as_ref()).to_glib_none().0,
                index_,
            );
        }
    }

    fn set_icon_size(&self, icon_size: IconSize) {
        unsafe {
            gtk_sys::gtk_toolbar_set_icon_size(self.as_ref().to_glib_none().0, icon_size.to_glib());
        }
    }

    fn set_show_arrow(&self, show_arrow: bool) {
        unsafe {
            gtk_sys::gtk_toolbar_set_show_arrow(
                self.as_ref().to_glib_none().0,
                show_arrow.to_glib(),
            );
        }
    }

    fn set_style(&self, style: ToolbarStyle) {
        unsafe {
            gtk_sys::gtk_toolbar_set_style(self.as_ref().to_glib_none().0, style.to_glib());
        }
    }

    fn unset_icon_size(&self) {
        unsafe {
            gtk_sys::gtk_toolbar_unset_icon_size(self.as_ref().to_glib_none().0);
        }
    }

    fn unset_style(&self) {
        unsafe {
            gtk_sys::gtk_toolbar_unset_style(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_icon_size_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"icon-size-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icon-size-set` getter")
                .unwrap()
        }
    }

    fn set_property_icon_size_set(&self, icon_size_set: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"icon-size-set\0".as_ptr() as *const _,
                Value::from(&icon_size_set).to_glib_none().0,
            );
        }
    }

    fn get_property_toolbar_style(&self) -> ToolbarStyle {
        unsafe {
            let mut value = Value::from_type(<ToolbarStyle as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"toolbar-style\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `toolbar-style` getter")
                .unwrap()
        }
    }

    fn set_property_toolbar_style(&self, toolbar_style: ToolbarStyle) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"toolbar-style\0".as_ptr() as *const _,
                Value::from(&toolbar_style).to_glib_none().0,
            );
        }
    }

    fn get_item_expand<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"expand\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `expand` getter")
                .unwrap()
        }
    }

    fn set_item_expand<T: IsA<Widget>>(&self, item: &T, expand: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"expand\0".as_ptr() as *const _,
                Value::from(&expand).to_glib_none().0,
            );
        }
    }

    fn get_item_homogeneous<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"homogeneous\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `homogeneous` getter")
                .unwrap()
        }
    }

    fn set_item_homogeneous<T: IsA<Widget>>(&self, item: &T, homogeneous: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"homogeneous\0".as_ptr() as *const _,
                Value::from(&homogeneous).to_glib_none().0,
            );
        }
    }

    fn connect_focus_home_or_end<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn focus_home_or_end_trampoline<P, F: Fn(&P, bool) -> bool + 'static>(
            this: *mut gtk_sys::GtkToolbar,
            focus_home: glib_sys::gboolean,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<Toolbar>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Toolbar::from_glib_borrow(this).unsafe_cast(),
                from_glib(focus_home),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"focus-home-or-end\0".as_ptr() as *const _,
                Some(transmute(focus_home_or_end_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_focus_home_or_end(&self, focus_home: bool) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("focus-home-or-end", &[&focus_home])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_focus_home_or_end`")
            .unwrap()
    }

    fn connect_orientation_changed<F: Fn(&Self, Orientation) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn orientation_changed_trampoline<P, F: Fn(&P, Orientation) + 'static>(
            this: *mut gtk_sys::GtkToolbar,
            orientation: gtk_sys::GtkOrientation,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Toolbar>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Toolbar::from_glib_borrow(this).unsafe_cast(),
                from_glib(orientation),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"orientation-changed\0".as_ptr() as *const _,
                Some(transmute(
                    orientation_changed_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_popup_context_menu<F: Fn(&Self, i32, i32, i32) -> Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn popup_context_menu_trampoline<
            P,
            F: Fn(&P, i32, i32, i32) -> Inhibit + 'static,
        >(
            this: *mut gtk_sys::GtkToolbar,
            x: libc::c_int,
            y: libc::c_int,
            button: libc::c_int,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<Toolbar>,
        {
            let f: &F = &*(f as *const F);
            f(&Toolbar::from_glib_borrow(this).unsafe_cast(), x, y, button).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"popup-context-menu\0".as_ptr() as *const _,
                Some(transmute(popup_context_menu_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_style_changed<F: Fn(&Self, ToolbarStyle) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn style_changed_trampoline<P, F: Fn(&P, ToolbarStyle) + 'static>(
            this: *mut gtk_sys::GtkToolbar,
            style: gtk_sys::GtkToolbarStyle,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Toolbar>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Toolbar::from_glib_borrow(this).unsafe_cast(),
                from_glib(style),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"style-changed\0".as_ptr() as *const _,
                Some(transmute(style_changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkToolbar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Toolbar>,
        {
            let f: &F = &*(f as *const F);
            f(&Toolbar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-size\0".as_ptr() as *const _,
                Some(transmute(notify_icon_size_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_icon_size_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_set_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkToolbar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Toolbar>,
        {
            let f: &F = &*(f as *const F);
            f(&Toolbar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-size-set\0".as_ptr() as *const _,
                Some(transmute(
                    notify_icon_size_set_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_arrow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_arrow_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkToolbar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Toolbar>,
        {
            let f: &F = &*(f as *const F);
            f(&Toolbar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-arrow\0".as_ptr() as *const _,
                Some(transmute(notify_show_arrow_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_toolbar_style_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_toolbar_style_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkToolbar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Toolbar>,
        {
            let f: &F = &*(f as *const F);
            f(&Toolbar::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::toolbar-style\0".as_ptr() as *const _,
                Some(transmute(
                    notify_toolbar_style_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Toolbar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Toolbar")
    }
}
