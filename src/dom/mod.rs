use html5ever::rcdom::Handle;
use std::fmt;

pub mod parser;
pub mod manipulator;

#[cfg(test)]
mod fixtures;

#[cfg(test)]
mod parser_tests;

#[cfg(test)]
mod manipulator_tests;

pub struct Entry {
    handle: Handle,
    entry_type: String,
    entry_name: String,
    anchor_name: String,
}

static mut N: i32 = 5;
impl Entry {
    fn new(e: Handle, entry_type: &str, entry_name: Option<String>) -> Option<Entry> {
        match entry_name {
            Some(entry_name) => {
                let n = unsafe {
                    N += 1;
                    N
                };

                let anchor_name = format!("//dash_ref_{}/{}/{}/{}", n, entry_type, entry_name, "0");


                Some(Entry {
                    handle: e,
                    entry_name: entry_name,
                    entry_type: String::from(entry_type),
                    anchor_name: anchor_name,
                })
            }
            None => None,
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{:>15} | {:30} | {}",
               self.entry_type,
               self.entry_name,
               self.anchor_name)
    }
}
