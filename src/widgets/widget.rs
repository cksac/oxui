use crate::{
    rendering::{FlexFit, RenderBox, RenderSliver},
    widgets::Flexible,
};
use downcast_rs::{impl_downcast, Downcast};
use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};

pub trait CachedValue: Downcast + Debug {}

impl_downcast!(CachedValue);
impl<T: 'static + Debug> CachedValue for T {}

#[derive(Debug)]
pub struct BuildContext {
    tape: RefCell<Vec<VecDeque<Rc<dyn CachedValue>>>>,
    end_scope: RefCell<bool>,
    index: RefCell<usize>,
}
impl BuildContext {
    pub fn new() -> Self {
        BuildContext {
            tape: RefCell::new(Vec::from([VecDeque::new()])),
            end_scope: RefCell::new(false),
            index: RefCell::new(0),
        }
    }

    pub fn reset_index(&mut self) {
        *self.index.borrow_mut() = 0;
    }
}

impl BuildContext {
    pub fn once<Init, Return>(&self, init_fn: Init) -> Rc<RefCell<Return>>
    where
        Init: FnOnce() -> Return,
        Return: 'static + Debug,
    {
        self.once_with(init_fn, |_| ())
    }

    pub fn once_with<Init, Update, Return>(
        &self,
        init_fn: Init,
        update_fn: Update,
    ) -> Rc<RefCell<Return>>
    where
        Init: FnOnce() -> Return,
        Update: FnOnce(&mut Return),
        Return: 'static + Debug,
    {
        //println!("index {:?}, {:#?}", *self.index.borrow(), self.tape);

        let cache = self
            .tape
            .borrow()
            .last()
            .unwrap()
            .get(*self.index.borrow())
            .cloned();

        *self.index.borrow_mut() += 1;
        if cache.is_some() {
            let cache = cache.unwrap();
            if let Ok(val) = cache.downcast_rc::<RefCell<Return>>() {
                //println!("get cache");
                let mut val_mut = val.borrow_mut();
                update_fn(&mut val_mut);
                return val.clone();
            } else {
                // TODO:
                // different type, remove rest of the tree?
                // or just remove the mismatch element?
                self.tape
                    .borrow_mut()
                    .last_mut()
                    .unwrap()
                    .drain((*self.index.borrow() - 1)..);
            }
        }
        self.begin();
        let obj = Rc::new(RefCell::new(init_fn()));
        //println!("miss cache");

        if *self.end_scope.borrow() {
            self.tape
                .borrow_mut()
                .last_mut()
                .unwrap()
                .push_front(obj.clone());
            *self.end_scope.borrow_mut() = false;
        } else {
            self.tape
                .borrow_mut()
                .last_mut()
                .unwrap()
                .push_back(obj.clone());
        }
        self.end();
        obj
    }

    fn begin(&self) {
        self.tape.borrow_mut().push(VecDeque::new());
    }

    fn end(&self) {
        let v = self.tape.borrow_mut().pop();
        if let Some(v) = v {
            if let Some(prev_scope) = self.tape.borrow_mut().last_mut() {
                for item in v.into_iter() {
                    prev_scope.push_back(item);
                }
            }
            *self.end_scope.borrow_mut() = true;
        }
    }
}

pub trait Widget: Debug {
    fn create(&self, context: &BuildContext) -> Rc<RefCell<dyn RenderBox>>;

    fn into_flexible(self, flex: usize, fit: FlexFit) -> Flexible
    where
        Self: 'static + Sized,
    {
        Flexible {
            flex,
            fit,
            child: Box::new(self),
        }
    }
}

pub trait SliverWidget: Debug {
    fn create(&self, context: &BuildContext) -> Rc<RefCell<dyn RenderSliver>>;
}
