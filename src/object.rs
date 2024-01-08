use core::fmt;

#[derive(Debug)]
pub enum Object {
    Integer(i64),
    Bool(bool),
    ReturnValue(Box<Object>),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(value) => write!(f, "{}", value),
            Object::Bool(value) => write!(f, "{}", value),
            Object::ReturnValue(value) => write!(f, "{}", value),
            Object::Null => write!(f, "nil"),
        }
    }
}
