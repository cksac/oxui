use crate::rendering::{BoxConstraints, RenderConstrainedBox, RenderFlex, RenderImage};
use ctor::ctor;
use std::{
    any::{type_name, Any, TypeId},
    collections::HashMap,
    ptr::{from_raw_parts_mut, DynMetadata},
    sync::RwLock,
};

lazy_static! {
    static ref LAYOUT_VTABLES: RwLock<HashMap<(TypeId, TypeId), Box<dyn Any + Sync + Send>>> =
        RwLock::new(HashMap::new());
}

pub fn register_layout_vt<C, T>()
where
    C: 'static,
    T: Layout<C> + Default + 'static,
{
    let mut vtables = LAYOUT_VTABLES.write().expect("writable LAYOUT_VTABLES");
    let layouter: &mut dyn Layout<C> = &mut T::default();
    let (_, vt) = (&mut *layouter as &mut dyn Layout<C> as *const dyn Layout<C>).to_raw_parts();

    let vt_key = (TypeId::of::<C>(), TypeId::of::<T>());
    let vt_any: Box<dyn Any + Sync + Send> = Box::new(vt);

    vtables.insert(vt_key, vt_any);
}

#[ctor]
fn register_layout_vts() {
    register_layout_vt::<BoxConstraints, RenderImage>();
    register_layout_vt::<BoxConstraints, RenderConstrainedBox>();
    register_layout_vt::<BoxConstraints, RenderFlex>();
}

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
    fn ty_id(&self) -> TypeId;

    fn ty_name(&self) -> &'static str;

    fn size(&self) -> Option<Size>;
}

impl dyn RenderObject {
    #[track_caller]
    pub fn layout<C>(&mut self, constraints: &C)
    where
        C: Any,
    {
        self.layout_parent_use_size(constraints, false)
    }

    #[track_caller]
    pub fn layout_parent_use_size<C>(&mut self, constraints: &C, parent_use_size: bool)
    where
        C: Any,
    {
        let vtables = LAYOUT_VTABLES.read().expect("readable LAYOUT_VTABLES");

        let vt_key = (TypeId::of::<C>(), self.ty_id());
        if let Some(vt) = vtables.get(&vt_key) {
            let (data, _) =
                (&mut *self as &mut dyn RenderObject as *mut dyn RenderObject).to_raw_parts();

            match vt.downcast_ref::<DynMetadata<dyn Layout<C>>>() {
                Some(vtable) => {
                    let trait_obj: &mut dyn Layout<C> =
                        unsafe { &mut *from_raw_parts_mut(data, *vtable) };

                    trait_obj.layout_parent_use_size(constraints, parent_use_size);
                }
                None => {
                    unreachable!(
                        "Layout<{}> vtable for {} found, but downcast to trait vtable fail",
                        type_name::<C>(),
                        self.ty_name()
                    );
                }
            }
        } else {
            panic!(
                "Layout<{}> for {} not registered, please call register_layout_vt first",
                type_name::<C>(),
                self.ty_name()
            );
        }
    }
}
