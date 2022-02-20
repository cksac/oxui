use std::any::TypeId;

pub trait RenderObject {
    fn ty_id(&self) -> TypeId;

    fn ty_name(&self) -> &'static str;
}
