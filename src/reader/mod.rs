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

use std::vec::*;
use std::str;
use std::str::{FromStr};
use std::fs::File;
use std::io::{Read, BufReader, BufRead, SeekFrom};
use bytecount;
use lexical;

use failure::Error;
// use failure::err_msg;

#[macro_use]
#[doc(hidden)]
pub mod macro_src;

pub mod int_reader;
pub mod uint_reader;
pub mod float_reader;
pub mod prim_reader;

#[doc(hidden)]
pub use self::macro_src::*;
pub use self::int_reader::*;
pub use self::uint_reader::*;
pub use self::float_reader::*;
pub use self::prim_reader::*;

const BUF_SIZE: usize = 8 * (1<<10);
///The type of delimiter that we can use
pub enum Delimiter{
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
///
/// max_rows - an optional field that tells us the maximum number of rows we should use from the file
pub struct ReaderParams{
    pub comments: u8,
    pub delimiter: Delimiter,
    pub skip_header: Option<usize>,
    pub skip_footer: Option<usize>,
    pub usecols: Option<Vec<usize>>,
    pub max_rows: Option<usize>,
}


impl Default for ReaderParams{
    fn default() -> ReaderParams {
        ReaderParams{
            comments: b'#',
            delimiter: Delimiter::WhiteSpace,
            skip_header: None,
            skip_footer: None,
            usecols: None,
            max_rows: None,
        }
    }
}

///A structure that contains all of the results. It tells us the number of fields we had
///along with the number of lines that we read. Finally, the results are stored in a single Vec of
///type T. Type T is what type one called load_txt_* for.
pub struct ReaderResults<T: FromStr>{
    pub num_fields: usize,
    pub num_lines: usize,
    pub results: Vec<T>,
}

///A private function that counts the number of lines that match a specified character specified to it.
///It is assummed that this character only appears once per line.
fn count_lines(buf: &[u8], eol: u8) -> usize {
    bytecount::count(buf, eol) as usize
}

///It simply reads all of the lines in the file when an end of line is denoted by \n. 
///It does not take into account whether any line is a comment or not.
pub fn read_num_file_tot_lines(f: &mut File) -> usize{
    let mut buffer = vec![0u8; BUF_SIZE];
    let mut count = 0;

    loop{
        let length = f.read(buffer.as_mut_slice()).unwrap();
        count += count_lines(&buffer[0..length], b'\n');
        if length < BUF_SIZE{
            break;
        }
    }

    count
}

///It simply reads all of the lines in the file when an end of line is denoted by \n. 
///A comment character is provided and if it is seen then it is not counted in the total.
///
/// Note it is assummed that the comment character only appears at the beginning of a line and nowhere else.
///If it does appear in more then one location this will currently provide the incorrect number of lines per
///the file. A more careful solution could be introduced which does not take advantage of this quick method.
// pub fn read_num_file_lines(f: &mut File, com: u8) -> usize{
//     let mut buffer = vec![0u8; BUF_SIZE];
//     let mut count = 0;

//     loop{
//         let length = f.read(buffer.as_mut_slice()).unwrap();
//         count += count_lines(&buffer[0..length], b'\n');
//         count -= count_lines(&buffer[0..length], com);
//         if length < BUF_SIZE{
//             break;
//         }
//     }

//     count
// }

pub fn read_num_file_lines(f: &File, com: u8) -> usize{
    
    let tmp = [com.clone()];
    let comment = str::from_utf8(&tmp).unwrap();
    let mut count = 0;

    //We are now creating a reader buffer to easily read through our file
    let mut reader = BufReader::new(f);
    //An allocated string to read in the buffer file.
    let mut line = String::new();

    //Very way to count the total number of useable lines.
    while reader.read_line(&mut line).unwrap() > 0{
        //Here we're checking to see if we've run across a blank line or
        //a commented line.
        if (!line.trim_left().starts_with(&comment)) && (!line.trim_left().is_empty()){
            count += 1;
        }
        //clear our buffer
        line.clear();
    }

    count

}
