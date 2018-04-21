// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Display;
use Screen;
use ffi;
use gio;
use glib;
use glib::StaticType;
use glib::Value;
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
    pub struct AppLaunchContext(Object<ffi::GdkAppLaunchContext>);

    match fn {
        get_type => || ffi::gdk_app_launch_context_get_type(),
    }
}

impl AppLaunchContext {
    #[deprecated]
    pub fn new() -> AppLaunchContext {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_app_launch_context_new())
        }
    }
}

#[deprecated]
impl Default for AppLaunchContext {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AppLaunchContextExt {
    fn set_desktop(&self, desktop: i32);

    #[deprecated]
    fn set_display(&self, display: &Display);

    fn set_icon<'a, P: IsA<gio::Icon> + 'a, Q: Into<Option<&'a P>>>(&self, icon: Q);

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P);

    fn set_screen(&self, screen: &Screen);

    fn set_timestamp(&self, timestamp: u32);

    fn get_property_display(&self) -> Option<Display>;

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AppLaunchContext> + IsA<glib::object::Object>> AppLaunchContextExt for O {
    fn set_desktop(&self, desktop: i32) {
        unsafe {
            ffi::gdk_app_launch_context_set_desktop(self.to_glib_none().0, desktop);
        }
    }

    fn set_display(&self, display: &Display) {
        unsafe {
            ffi::gdk_app_launch_context_set_display(self.to_glib_none().0, display.to_glib_none().0);
        }
    }

    fn set_icon<'a, P: IsA<gio::Icon> + 'a, Q: Into<Option<&'a P>>>(&self, icon: Q) {
        let icon = icon.into();
        let icon = icon.to_glib_none();
        unsafe {
            ffi::gdk_app_launch_context_set_icon(self.to_glib_none().0, icon.0);
        }
    }

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P) {
        let icon_name = icon_name.into();
        let icon_name = icon_name.to_glib_none();
        unsafe {
            ffi::gdk_app_launch_context_set_icon_name(self.to_glib_none().0, icon_name.0);
        }
    }

    fn set_screen(&self, screen: &Screen) {
        unsafe {
            ffi::gdk_app_launch_context_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    fn set_timestamp(&self, timestamp: u32) {
        unsafe {
            ffi::gdk_app_launch_context_set_timestamp(self.to_glib_none().0, timestamp);
        }
    }

    fn get_property_display(&self) -> Option<Display> {
        unsafe {
            let mut value = Value::from_type(<Display as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "display".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::display",
                transmute(notify_display_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_display_trampoline<P>(this: *mut ffi::GdkAppLaunchContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppLaunchContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AppLaunchContext::from_glib_borrow(this).downcast_unchecked())
}
