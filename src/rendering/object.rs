use std::{
    any::{type_name, Any, TypeId},
    borrow::Borrow,
    collections::HashMap,
    ptr::{from_raw_parts_mut, DynMetadata},
};

use crate::rendering::{BoxConstraints, RenderConstrainedBox, RenderImage};

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

    fn layout_parent_use_size(&mut self, constraints: &C, _parent_use_size: bool) {
        self.perform_layout(constraints);
    }

    fn perform_layout(&mut self, constraints: &C);

    fn perform_resize(&mut self, _constraints: &C) {}
}

pub trait RenderObject {
    fn type_id(&self) -> TypeId;
    fn size(&self) -> Option<Size>;
}

fn registered_layouter<T, C>(vtables: &mut HashMap<(TypeId, TypeId), Box<dyn Any + Sync + Send>>)
where
    T: Layout<C> + Default + 'static,
    C: 'static,
{
    let layouter: &mut dyn Layout<C> = &mut T::default();
    let (_, layout_vtable) =
        (&mut *layouter as &mut dyn Layout<C> as *const dyn Layout<C>).to_raw_parts();

    let type_id = (TypeId::of::<T>(), TypeId::of::<C>());
    let vt_any: Box<dyn Any + Sync + Send> = Box::new(layout_vtable);

    vtables.insert(type_id, vt_any);
}

lazy_static! {
    static ref LAYOUT_VTABLES: HashMap<(TypeId, TypeId), Box<dyn Any + Sync + Send>> = {
        let mut vtables = HashMap::new();

        registered_layouter::<RenderConstrainedBox, BoxConstraints>(&mut vtables);
        registered_layouter::<RenderImage, BoxConstraints>(&mut vtables);

        vtables
    };
}

impl dyn RenderObject {
    pub fn layout<C>(&mut self, constraints: &C)
    where
        C: Any,
    {
        let vt_key = (RenderObject::type_id(self), TypeId::of::<C>());
        if let Some(layout_vt) = LAYOUT_VTABLES.get(&vt_key) {
            let (data, _) =
                (&mut *self as &mut dyn RenderObject as *mut dyn RenderObject).to_raw_parts();

            match layout_vt.downcast_ref::<DynMetadata<dyn Layout<C>>>() {
                Some(vtable) => {
                    let layouter: &mut dyn Layout<C> =
                        unsafe { &mut *from_raw_parts_mut(data, *vtable) };

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
