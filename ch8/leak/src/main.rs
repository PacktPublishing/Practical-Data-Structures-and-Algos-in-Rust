use std::cell::Cell;
use std::rc::{Rc, Weak};

struct Leaked(usize);

impl Drop for Leaked {
    fn drop(&mut self) {
        println!("Leaked {} dropped!", self.0);
    }
}

struct Forget<T> {
    _value: T,
    _node: Cell<Option<Rc<Self>>>,
}

impl<T> Forget<T> {
    pub fn new(value: T) -> Weak<Self> {
        let this = Rc::new(Forget {
            _value: value,
            _node: None.into(),
        });

        this._node.set(Some(Rc::clone(&this)));

        Rc::downgrade(&this)
    }
}

fn forget<T>(value: T) {
    Forget::new(value);
}

fn main() {
    let _non_leaked = Leaked(1);
    let leaked = Leaked(2);

    forget(leaked)
}
