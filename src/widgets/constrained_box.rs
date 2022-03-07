use crate::{
    rendering::{BoxConstraints, RenderConstrainedBox},
    widgets::{Element, Widget},
};

pub struct ConstrainedBox {
    pub constraints: BoxConstraints,
}

impl Default for ConstrainedBox {
    fn default() -> Self {
        Self {
            constraints: BoxConstraints::expand(),
        }
    }
}

impl Widget for ConstrainedBox {
    #[topo::nested]
    fn build(&self) -> Element {
        RenderConstrainedBox::new(self.constraints).into()    
    }
}
