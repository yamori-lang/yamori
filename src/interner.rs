use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Eq, PartialEq, Clone, Default, Hash)]
pub struct InternedStr(usize);

#[derive(Clone)]
pub struct Interner {
    indexes: HashMap<String, usize>,
    strings: Vec<String>,
}

impl Interner {
    pub fn new() -> Interner {
        Interner {
            indexes: HashMap::new(),
            strings: Vec::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> InternedStr {
        match self.indexes.find_equiv(&s).map(|x| *x) {
            Some(index) => InternedStr(index),
            None => {
                let index = self.strings.len();
                self.indexes.insert(s.to_string(), index);
                self.strings.push(s.to_string());
                InternedStr(index)
            }
        }
    }

    pub fn get_str(&self, InternedStr(i): InternedStr) -> &str {
        if i < self.strings.len() {
            self.strings.get(i).as_slice()
        } else {
            panic!("Invalid InternedStr {}", i)
        }
    }
}

///Returns a reference to the interner stored in TLD
pub fn get_local_interner() -> Rc<RefCell<Interner>> {
    thread_local!(static key: Rc<RefCell<Interner>> = Rc::new(RefCell::new(Interner::new())));

    key.with(|k| match k.borrow() {
        Some(interner) => interner.clone(),
        None => {
            let interner = Rc::new(RefCell::new(Interner::new()));
            k.replace(Some(interner.clone()));
            interner
        }
    });
}

pub fn set_local_interner(interner: Interner) {
    *(*get_local_interner()).borrow_mut() = interner;
}

pub fn intern(s: &str) -> InternedStr {
    let i = get_local_interner();
    (*i).borrow_mut().intern(s)
}

trait Str {
    fn as_slice(&self) -> &str;
}

impl Str for InternedStr {
    fn as_slice(&self) -> &str {
        let interner = get_local_interner();
        let mut x = (*interner).borrow_mut();
        let r: &str = x.get_str(*self);
        //The interner is task local and will never remove a string so this is safe
        unsafe { ::std::mem::transmute(r) }
    }
}

impl fmt::Display for InternedStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_slice())
    }
}
