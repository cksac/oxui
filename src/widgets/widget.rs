use crate::{
    rendering::{FlexFit, RenderBox, RenderSliver},
    widgets::Flexible,
};
use downcast_rs::{impl_downcast, Downcast};
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::Debug,
    panic::Location,
    rc::Rc,
};
use std::{hash::Hash, sync::RwLock};

pub trait CachedValue: Downcast + Debug {}

impl_downcast!(CachedValue);
impl<T: 'static + Debug> CachedValue for T {}

#[derive(Clone, Copy, Debug)]
pub struct CallSite(&'static Location<'static>);
impl CallSite {
    /// The pointer to the location metadata
    ///
    /// Unique locations are expected to have unique pointers. This
    /// is perhaps not formally guaranteed by the language spec, but
    /// it's hard to imagine how it can be implemented otherwise.
    fn as_ptr(&self) -> *const Location<'static> {
        self.0
    }
}

impl PartialEq for CallSite {
    fn eq(&self, other: &CallSite) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl Eq for CallSite {}

impl Hash for CallSite {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state)
    }
}

impl PartialOrd for CallSite {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_ptr().partial_cmp(&other.as_ptr())
    }
}

impl Ord for CallSite {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ptr().cmp(&other.as_ptr())
    }
}

impl From<&'static Location<'static>> for CallSite {
    fn from(inner: &'static Location<'static>) -> Self {
        CallSite(inner)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub struct StateKey {
    call_site: CallSite,
    slot: usize,
}
impl StateKey {
    pub fn new(caller: impl Into<CallSite>, slot: usize) -> StateKey {
        StateKey {
            call_site: caller.into(),
            slot,
        }
    }
}
#[derive(Debug)]
pub struct BuildContext {
    // state to store call site stable states
    state: RwLock<HashMap<StateKey, Rc<dyn CachedValue>>>,
    // tape to store render object tree
    tape: RefCell<Vec<VecDeque<Rc<dyn CachedValue>>>>,
    end_scope: RefCell<bool>,
    cursor: RefCell<usize>,
}
impl BuildContext {
    pub fn new() -> Self {
        BuildContext {
            state: RwLock::new(HashMap::new()),
            tape: RefCell::new(Vec::from([VecDeque::new()])),
            end_scope: RefCell::new(false),
            cursor: RefCell::new(0),
        }
    }

    pub fn reset_cursor(&mut self) {
        *self.cursor.borrow_mut() = 0;
    }
}

impl BuildContext {
    #[track_caller]
    pub fn state<Init, Return>(&self, init_fn: Init) -> Rc<RefCell<Return>>
    where
        Init: FnOnce() -> Return,
        Return: 'static + Debug,
    {
        self.state_slot(0, init_fn)
    }

    #[track_caller]
    pub fn state_slot<Init, Return>(&self, slot: usize, init_fn: Init) -> Rc<RefCell<Return>>
    where
        Init: FnOnce() -> Return,
        Return: 'static + Debug,
    {
        let mut state = self.state.write().expect("get writable state");

        let key = StateKey::new(Location::caller(), slot);
        if let Some(cache) = state.get(&key).cloned() {
            if let Ok(val) = cache.downcast_rc::<RefCell<Return>>() {
                val
            } else {
                unreachable!()
            }
        } else {
            let obj = Rc::new(RefCell::new(init_fn()));
            state.insert(key, obj.clone());
            obj
        }
    }

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
            .get(*self.cursor.borrow())
            .cloned();

        *self.cursor.borrow_mut() += 1;
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
                    .drain((*self.cursor.borrow() - 1)..);
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
