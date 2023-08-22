// This file is a part of the Rust Data Reader Library
// Copyright 2018 Robert Carson
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

use anyhow::Error;
use std::str::FromStr;
use std::str;

#[macro_use]
#[doc(hidden)]
mod macro_src;

/// Contains various float type readers
pub mod float_reader;
/// Contains various integer type readers
pub mod int_reader;
/// Contains various primitive type readers that don't fit into the other reader modules
pub mod prim_reader;
/// Contains various unsigned integer type readers
pub mod uint_reader;
/// Contains the results from parse_txt or load_txt!
pub mod reader_results;
/// Contains the functions that will parse a file and return a RawReaderResults
pub mod parser;
pub(crate) mod parser_core;
/// Contains a couple functions that are useful for parsing files
pub mod parser_utility;

pub use self::float_reader::*;
pub use self::int_reader::*;
pub use self::prim_reader::*;
pub use self::uint_reader::*;
pub use self::macro_src::*;
pub use self::reader_results::*;
pub use self::parser::parse_txt;
pub use self::parser_utility::*;

//This value is similar in value to the one found in BurntSushi's CSV buffer size
//Our's is just 4x as large.
const BUF_SIZE: usize = 8 * (1 << 12);
///The type of delimiter that we can use
pub enum Delimiter {
    WhiteSpace,
    Any(u8),
}

///ReaderParams tells us what our reader should be doing.
///
///delimiter - the delimiter that tells us what our data fields are seperated by
///
/// skip_header - an optional field that tells us whether or not we should skip so many lines that are not
///     comment lines from the beginning of the file
///
/// skip_footer - an optional field that tells us whether or not we should skip so many lines that are not
///     comment lines from the end of the file
///
/// usecols - an optional field that tells us what column numbers we should be using from the data field
///     where these values should be >= 0. Values are 0 indexed here.
///
/// max_rows - an optional field that tells us the maximum number of rows we should use from the file
/// row_format - a required field that tells us whether or not the file should be read in row major or column major
///              Using ..Default::default() it defaults to being true to preserve old behavior of the code.
// is_string - an optional field that tells us if the string passed is a string or file
pub struct ReaderParams {
    pub comments: Option<u8>,
    pub delimiter: Delimiter,
    pub skip_header: Option<usize>,
    pub skip_footer: Option<usize>,
    pub usecols: Option<Vec<usize>>,
    pub max_rows: Option<usize>,
    pub row_format: bool,
    // pub is_string: Option<bool>,
}

///You can use the default constructor like this:
///
///let params = ReaderParams::default(); or you could do
///
///something like -     
///let params = ReaderParams{
///        comments: Some(b'%'),
///        ..Default::default()
///};
impl Default for ReaderParams {
    fn default() -> ReaderParams {
        ReaderParams {
            comments: Some(b'#'),
            delimiter: Delimiter::WhiteSpace,
            skip_header: None,
            skip_footer: None,
            usecols: None,
            max_rows: None,
            row_format: true,
            // is_string: None,
        }
    }
}
