use html5ever::rcdom::Handle;
use std::fmt;
use url::percent_encoding::{QUERY_ENCODE_SET, percent_encode};

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
    pub entry_type: String,
    pub entry_name: String,
    pub anchor_name: String,
    pub is_section: bool,
}

static mut N: i32 = 5;
impl Entry {
    fn new(e: Handle, entry_type: &str, entry_name: Option<String>, is_section: bool)
           -> Option<Entry> {
        match entry_name {
            Some(entry_name) => {

                let anchor_name = format!("//dash_ref_{id}/{type}/{name}/{is_section}",
                                          id = unsafe {
                                              N += 1;
                                              N
                                          },
                                          type = entry_type,
                                          name = percent_encode(entry_name.as_bytes(),
                                                                QUERY_ENCODE_SET)
                                                     .collect::<String>(),
                                          is_section = if is_section { "1" } else { "0" });


                Some(Entry {
                    handle: e,
                    entry_name: entry_name,
                    entry_type: String::from(entry_type),
                    anchor_name: anchor_name,
                    is_section: is_section,
                })
            }
            None => None,
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{:>15} | {:30} | {:5} | {}",
               self.entry_type,
               self.entry_name,
               self.is_section,
               self.anchor_name)
    }
}
