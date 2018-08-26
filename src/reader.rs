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
use std::io::{Read, BufRead, BufReader, Seek, SeekFrom};
use bytecount;

use failure::Error;
use failure::err_msg;

const BUF_SIZE: usize = 8 * (1<<10);
///The type of delimiter that we can use
pub enum Delimiter{
    WhiteSpace,
    Any(u8),
}

///The different data types that we support the ability to parse from a file. 
pub enum DataType{
    Float32,
    Float64,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Uintsize,
    Boolean,
    Strings,
}

///ReaderParams tells us what our reader should be doing.
///dtype - the data type of our outputted vector. We currently only support one type.
///delimiter - the delimiter that tells us what our data fields are seperated by
///skip_header - an optional field that tells us whether or not we should skip so many lines that are not
///     comment lines from the beginning of the file
///skip_footer - an optional field that tells us whether or not we should skip so many lines that are not
///     comment lines from the end of the file
///usecols - an optional field that tells us what column numbers we should be using from the data field
///max_rows - an optional field that tells us the maximum number of rows we should use from the file
pub struct ReaderParams{
    pub dtype: String,
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
            dtype: String::from("String"),
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
///type T. Type T is dependent on what was supplied to ReaderParams for dtype.
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
///Note it is assummed that the comment character only appears at the beginning of a line and nowhere else.
///If it does appear in more then one location this will currently provide the incorrect number of lines per
///the file. A more careful solution could be introduced which does not take advantage of this quick method.
pub fn read_num_file_lines(f: &mut File, com: u8) -> usize{
    let mut buffer = vec![0u8; BUF_SIZE];
    let mut count = 0;

    loop{
        let length = f.read(buffer.as_mut_slice()).unwrap();
        count += count_lines(&buffer[0..length], b'\n');
        count -= count_lines(&buffer[0..length], com);
        if length < BUF_SIZE{
            break;
        }
    }

    count
}

///Temporary solution but once this has been written we should be able to create a macro that generates all of this for us...
///A note needs to be added that this needs to better commented at this point.
pub fn load_txt_i32(f: &str, params: &ReaderParams) -> Result<ReaderResults<i32>, Error>{

    let mut file = File::open(f)?;

    let num_lines = read_num_file_lines(&mut file, params.comments);

    file.seek(SeekFrom::Start(0))?;

    let mut reader = BufReader::new(file);

    let mut line = String::new();

    let mut results = ReaderResults{
        num_fields: 0,
        num_lines: 0,
        results: Vec::<i32>::new(),
    };

    match &params.skip_header{
        Some(x) => {
            if *x >= num_lines{
                return Err(format_err!("Input for skip_header greater than the number of readable lines in the file"));
            }
        }
        None => (),
    }

    let sk_h = if let Some(x) = params.skip_header{
        x
    }else{
        0
    };

    match &params.skip_footer{
        Some(x) => {
            if *x >= num_lines {
                return Err(format_err!("Input for skip_footer greater than the number of readable lines in the file"));
            }
        }
        None => (),
    }

    let sk_f = if let Some(x) = params.skip_footer{
        x
    }else{
        0
    };

    if num_lines <= (sk_h + sk_f) {
        return Err(format_err!("Input for skip_footer and skip_header greater than or equal to the number of readable lines in the file"));
    }

    let num_lines_read = match &params.max_rows{
        Some(x) => {
                let diff_lines = num_lines - sk_h - sk_f;
                if diff_lines > *x {
                    *x
                }else{
                    diff_lines
                }
            }
        None => (num_lines - sk_h - sk_f)
    };
    let tmp = [params.comments.clone()];
    let comment = str::from_utf8(&tmp).unwrap();
    
    //File line number used for Error information
    let mut fln = 0;
    if sk_h > 0{
        let mut n_line_skip = 0;
        while reader.read_line(&mut line).unwrap() > 0{
            fln += 1;
            if !line.starts_with(&comment){
                n_line_skip += 1; 
            }
            if n_line_skip == sk_h {
                line.clear();
                break;
            }
            //clear our buffer
            line.clear();
        }
    }

    //Loop through the rest of the file until we either reach the end or the maximum number of lines that we want.
    while reader.read_line(&mut line).unwrap() > 0{
        fln += 1;
        if !line.starts_with(&comment){
            //I really don't like that I have to clone this...
            let tmp_line = line.clone();
            //clear our buffer

            results.num_lines += 1;
            //I also am not happy about having to create this each time it gets a new line.
            //Later versions will have to go about it by streaming the bytes and just dealing with the
            //ascii/utf-8 data directly like rust_csv.
            let line_split_vec: Vec<&str> = match &params.delimiter{
                Delimiter::WhiteSpace => {
                    tmp_line.split_whitespace().collect()
                }
                Delimiter::Any(b) => {   
                    tmp_line.split(str::from_utf8(&[*b]).unwrap()).collect()
                }
            };
            if results.num_lines == 1 {
                results.num_fields = line_split_vec.len();
            }

            if line_split_vec.len() != results.num_fields {
                println!("Contents of line_split_vec {:?}", line_split_vec);
                return Err(format_err!("Number of fields provided at line {} is different than the initial field number of {}", fln, results.num_fields));
            }

            match &params.usecols{
                Some(x) =>{
                    results.results.extend({
                            x.iter().map(|y| i32::from_str(line_split_vec[*y].trim()).unwrap())
                        });
                }
                None =>{
                    results.results.extend({
                        line_split_vec.iter().map(|x| i32::from_str(x.trim()).unwrap())
                    });
                }
            }
        }
        if results.num_lines == num_lines_read {
            break;
        }

        line.clear();
    }

    Ok(results)
}

