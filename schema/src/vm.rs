mod parser;

pub use parser::parse;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Arithmetic(ArithmeticCommand),
    MemoryAccess(MemoryAccessCommand),
    Function {
        name: Label,
        local_variable_count: u16,
    },
    Call {
        name: Label,
        args_count: u16,
    },
    Return,
    Label(Label),
    Goto(Label),
    IfGoto(Label),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label(String);

impl Label {
    pub fn new(str: &str) -> Self {
        Self(str.to_string())
    }
    pub fn get(&self) -> &str {
        &self.0
    }
    pub fn get_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArithmeticCommand {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessType {
    Push,
    Pop,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryAccessCommand {
    pub access_type: AccessType,
    pub segment: Segment,
    pub index: Index,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index(u16);
impl Index {
    pub fn new(v: u16) -> Self {
        Self(v)
    }
    pub fn get(&self) -> u16 {
        self.0
    }
}
