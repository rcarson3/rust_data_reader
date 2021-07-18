// This file is a part of the mori - Material Orientation Library in Rust
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

use bytecount;
use lexical;
use fast_float;
use memchr::memchr2_iter;
use std::fs::File;
use std::io::{BufRead};
#[cfg(not(feature = "mmap"))]
use std::io::{BufReader, SeekFrom};
use std::str;
use std::str::FromStr;
use std::vec::*;

use failure::Error;
// use failure::err_msg;

#[macro_use]
#[doc(hidden)]
mod macro_src;

pub mod float_reader;
pub mod int_reader;
#[cfg(not(feature = "mmap"))]
pub mod parser;
pub mod prim_reader;
pub mod uint_reader;
#[cfg(feature = "mmap")]
pub mod parser_mem;

pub use self::float_reader::*;
pub use self::int_reader::*;
pub use self::macro_src::*;
#[cfg(not(feature = "mmap"))]
pub use self::parser::parse_txt;
pub use self::prim_reader::*;
pub use self::uint_reader::*;
#[cfg(feature = "mmap")]
pub use self::parser_mem::parse_txt;

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
/// is_string - an optional field that tells us if the string passed is a string or file
pub struct ReaderParams {
    pub comments: Option<u8>,
    pub delimiter: Delimiter,
    pub skip_header: Option<usize>,
    pub skip_footer: Option<usize>,
    pub usecols: Option<Vec<usize>>,
    pub max_rows: Option<usize>,
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
            // is_string: None,
        }
    }
}

///A structure that contains all of the results. It tells us the number of fields we had
///along with the number of lines that we read. Finally, the results are stored in a single Vec of
///type T. Type T is what type one called load_txt_* for.
#[derive(Debug, Clone)]
pub struct ReaderResults<T: FromStr + Clone> {
    pub num_fields: usize,
    pub num_lines: usize,
    pub results: Vec<T>,
}

impl<T: FromStr + Clone> ReaderResults<T> {
    ///Obtains a value for a given row and column where both indices inputted
    ///are 0 based.
    pub fn get_value(&self, row_index: usize, col_index: usize) -> T {
        assert!(row_index < self.num_lines);
        assert!(col_index < self.num_fields);
        self.results[row_index * self.num_fields + col_index].clone()
    }
    ///Returns a row given a valid index that is 0 based and less than the
    ///number of lines read in.
    pub fn get_row(&self, row_index: usize) -> Vec<T> {
        assert!(row_index < self.num_lines);

        let out: Vec<T> = self
            .results
            .iter()
            .skip(row_index * self.num_fields)
            .take(self.num_fields)
            .cloned()
            .collect();

        out
    }
    ///Returns a series of rows given a valid indices that are 0 based and less than the
    ///number of lines read in.
    pub fn get_rows(&self, row_indices: Vec<usize>) -> Vec<Vec<T>> {
        let mut out: Vec<Vec<T>> = Vec::new();

        for index in row_indices.iter() {
            out.push(self.get_row(*index));
        }

        out
    }
    ///Returns a given column given a valid index that is 0 based and less than the
    ///number of fields read in.
    pub fn get_col(&self, col_index: usize) -> Vec<T> {
        assert!(col_index < self.num_fields);
        //We should just be able to use a slice to obtaine the values we want
        let out: Vec<T> = self
            .results
            .iter()
            .skip(col_index)
            .step_by(self.num_fields)
            .cloned()
            .collect();

        out
    }
    ///Returns a series of rows given a valid indices that are 0 based and less than the
    ///number of lines read in.
    pub fn get_cols(&self, col_indices: Vec<usize>) -> Vec<Vec<T>> {
        let mut out: Vec<Vec<T>> = Vec::new();

        for index in col_indices.iter() {
            out.push(self.get_col(*index));
        }

        out
    }
}

///A structure that contains all of the raw results. It tells us the number of fields we had
///along with the number of lines that we read. Results contains all of the data that was read in
///from the file in its raw u8 format. The index field contains the starting index for each field
///that was read in.
pub struct RawReaderResults {
    pub num_fields: usize,
    pub num_lines: usize,
    pub results: Vec<u8>,
    pub index: Vec<usize>,
}

///A private function that counts the number of lines that match a specified character specified to it.
///It is assumed that this character only appears once per line.
fn count_lines(buf: &[u8], eol: u8) -> usize {
    bytecount::count(buf, eol) as usize
}

/// It simply reads all of the lines in the file when an end of line is denoted by \n.
/// It does not take into account whether any line is a comment or not.
pub fn read_num_file_tot_lines<R: BufRead>(reader: &mut R) -> usize {
    let mut count = 0;
    loop {
        let buffer = reader.fill_buf().unwrap();
        let length = buffer.len();
        count += count_lines(&buffer[0..length], b'\n');
        //We now need to consume everything in our buffer, so it's marked off as no longer being needed
        reader.consume(length);
        if length < BUF_SIZE {
            break;
        }
    }

    count
}

///It simply reads all of the lines in the file when an end of line is denoted by \n or \r.
///A comment character is provided and if it is seen then before any nonwhite space the line is not counted in the total.
pub fn read_num_file_lines<R: BufRead>(reader: & mut R, com: u8) -> usize {
    let mut count = 0;
    //We're explicitly using the raw bytes here
    // let mut reader = BufReader::with_capacity(BUF_SIZE, f);
    //We loop over until the file has been completely read
    loop {
        //We first find the length of our buffer
        let length = {
            //We fill the buffer up. Our buffer is mutable which is why it's in this block
            let buffer = reader.fill_buf().unwrap();
            //We're now going to use an explicit loop.
            //I know this isn't idiomatic rust, but I couldn't really see a good way of skipping my iterator
            //to a location of my choosing.
            let mut i = 0;
            //We're using the memchr crate to locate all of the most common newline characters
            //It provides a nice iterator over our buffer that we can now use.
            let mut newline = memchr2_iter(b'\n', b'\r', buffer);
            //We don't want our loop index to go past our buffer length or else bad things could occur
            let length = buffer.len();
            //Keeping it old school with some nice wild loops
            while i < length {
                //Here's where the main magic occurs
                //If we come across a space or tab we move to the next item in the buffer
                //If we come across a newline character we advance our iterator and move onto the
                //next index essentially
                //If we come across a comment character first (white spaces aren't counted) we completely skip the line
                //If we come across any other character first (white spaces aren't counted) we increment our line counter
                //and then skip the rest of the contents of the line.
                //If we no longer have an item in our newline iterator we're done with everything in our buffer, and so
                //we can exit the loop.
                if (buffer[i] == b' ') | (buffer[i] == b'\t') {
                    i += 1;
                } else if (buffer[i] == b'\n') | (buffer[i] == b'\r') | (buffer[i] == com) {
                    let val = newline.next();
                    i = match val {
                        Some(val) => val + 1,
                        None => length,
                    };
                } else {
                    count += 1;
                    let val = newline.next();
                    i = match val {
                        Some(val) => val + 1,
                        None => length,
                    };
                }
            }
            //Pass off our length to set our length outside of this block of code
            length
        };
        //We now need to consume everything in our buffer, so it's marked off as no longer being needed
        reader.consume(length);
        //If our length is less than our fixed buffer size we've reached the end of our file and can now exit.
        if length < BUF_SIZE {
            break;
        }
    }
    //Finally, we return our line count to the main code.
    count
}

///It simply reads all of the lines in the file when an end of line is denoted by \n or \r.
///A comment character is provided and if it is seen then before any nonwhite space the line is not counted in the total.
#[cfg(feature = "mmap")]
pub fn read_num_file_lines_mmap(reader: &[u8], com: u8) -> usize {
    let mut count = 0;
    //We're explicitly using the raw bytes here
    //We're now going to use an explicit loop.
    //I know this isn't idiomatic rust, but I couldn't really see a good way of skipping my iterator
    //to a location of my choosing.
    let mut i = 0;
    //We're using the memchr crate to locate all of the most common newline characters
    //It provides a nice iterator over our buffer that we can now use.
    let mut newline = memchr2_iter(b'\n', b'\r', reader);
    //We don't want our loop index to go past our buffer length or else bad things could occur
    let length = reader.len();
    //Keeping it old school with some nice wild loops
    while i < length {
        //Here's where the main magic occurs
        //If we come across a space or tab we move to the next item in the buffer
        //If we come across a newline character we advance our iterator and move onto the
        //next index essentially
        //If we come across a comment character first (white spaces aren't counted) we completely skip the line
        //If we come across any other character first (white spaces aren't counted) we increment our line counter
        //and then skip the rest of the contents of the line.
        //If we no longer have an item in our newline iterator we're done with everything in our buffer, and so
        //we can exit the loop.
        if (reader[i] == b' ') | (reader[i] == b'\t') {
            i += 1;
        } else if (reader[i] == b'\n') | (reader[i] == b'\r') | (reader[i] == com) {
            let val = newline.next();
            i = match val {
                Some(val) => val + 1,
                None => length,
            };
        } else {
            count += 1;
            let val = newline.next();
            i = match val {
                Some(val) => val + 1,
                None => length,
            };
        }
    }
    //Finally, we return our line count to the main code.
    count
}
