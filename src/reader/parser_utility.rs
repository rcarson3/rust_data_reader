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

use memchr::memchr2_iter;
use std::io::{BufRead};

use super::*;

///A private function that counts the number of lines that match a specified character specified to it.
///It is assumed that this character only appears once per line.
fn count_lines(buf: &[u8], eol: u8) -> usize {
    bytecount::count(buf, eol)
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

pub fn skip_header_lines<R: BufRead>(reader: & mut R, fln: &mut usize, cmt: u8, sk_h: usize) {
    //If we skip any header lines then we need to skip forward through the file by
    //the correct number of lines when not taking into account commented lines.
    if sk_h > 0 {
        let mut count = 0;

        //We loop over until we've skipped over the desired number of lines
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
                    } else if (buffer[i] == b'\n') | (buffer[i] == b'\r') | (buffer[i] == cmt) {
                        let val = newline.next();
                        i = match val {
                            Some(val) => val + 1,
                            None => length + 1,
                        };
                        *fln += 1;
                    } else {
                        count += 1;
                        let val = newline.next();
                        i = match val {
                            Some(val) => val + 1,
                            None => length + 1,
                        };
                        *fln += 1;
                    }
                    //Here we're checking to see if we've reached the number of lines to skip or not
                    if count == sk_h {
                        break;
                    }
                }
                //Pass off our length to set our length outside of this block of code
                i - 1
            };
            //We now need to consume everything upto "length" in our buffer, so it's marked off as no longer being needed
            reader.consume(length);
            //If we've skipped over the desired number of lines we can exit the loop.
            if count == sk_h || length < BUF_SIZE {
                break;
            }
        }
    }
}

pub fn count_num_fields<R: BufRead>(reader:&mut R, cmt: u8, delim: u8, delim_ws: bool) -> usize {
    let mut field_counter = 0;

    enum ParseState {CmtNwLine, Field, Space, Delim}

    let mut state = ParseState::CmtNwLine;

    //We loop over until we've skipped over the desired number of lines
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
                if (buffer[i] == delim) & !delim_ws {
                    state = match state {
                        ParseState::CmtNwLine => {
                            field_counter = 1;
                            ParseState::Delim
                        }
                        ParseState::Delim => ParseState::Delim,
                        ParseState::Field => {
                            field_counter += 1;
                            ParseState::Delim
                        }
                        ParseState::Space => {
                            field_counter += 1;
                            ParseState::Delim
                        }
                    };
                    i += 1;
                }
                else if (buffer[i] == b' ') | (buffer[i] == b'\t') {
                    if delim_ws {
                        state = match state {
                            ParseState::CmtNwLine => {
                                field_counter = 1;
                                ParseState::Delim
                            }
                            ParseState::Delim => ParseState::Delim,
                            ParseState::Field => {
                                field_counter += 1;
                                ParseState::Delim
                            }
                            ParseState::Space => {
                                field_counter += 1;
                                ParseState::Delim
                            }
                        };
                    } else {
                        state = match state {
                            ParseState::CmtNwLine => ParseState::Space,
                            ParseState::Delim => ParseState::Space,
                            ParseState::Field => ParseState::Field,
                            ParseState::Space => ParseState::Space,
                        };
                    }
                    i += 1;
                } 
                else if (buffer[i] == b'\n') | (buffer[i] == b'\r') | (buffer[i] == cmt) {
                    if field_counter == 0 {
                        let val = newline.next();
                        i = match val {
                            Some(val) => val + 1,
                            None => length + 1,
                        };
                    }
                    else {
                        if let ParseState::Delim = state { field_counter -= 1 };
                        return field_counter;
                    }
                }
                else {
                    state = match state {
                        ParseState::CmtNwLine => {
                            field_counter = 1;
                            ParseState::Field
                        }
                        ParseState::Delim => ParseState::Field,
                        ParseState::Field => ParseState::Field,
                        ParseState::Space => {
                            if field_counter == 0 {
                                field_counter += 1;
                            }
                            ParseState::Field
                        }
                    };
                    i += 1;
                }
            }
            //Pass off our length to set our length outside of this block of code
            i - 1
        };
        //We now need to consume everything upto "length" in our buffer, so it's marked off as no longer being needed
        reader.consume(length);
        //If we've skipped over the desired number of lines we can exit the loop.
        if length < BUF_SIZE {
            break;
        }
    }

    field_counter
}