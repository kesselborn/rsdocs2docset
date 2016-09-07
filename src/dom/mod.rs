use html5ever::rcdom::Handle;

pub mod parser;
pub mod manipulator;

#[cfg(test)]
mod fixtures;

#[cfg(test)]
mod parser_tests;

#[cfg(test)]
mod manipulator_tests;

pub enum Entry {
    Const(Handle),
    Enum(Handle),
    Function(Handle),
    Macro(Handle),
    Method(Handle),
    Module(Handle),
    Struct(Handle),
    Trait(Handle),
    Type(Handle),
}
