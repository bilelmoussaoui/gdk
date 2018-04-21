// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ByteOrder;
use Screen;
use VisualType;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Visual(Object<ffi::GdkVisual>);

    match fn {
        get_type => || ffi::gdk_visual_get_type(),
    }
}

impl Visual {
    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_best() -> Visual {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gdk_visual_get_best())
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_best_depth() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_visual_get_best_depth()
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_best_type() -> VisualType {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gdk_visual_get_best_type())
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_best_with_both(depth: i32, visual_type: VisualType) -> Option<Visual> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gdk_visual_get_best_with_both(depth, visual_type.to_glib()))
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_best_with_depth(depth: i32) -> Option<Visual> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gdk_visual_get_best_with_depth(depth))
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_best_with_type(visual_type: VisualType) -> Option<Visual> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gdk_visual_get_best_with_type(visual_type.to_glib()))
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_system() -> Visual {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gdk_visual_get_system())
        }
    }
}

pub trait VisualExt {
    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_bits_per_rgb(&self) -> i32;

    fn get_blue_pixel_details(&self) -> (u32, i32, i32);

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_byte_order(&self) -> ByteOrder;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_colormap_size(&self) -> i32;

    fn get_depth(&self) -> i32;

    fn get_green_pixel_details(&self) -> (u32, i32, i32);

    fn get_red_pixel_details(&self) -> (u32, i32, i32);

    fn get_screen(&self) -> Screen;

    fn get_visual_type(&self) -> VisualType;
}

impl<O: IsA<Visual>> VisualExt for O {
    fn get_bits_per_rgb(&self) -> i32 {
        unsafe {
            ffi::gdk_visual_get_bits_per_rgb(self.to_glib_none().0)
        }
    }

    fn get_blue_pixel_details(&self) -> (u32, i32, i32) {
        unsafe {
            let mut mask = mem::uninitialized();
            let mut shift = mem::uninitialized();
            let mut precision = mem::uninitialized();
            ffi::gdk_visual_get_blue_pixel_details(self.to_glib_none().0, &mut mask, &mut shift, &mut precision);
            (mask, shift, precision)
        }
    }

    fn get_byte_order(&self) -> ByteOrder {
        unsafe {
            from_glib(ffi::gdk_visual_get_byte_order(self.to_glib_none().0))
        }
    }

    fn get_colormap_size(&self) -> i32 {
        unsafe {
            ffi::gdk_visual_get_colormap_size(self.to_glib_none().0)
        }
    }

    fn get_depth(&self) -> i32 {
        unsafe {
            ffi::gdk_visual_get_depth(self.to_glib_none().0)
        }
    }

    fn get_green_pixel_details(&self) -> (u32, i32, i32) {
        unsafe {
            let mut mask = mem::uninitialized();
            let mut shift = mem::uninitialized();
            let mut precision = mem::uninitialized();
            ffi::gdk_visual_get_green_pixel_details(self.to_glib_none().0, &mut mask, &mut shift, &mut precision);
            (mask, shift, precision)
        }
    }

    fn get_red_pixel_details(&self) -> (u32, i32, i32) {
        unsafe {
            let mut mask = mem::uninitialized();
            let mut shift = mem::uninitialized();
            let mut precision = mem::uninitialized();
            ffi::gdk_visual_get_red_pixel_details(self.to_glib_none().0, &mut mask, &mut shift, &mut precision);
            (mask, shift, precision)
        }
    }

    fn get_screen(&self) -> Screen {
        unsafe {
            from_glib_none(ffi::gdk_visual_get_screen(self.to_glib_none().0))
        }
    }

    fn get_visual_type(&self) -> VisualType {
        unsafe {
            from_glib(ffi::gdk_visual_get_visual_type(self.to_glib_none().0))
        }
    }
}
