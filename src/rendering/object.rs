use std::{
    any::{type_name, Any},
    borrow::Borrow,
    ptr::{DynMetadata, NonNull},
};

use typemap::{Key, TypeMap};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Offset {
    pub x: f32,
    pub y: f32,
}

impl Offset {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl std::ops::Add<Offset> for Offset {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        Offset {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn zero() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }
}

impl From<(f32, f32)> for Size {
    fn from(val: (f32, f32)) -> Self {
        Self {
            width: val.0,
            height: val.1,
        }
    }
}

pub trait Layout<C>
where
    C: Sized,
{
    fn layout(&mut self, constraints: &C) {
        self.layout_parent_use_size(constraints, false);
    }

    fn layout_parent_use_size(&mut self, constraints: &C, parent_use_size: bool) {
        self.perform_layout(constraints);
    }

    fn perform_layout(&mut self, constraints: &C);

    fn perform_resize(&mut self, constraints: &C) {}
}

pub trait RenderObject {
    fn size(&self) -> Option<Size>;
}

pub struct BoxedRenderObject {
    data: NonNull<()>, // Box<T>
    vtable: DynMetadata<dyn RenderObject>,
    layout_vtables: TypeMap,
    drop: unsafe fn(*mut ()),
}

impl BoxedRenderObject {
    pub fn layout<C>(&mut self, constraints: &C)
    where
        C: Key,
    {
        if let Some(layout_vt) = self.layout_vtables.get_mut::<C>() {
            let vt_any: &mut dyn Any = layout_vt;
            match vt_any.downcast_ref::<DynMetadata<dyn Layout<C>>>() {
                Some(vtable) => {
                    let layouter: &mut dyn Layout<C> =
                        unsafe { &mut *std::ptr::from_raw_parts_mut(self.data.as_mut(), *vtable) };
                    layouter.perform_layout(constraints.borrow());
                }
                None => {
                    // NOTE: BoxedRenderObjectBuilder enforced `C` has correct `DynMetadata`
                    unreachable!("dyn Layout<{}> found but downcast fail", type_name::<C>());
                }
            }
        } else {
            // runtime error if dynamic dispatch failed
            panic!("Layout<{}> not registered", type_name::<C>());
        }
    }
}

impl RenderObject for BoxedRenderObject {
    fn size(&self) -> Option<Size> {
        let obj: &dyn RenderObject =
            unsafe { &*std::ptr::from_raw_parts(self.data.as_ptr().cast(), self.vtable) };
        obj.size()
    }
}

impl Drop for BoxedRenderObject {
    fn drop(&mut self) {
        unsafe { (self.drop)(self.data.as_ptr()) }
    }
}

unsafe fn v_drop<T>(this: *mut ())
where
    T: 'static,
{
    drop(Box::from_raw(this.cast::<T>()))
}

pub struct BoxedRenderObjectBuilder<T> {
    data: Box<T>,
    vtable: DynMetadata<dyn RenderObject>,
    layout_vtables: TypeMap,
}

impl<T> BoxedRenderObjectBuilder<T>
where
    T: RenderObject + 'static,
{
    pub fn new(obj: T) -> Self {
        let (_, vtable) = (&obj as &dyn RenderObject as *const dyn RenderObject).to_raw_parts();
        BoxedRenderObjectBuilder {
            data: Box::new(obj),
            vtable,
            layout_vtables: TypeMap::new(),
        }
    }

    pub fn register_layout_for<C>(mut self) -> Self
    where
        T: Layout<C> + 'static,
        C: Key<Value = DynMetadata<dyn Layout<C>>>,
    {
        let (_, layout_vtable) =
            (&mut *self.data as &mut dyn Layout<C> as *const dyn Layout<C>).to_raw_parts();

        self.layout_vtables.insert::<C>(layout_vtable);
        self
    }

    pub fn build(self) -> BoxedRenderObject {
        unsafe {
            BoxedRenderObject {
                data: NonNull::new_unchecked(Box::into_raw(self.data)).cast(),
                vtable: self.vtable,
                layout_vtables: self.layout_vtables,
                drop: v_drop::<T>,
            }
        }
    }
}
