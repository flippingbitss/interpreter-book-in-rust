use core::fmt;



pub enum Object {
    Integer {
        value: i64
    },
    Bool {
        value: bool
    },
    Null 
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer { value } => write!(f, "{}", value),
            Object::Bool { value } => write!(f, "{}", value),
            Object::Null => write!(f, "nil"),
        }
    }
}
