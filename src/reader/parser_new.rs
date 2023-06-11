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
use super::*;
use std::io::{BufRead, Seek};
#[cfg(feature = "mmap")]
use std::io::{Cursor};
#[cfg(feature = "mmap")]
use memmap::MmapOptions;

use super::parser_utility::{NwLine, ParserState, CoreData};


///parse_txt reads in a data file that is made up any type(s). It parses the data file finding all of the field data and saving off in its raw
///byte form. It can fail in a number of other ways related to invalid parameters or the data file having malformed fields. These errors are
///percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///
///Input -
///
/// f is simply the location of the file.
///
/// params is ReaderParams structure. An example for what this looks like can be found in the test directory.
///
///Output -
///
/// A Result type that either contains a RawReaderResults structure or an error.
pub fn parse_txt(f: &str, params: &ReaderParams) -> Result<RawReaderResults, Error> {

    let file = File::open(f)?;

    //our comment string
    //If we don't have one then we just say a comment is a newline character.
    //The newline check comes first in all of these so it'll be as if the parser never
    //has to worry about the comments.
    let cmt = if let Some(x) = params.comments {
        x
    } else {
        b'\n'
    };

    #[cfg(feature = "mmap")]
    let buffer = unsafe { MmapOptions::new().map(&file)? };

    //We're explicitly using the raw bytes here
    #[cfg(not(feature = "mmap"))]
    let mut reader = BufReader::with_capacity(BUF_SIZE, file);
    #[cfg(feature = "mmap")]
    let mut reader = Cursor::new(&buffer[..]);

    //We are finding how many lines in our data file are actually readable and are not commented lines.
    let num_lines = read_num_file_lines(& mut reader, cmt);
    //We need to rewind our file back to the start.
    reader.seek(SeekFrom::Start(0))?;

    //We are initializing our ReaderResult structure
    let mut results = RawReaderResults {
        num_fields: 0,
        num_lines: 0,
        results: Vec::<u8>::new(),
        index: Vec::<usize>::new(),
    };

    //The next portion of lines is some minor error handling to make sure our parameters we provided were valid for our data file.
    //We're checking to see if we have a valid number of skipped lines for the header.
    match &params.skip_header {
        Some(x) => {
            if *x >= num_lines {
                return Err(format_err!(
                    "Input for skip_header greater than the number of readable lines in the file"
                ));
            }
        }
        None => (),
    }

    //Now that we know our number is valid we are setting a variable for our skipped header lines to be equal to our skippable lines.
    let sk_h = if let Some(x) = params.skip_header {
        x
    } else {
        0
    };

    //We're checking to see if we have a valid number of skipped lines for the footer.
    match &params.skip_footer {
        Some(x) => {
            if *x >= num_lines {
                return Err(format_err!(
                    "Input for skip_footer greater than the number of readable lines in the file"
                ));
            }
        }
        None => (),
    }

    //Now that we know our number is valid we are setting a variable for our skipped footer lines to be equal to our skippable lines.
    let sk_f = if let Some(x) = params.skip_footer {
        x
    } else {
        0
    };
    //We need to error if the number of lines we can read is equal to or less than the number of skipped header and footer lines.
    if num_lines <= (sk_h + sk_f) {
        return Err(format_err!("Input for skip_footer and skip_header greater than or equal to the number of readable lines in the file"));
    }
    //Here we're determining the number of lines that we need to read from our file.
    let num_lines_read = match &params.max_rows {
        //If we've set a maximum number of rows. We need to first see if that number is greater than
        //the number of readable none skipped lines.
        //If we didn't then the maximum number is set to our number of readable non skipped lines.
        Some(x) => {
            let diff_lines = num_lines - sk_h - sk_f;
            if diff_lines > *x {
                *x
            } else {
                diff_lines
            }
        }
        None => num_lines - sk_h - sk_f,
    };

    //We're simply stating whether we're using whitespaces or not for our delimiter.
    let delim_ws = match &params.delimiter {
        Delimiter::WhiteSpace => true,
        Delimiter::Any(_b) => false,
    };
    //Our delimeter value. If we are delimiting using whitespace we set this as a space. However, we'll take into consideration tabs as well.
    let delim = match &params.delimiter {
        Delimiter::WhiteSpace => b' ',
        Delimiter::Any(b) => *b,
    };

    //File line number used for Error information
    let mut fln = 0;
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
                        fln += 1;
                    } else {
                        count += 1;
                        let val = newline.next();
                        i = match val {
                            Some(val) => val + 1,
                            None => length + 1,
                        };
                        fln += 1;
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
            if count == sk_h {
                break;
            }
        }
    }

    //Next we need to get a list of our columns we might be using. If we aren't we supply an empty vector, so we can easily check if the len is 0.
    //While these values are 0 indexed externally, internally it's a bit easier to deal with 1-based indexing for the time being.
    let mut cols = match &params.usecols {
        Some(x) => x.iter().map(|&x| x + 1).collect::<Vec<usize>>(),
        None => Vec::<usize>::new(),
    };

    //We need to count our field variables and set this variable initially outside the main loop.
    let _field_counter = 0;
    //We'll need to now the total number of fields later on and set this variable initially outside the main loop.
    let _tot_fields = 0;
    //The loop here is where all of the magic happens. It's designed so that it operates based on a state. So, we're essentially running a poorly optimized
    //state machine. However, it turns out that this works decent enough for our purposes as long as the optimizer is used.
    
    let mut core_data = CoreData{
        length: 0,
        offset: 0,
        cmt: cmt,
        delim_ws: delim_ws,
        delim: delim,
        fln: fln,
        cols: &mut cols,
        field_counter: 0,
        tot_fields: 0,
        results: &mut results,
    };

    let mut state = ParserState::NwLine(NwLine{});

    loop {
        //We first find the length of our buffer
        let length = {
            //We fill the buffer up. Our buffer is mutable which is why it's in this block
            let buffer = reader.fill_buf().unwrap();
            //We're now going to use an explicit loop.
            //I know this isn't idiomatic rust, but I couldn't really see a good way of skipping my iterator
            //to a location of my choosing.
            core_data.offset = 0;
            //We're using the memchr crate to locate all of the most common newline characters
            //It provides a nice iterator over our buffer that we can now use.
            let mut newline = memchr2_iter(b'\n', b'\r', buffer);

            //We don't want our loop index to go past our buffer length or else bad things could occur
            core_data.length = buffer.len();
            //Keeping it old school with some nice wild loops
            while core_data.offset < core_data.length {
                state = state.next(buffer, &mut newline, &mut core_data)?;
                //Check to see if we've read enough lines in if so break out of the loop
                if core_data.results.num_lines == num_lines_read {
                    break;
                }
            }
            core_data.length
        };
        //We now need to consume everything in our buffer, so it's marked off as no longer being needed
        reader.consume(length);
        //If our length is less than our fixed buffer size we've reached the end of our file and can now exit.
        if (length < BUF_SIZE) | (core_data.results.num_lines == num_lines_read) {
            break;
        }
    }
    //Assumming everything went well we save off our results.
    Ok(results)
}
