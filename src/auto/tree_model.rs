// This file was generated by gir (1644ef1) from gir-files (71d73f0)
// DO NOT EDIT

use Object;
use TreeIter;
use TreeModelFlags;
use TreePath;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct TreeModel(Object<ffi::GtkTreeModel>);

    match fn {
        get_type => || ffi::gtk_tree_model_get_type(),
    }
}

pub trait TreeModelExt {
    //fn foreach(&self, func: /*Unknown conversion*//*Unimplemented*/TreeModelForeachFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    //fn get(&self, iter: &TreeIter, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_column_type(&self, index_: i32) -> glib::types::Type;

    fn get_flags(&self) -> TreeModelFlags;

    fn get_iter(&self, path: &TreePath) -> Option<TreeIter>;

    fn get_iter_first(&self) -> Option<TreeIter>;

    fn get_iter_from_string(&self, path_string: &str) -> Option<TreeIter>;

    fn get_n_columns(&self) -> i32;

    fn get_path(&self, iter: &TreeIter) -> Option<TreePath>;

    fn get_string_from_iter(&self, iter: &TreeIter) -> Option<String>;

    //fn get_valist(&self, iter: &TreeIter, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    fn get_value(&self, iter: &TreeIter, column: i32) -> glib::Value;

    fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter>;

    fn iter_has_child(&self, iter: &TreeIter) -> bool;

    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32;

    fn iter_next(&self, iter: &TreeIter) -> bool;

    fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter>;

    fn iter_parent(&self, child: &TreeIter) -> Option<TreeIter>;

    fn iter_previous(&self, iter: &TreeIter) -> bool;

    fn row_changed(&self, path: &TreePath, iter: &TreeIter);

    fn row_deleted(&self, path: &TreePath);

    fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter);

    fn row_inserted(&self, path: &TreePath, iter: &TreeIter);

    //#[cfg(feature = "v3_10")]
    //fn rows_reordered_with_length(&self, path: &TreePath, iter: Option<&TreeIter>, new_order: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }, length: i32);

    fn sort_new_with_model(&self) -> Option<TreeModel>;

    fn connect_row_changed<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(&self, f: F) -> u64;

    fn connect_row_deleted<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> u64;

    fn connect_row_has_child_toggled<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(&self, f: F) -> u64;

    fn connect_row_inserted<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(&self, f: F) -> u64;

    //fn connect_rows_reordered<Unsupported or ignored types>(&self, f: F) -> u64;
}

impl<O: IsA<TreeModel> + IsA<Object>> TreeModelExt for O {
    //fn foreach(&self, func: /*Unknown conversion*//*Unimplemented*/TreeModelForeachFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_tree_model_foreach() }
    //}

    //fn get(&self, iter: &TreeIter, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_tree_model_get() }
    //}

    fn get_column_type(&self, index_: i32) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gtk_tree_model_get_column_type(self.to_glib_none().0, index_))
        }
    }

    fn get_flags(&self) -> TreeModelFlags {
        unsafe {
            from_glib(ffi::gtk_tree_model_get_flags(self.to_glib_none().0))
        }
    }

    fn get_iter(&self, path: &TreePath) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_get_iter(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(path.to_glib_none().0)));
            if ret { Some(iter) } else { None }
        }
    }

    fn get_iter_first(&self) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_get_iter_first(self.to_glib_none().0, iter.to_glib_none_mut().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn get_iter_from_string(&self, path_string: &str) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_get_iter_from_string(self.to_glib_none().0, iter.to_glib_none_mut().0, path_string.to_glib_none().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn get_n_columns(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_model_get_n_columns(self.to_glib_none().0)
        }
    }

    fn get_path(&self, iter: &TreeIter) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_get_path(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    fn get_string_from_iter(&self, iter: &TreeIter) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_get_string_from_iter(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    //fn get_valist(&self, iter: &TreeIter, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_tree_model_get_valist() }
    //}

    fn get_value(&self, iter: &TreeIter, column: i32) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::gtk_tree_model_get_value(self.to_glib_none().0, mut_override(iter.to_glib_none().0), column, value.to_glib_none_mut().0);
            value
        }
    }

    fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_iter_children(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0)));
            if ret { Some(iter) } else { None }
        }
    }

    fn iter_has_child(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_has_child(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32 {
        unsafe {
            ffi::gtk_tree_model_iter_n_children(self.to_glib_none().0, mut_override(iter.to_glib_none().0))
        }
    }

    fn iter_next(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_next(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_iter_nth_child(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), n));
            if ret { Some(iter) } else { None }
        }
    }

    fn iter_parent(&self, child: &TreeIter) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_iter_parent(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(child.to_glib_none().0)));
            if ret { Some(iter) } else { None }
        }
    }

    fn iter_previous(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_previous(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    fn row_changed(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_changed(self.to_glib_none().0, mut_override(path.to_glib_none().0), mut_override(iter.to_glib_none().0));
        }
    }

    fn row_deleted(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_model_row_deleted(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_has_child_toggled(self.to_glib_none().0, mut_override(path.to_glib_none().0), mut_override(iter.to_glib_none().0));
        }
    }

    fn row_inserted(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_inserted(self.to_glib_none().0, mut_override(path.to_glib_none().0), mut_override(iter.to_glib_none().0));
        }
    }

    //#[cfg(feature = "v3_10")]
    //fn rows_reordered_with_length(&self, path: &TreePath, iter: Option<&TreeIter>, new_order: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }, length: i32) {
    //    unsafe { TODO: call ffi::gtk_tree_model_rows_reordered_with_length() }
    //}

    fn sort_new_with_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_sort_new_with_model(self.to_glib_none().0))
        }
    }

    fn connect_row_changed<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TreePath, &TreeIter) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "row-changed",
                transmute(row_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_row_deleted<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TreePath) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "row-deleted",
                transmute(row_deleted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_row_has_child_toggled<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TreePath, &TreeIter) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "row-has-child-toggled",
                transmute(row_has_child_toggled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_row_inserted<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TreePath, &TreeIter) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "row-inserted",
                transmute(row_inserted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //fn connect_rows_reordered<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Unimplemented new_order: *.Pointer
    //}
}

unsafe extern "C" fn row_changed_trampoline<T>(this: *mut ffi::GtkTreeModel, path: *mut ffi::GtkTreePath, iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer)
where T: IsA<TreeModel> {
    callback_guard!();
    let f: &Box_<Fn(&T, &TreePath, &TreeIter) + 'static> = transmute(f);
    f(&TreeModel::from_glib_none(this).downcast_unchecked(), &from_glib_none(path), &from_glib_none(iter))
}

unsafe extern "C" fn row_deleted_trampoline<T>(this: *mut ffi::GtkTreeModel, path: *mut ffi::GtkTreePath, f: glib_ffi::gpointer)
where T: IsA<TreeModel> {
    callback_guard!();
    let f: &Box_<Fn(&T, &TreePath) + 'static> = transmute(f);
    f(&TreeModel::from_glib_none(this).downcast_unchecked(), &from_glib_none(path))
}

unsafe extern "C" fn row_has_child_toggled_trampoline<T>(this: *mut ffi::GtkTreeModel, path: *mut ffi::GtkTreePath, iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer)
where T: IsA<TreeModel> {
    callback_guard!();
    let f: &Box_<Fn(&T, &TreePath, &TreeIter) + 'static> = transmute(f);
    f(&TreeModel::from_glib_none(this).downcast_unchecked(), &from_glib_none(path), &from_glib_none(iter))
}

unsafe extern "C" fn row_inserted_trampoline<T>(this: *mut ffi::GtkTreeModel, path: *mut ffi::GtkTreePath, iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer)
where T: IsA<TreeModel> {
    callback_guard!();
    let f: &Box_<Fn(&T, &TreePath, &TreeIter) + 'static> = transmute(f);
    f(&TreeModel::from_glib_none(this).downcast_unchecked(), &from_glib_none(path), &from_glib_none(iter))
}
