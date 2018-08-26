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
use super::*;
use std::io::{BufRead, Seek};

///load_txt_i8 reads in a data file that is made up of i8 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to i8. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///Input - f is simply the location of the file.
///        params is ReaderParams structure. An example for what this looks like can be found in the test directory.
///Output - A Result type that either contains a ReaderResults structure or an error. 
///Temporary solution but once this has been written we should be able to create a macro that generates all of this for us...
///A note needs to be added that this needs to better commented at this point.
pub fn load_txt_i8(f: &str, params: &ReaderParams) -> Result<ReaderResults<i8>, Error>{

    let mut file = File::open(f)?;

    //We are finding how many lines in our data file are actually readable and are not commented lines.
    let num_lines = read_num_file_lines(&mut file, params.comments);
    //We need to rewind our file back to the start.
    file.seek(SeekFrom::Start(0))?;
    //We are now creating a reader buffer to easily read through our file
    let mut reader = BufReader::new(file);
    //An allocated string to read in the buffer file.
    let mut line = String::new();

    //We are initializing our ReaderResult structure
    let mut results = ReaderResults{
        num_fields: 0,
        num_lines: 0,
        results: Vec::<i8>::new(),
    };

    //The next portion of lines is some minor error handling to make sure our parameters we provided were valid for our data file.
    //We're checking to see if we have a valid number of skipped lines for the header.
    match &params.skip_header{
        Some(x) => {
            if *x >= num_lines{
                return Err(format_err!("Input for skip_header greater than the number of readable lines in the file"));
            }
        }
        None => (),
    }

    //Now that we know our number is valid we are setting a variable for our skipped header lines to be equal to our skippable lines.
    let sk_h = if let Some(x) = params.skip_header{
        x
    }else{
        0
    };

    //We're checking to see if we have a valid number of skipped lines for the footer.
    match &params.skip_footer{
        Some(x) => {
            if *x >= num_lines {
                return Err(format_err!("Input for skip_footer greater than the number of readable lines in the file"));
            }
        }
        None => (),
    }

    //Now that we know our number is valid we are setting a variable for our skipped footer lines to be equal to our skippable lines.
    let sk_f = if let Some(x) = params.skip_footer{
        x
    }else{
        0
    };
    //We need to error if the number of lines we can read is equal to or less than the number of skipped header and footer lines. 
    if num_lines <= (sk_h + sk_f) {
        return Err(format_err!("Input for skip_footer and skip_header greater than or equal to the number of readable lines in the file"));
    }
    //Here we're determining the number of lines that we need to read from our file.
    let num_lines_read = match &params.max_rows{
        //If we've set a maximum number of rows. We need to first see if that number is greater than
        //the number of readable none skipped lines.
        //If we didn't then the maximum number is set to our number of readable non skipped lines.
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
    //temporary variable to create our comment string
    let tmp = [params.comments.clone()];
    //convert over from binary to a str type for our comment
    let comment = str::from_utf8(&tmp).unwrap();
    
    //File line number used for Error information
    let mut fln = 0;
    //If we skip any header lines then we need to skip forward through the file by
    //the correct number of lines when not taking into account commented lines.
    if sk_h > 0{
        let mut n_line_skip = 0;
        while reader.read_line(&mut line).unwrap() > 0{
            fln += 1;
            //Checking to see if the line is a comment or not
            //if it is increment our counter
            if !line.starts_with(&comment){
                n_line_skip += 1; 
            }
            //If we've reached the number of lines to skip
            //if so we first clear our string and then break
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
            //It also forces the issue of having to potentially reallocate a string each time.
            //If it wasn't for the line split issue down below this probably wouldn't be an issue at all.
            //All of this should hopefully be cleaned up once I get a proper parser that just takes advantage of
            //streaming bytes instead of using strings.
            let tmp_line = line.clone();
            //Increment the number of lines that we've read
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
            //If this is the first valid field that we're reading then we will use its length to determine
            //what is a valid length for all of the other lines.
            if results.num_lines == 1 {
                results.num_fields = line_split_vec.len();
            }
            //If the line we parsed does not have the same number of fields as our initial value then it is an error.
            if line_split_vec.len() != results.num_fields {
                println!("Contents of line_split_vec {:?}", line_split_vec);
                return Err(format_err!("Number of fields provided at line {} is different than the initial field number of {}", fln, results.num_fields));
            }

            //Here we need to parse only the data that we want from string to the designated data format.
            //It should be noted that Rust's parser from string to data will fail if the string provided is:
            //outside the minimum or maximum range of the data type. The data type for example is an int/uint but
            //the string shows a float. The data type for example is an int/uint but the string is written in scientific
            //notation.
            //I would like to find some way to create better way to either percolate these errors up or provide a better failure statement.
            match &params.usecols{
                Some(x) =>{
                    results.results.extend({
                            x.iter().map(|y| i8::from_str(line_split_vec[*y].trim()).unwrap())
                        });
                }
                None =>{
                    results.results.extend({
                        line_split_vec.iter().map(|x| i8::from_str(x.trim()).unwrap())
                    });
                }
            }
        }
        //Break out of the loop early if we've read all of the lines that we need to read.
        if results.num_lines == num_lines_read {
            break;
        }
        //Here we clear out the contents of our string vector.
        line.clear();
    }
    //We've finished parsing the file successfully. It is now time to pass the results on.
    Ok(results)
}

///load_txt_i16 reads in a data file that is made up of i16 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to i16. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///Input - f is simply the location of the file.
///        params is ReaderParams structure. An example for what this looks like can be found in the test directory.
///Output - A Result type that either contains a ReaderResults structure or an error. 
///Temporary solution but once this has been written we should be able to create a macro that generates all of this for us...
///A note needs to be added that this needs to better commented at this point.
pub fn load_txt_i16(f: &str, params: &ReaderParams) -> Result<ReaderResults<i16>, Error>{

    let mut file = File::open(f)?;

    //We are finding how many lines in our data file are actually readable and are not commented lines.
    let num_lines = read_num_file_lines(&mut file, params.comments);
    //We need to rewind our file back to the start.
    file.seek(SeekFrom::Start(0))?;
    //We are now creating a reader buffer to easily read through our file
    let mut reader = BufReader::new(file);
    //An allocated string to read in the buffer file.
    let mut line = String::new();

    //We are initializing our ReaderResult structure
    let mut results = ReaderResults{
        num_fields: 0,
        num_lines: 0,
        results: Vec::<i16>::new(),
    };

    //The next portion of lines is some minor error handling to make sure our parameters we provided were valid for our data file.
    //We're checking to see if we have a valid number of skipped lines for the header.
    match &params.skip_header{
        Some(x) => {
            if *x >= num_lines{
                return Err(format_err!("Input for skip_header greater than the number of readable lines in the file"));
            }
        }
        None => (),
    }

    //Now that we know our number is valid we are setting a variable for our skipped header lines to be equal to our skippable lines.
    let sk_h = if let Some(x) = params.skip_header{
        x
    }else{
        0
    };

    //We're checking to see if we have a valid number of skipped lines for the footer.
    match &params.skip_footer{
        Some(x) => {
            if *x >= num_lines {
                return Err(format_err!("Input for skip_footer greater than the number of readable lines in the file"));
            }
        }
        None => (),
    }

    //Now that we know our number is valid we are setting a variable for our skipped footer lines to be equal to our skippable lines.
    let sk_f = if let Some(x) = params.skip_footer{
        x
    }else{
        0
    };
    //We need to error if the number of lines we can read is equal to or less than the number of skipped header and footer lines. 
    if num_lines <= (sk_h + sk_f) {
        return Err(format_err!("Input for skip_footer and skip_header greater than or equal to the number of readable lines in the file"));
    }
    //Here we're determining the number of lines that we need to read from our file.
    let num_lines_read = match &params.max_rows{
        //If we've set a maximum number of rows. We need to first see if that number is greater than
        //the number of readable none skipped lines.
        //If we didn't then the maximum number is set to our number of readable non skipped lines.
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
    //temporary variable to create our comment string
    let tmp = [params.comments.clone()];
    //convert over from binary to a str type for our comment
    let comment = str::from_utf8(&tmp).unwrap();
    
    //File line number used for Error information
    let mut fln = 0;
    //If we skip any header lines then we need to skip forward through the file by
    //the correct number of lines when not taking into account commented lines.
    if sk_h > 0{
        let mut n_line_skip = 0;
        while reader.read_line(&mut line).unwrap() > 0{
            fln += 1;
            //Checking to see if the line is a comment or not
            //if it is increment our counter
            if !line.starts_with(&comment){
                n_line_skip += 1; 
            }
            //If we've reached the number of lines to skip
            //if so we first clear our string and then break
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
            //It also forces the issue of having to potentially reallocate a string each time.
            //If it wasn't for the line split issue down below this probably wouldn't be an issue at all.
            //All of this should hopefully be cleaned up once I get a proper parser that just takes advantage of
            //streaming bytes instead of using strings.
            let tmp_line = line.clone();
            //Increment the number of lines that we've read
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
            //If this is the first valid field that we're reading then we will use its length to determine
            //what is a valid length for all of the other lines.
            if results.num_lines == 1 {
                results.num_fields = line_split_vec.len();
            }
            //If the line we parsed does not have the same number of fields as our initial value then it is an error.
            if line_split_vec.len() != results.num_fields {
                println!("Contents of line_split_vec {:?}", line_split_vec);
                return Err(format_err!("Number of fields provided at line {} is different than the initial field number of {}", fln, results.num_fields));
            }

            //Here we need to parse only the data that we want from string to the designated data format.
            //It should be noted that Rust's parser from string to data will fail if the string provided is:
            //outside the minimum or maximum range of the data type. The data type for example is an int/uint but
            //the string shows a float. The data type for example is an int/uint but the string is written in scientific
            //notation.
            //I would like to find some way to create better way to either percolate these errors up or provide a better failure statement.
            match &params.usecols{
                Some(x) =>{
                    results.results.extend({
                            x.iter().map(|y| i16::from_str(line_split_vec[*y].trim()).unwrap())
                        });
                }
                None =>{
                    results.results.extend({
                        line_split_vec.iter().map(|x| i16::from_str(x.trim()).unwrap())
                    });
                }
            }
        }
        //Break out of the loop early if we've read all of the lines that we need to read.
        if results.num_lines == num_lines_read {
            break;
        }
        //Here we clear out the contents of our string vector.
        line.clear();
    }
    //We've finished parsing the file successfully. It is now time to pass the results on.
    Ok(results)
}


///load_txt_i32 reads in a data file that is made up of i32 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to i32. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///Input - f is simply the location of the file.
///        params is ReaderParams structure. An example for what this looks like can be found in the test directory.
///Output - A Result type that either contains a ReaderResults structure or an error. 
///Temporary solution but once this has been written we should be able to create a macro that generates all of this for us...
///A note needs to be added that this needs to better commented at this point.
pub fn load_txt_i32(f: &str, params: &ReaderParams) -> Result<ReaderResults<i32>, Error>{

    let mut file = File::open(f)?;

    //We are finding how many lines in our data file are actually readable and are not commented lines.
    let num_lines = read_num_file_lines(&mut file, params.comments);
    //We need to rewind our file back to the start.
    file.seek(SeekFrom::Start(0))?;
    //We are now creating a reader buffer to easily read through our file
    let mut reader = BufReader::new(file);
    //An allocated string to read in the buffer file.
    let mut line = String::new();

    //We are initializing our ReaderResult structure
    let mut results = ReaderResults{
        num_fields: 0,
        num_lines: 0,
        results: Vec::<i32>::new(),
    };

    //The next portion of lines is some minor error handling to make sure our parameters we provided were valid for our data file.
    //We're checking to see if we have a valid number of skipped lines for the header.
    match &params.skip_header{
        Some(x) => {
            if *x >= num_lines{
                return Err(format_err!("Input for skip_header greater than the number of readable lines in the file"));
            }
        }
        None => (),
    }

    //Now that we know our number is valid we are setting a variable for our skipped header lines to be equal to our skippable lines.
    let sk_h = if let Some(x) = params.skip_header{
        x
    }else{
        0
    };

    //We're checking to see if we have a valid number of skipped lines for the footer.
    match &params.skip_footer{
        Some(x) => {
            if *x >= num_lines {
                return Err(format_err!("Input for skip_footer greater than the number of readable lines in the file"));
            }
        }
        None => (),
    }

    //Now that we know our number is valid we are setting a variable for our skipped footer lines to be equal to our skippable lines.
    let sk_f = if let Some(x) = params.skip_footer{
        x
    }else{
        0
    };
    //We need to error if the number of lines we can read is equal to or less than the number of skipped header and footer lines. 
    if num_lines <= (sk_h + sk_f) {
        return Err(format_err!("Input for skip_footer and skip_header greater than or equal to the number of readable lines in the file"));
    }
    //Here we're determining the number of lines that we need to read from our file.
    let num_lines_read = match &params.max_rows{
        //If we've set a maximum number of rows. We need to first see if that number is greater than
        //the number of readable none skipped lines.
        //If we didn't then the maximum number is set to our number of readable non skipped lines.
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
    //temporary variable to create our comment string
    let tmp = [params.comments.clone()];
    //convert over from binary to a str type for our comment
    let comment = str::from_utf8(&tmp).unwrap();
    
    //File line number used for Error information
    let mut fln = 0;
    //If we skip any header lines then we need to skip forward through the file by
    //the correct number of lines when not taking into account commented lines.
    if sk_h > 0{
        let mut n_line_skip = 0;
        while reader.read_line(&mut line).unwrap() > 0{
            fln += 1;
            //Checking to see if the line is a comment or not
            //if it is increment our counter
            if !line.starts_with(&comment){
                n_line_skip += 1; 
            }
            //If we've reached the number of lines to skip
            //if so we first clear our string and then break
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
            //It also forces the issue of having to potentially reallocate a string each time.
            //If it wasn't for the line split issue down below this probably wouldn't be an issue at all.
            //All of this should hopefully be cleaned up once I get a proper parser that just takes advantage of
            //streaming bytes instead of using strings.
            let tmp_line = line.clone();
            //Increment the number of lines that we've read
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
            //If this is the first valid field that we're reading then we will use its length to determine
            //what is a valid length for all of the other lines.
            if results.num_lines == 1 {
                results.num_fields = line_split_vec.len();
            }
            //If the line we parsed does not have the same number of fields as our initial value then it is an error.
            if line_split_vec.len() != results.num_fields {
                println!("Contents of line_split_vec {:?}", line_split_vec);
                return Err(format_err!("Number of fields provided at line {} is different than the initial field number of {}", fln, results.num_fields));
            }

            //Here we need to parse only the data that we want from string to the designated data format.
            //It should be noted that Rust's parser from string to data will fail if the string provided is:
            //outside the minimum or maximum range of the data type. The data type for example is an int/uint but
            //the string shows a float. The data type for example is an int/uint but the string is written in scientific
            //notation.
            //I would like to find some way to create better way to either percolate these errors up or provide a better failure statement.
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
        //Break out of the loop early if we've read all of the lines that we need to read.
        if results.num_lines == num_lines_read {
            break;
        }
        //Here we clear out the contents of our string vector.
        line.clear();
    }
    //We've finished parsing the file successfully. It is now time to pass the results on.
    Ok(results)
}

///load_txt_i64 reads in a data file that is made up of i64 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to i64. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///Input - f is simply the location of the file.
///        params is ReaderParams structure. An example for what this looks like can be found in the test directory.
///Output - A Result type that either contains a ReaderResults structure or an error. 
///Temporary solution but once this has been written we should be able to create a macro that generates all of this for us...
///A note needs to be added that this needs to better commented at this point.
pub fn load_txt_i64(f: &str, params: &ReaderParams) -> Result<ReaderResults<i64>, Error>{

    let mut file = File::open(f)?;

    //We are finding how many lines in our data file are actually readable and are not commented lines.
    let num_lines = read_num_file_lines(&mut file, params.comments);
    //We need to rewind our file back to the start.
    file.seek(SeekFrom::Start(0))?;
    //We are now creating a reader buffer to easily read through our file
    let mut reader = BufReader::new(file);
    //An allocated string to read in the buffer file.
    let mut line = String::new();

    //We are initializing our ReaderResult structure
    let mut results = ReaderResults{
        num_fields: 0,
        num_lines: 0,
        results: Vec::<i64>::new(),
    };

    //The next portion of lines is some minor error handling to make sure our parameters we provided were valid for our data file.
    //We're checking to see if we have a valid number of skipped lines for the header.
    match &params.skip_header{
        Some(x) => {
            if *x >= num_lines{
                return Err(format_err!("Input for skip_header greater than the number of readable lines in the file"));
            }
        }
        None => (),
    }

    //Now that we know our number is valid we are setting a variable for our skipped header lines to be equal to our skippable lines.
    let sk_h = if let Some(x) = params.skip_header{
        x
    }else{
        0
    };

    //We're checking to see if we have a valid number of skipped lines for the footer.
    match &params.skip_footer{
        Some(x) => {
            if *x >= num_lines {
                return Err(format_err!("Input for skip_footer greater than the number of readable lines in the file"));
            }
        }
        None => (),
    }

    //Now that we know our number is valid we are setting a variable for our skipped footer lines to be equal to our skippable lines.
    let sk_f = if let Some(x) = params.skip_footer{
        x
    }else{
        0
    };
    //We need to error if the number of lines we can read is equal to or less than the number of skipped header and footer lines. 
    if num_lines <= (sk_h + sk_f) {
        return Err(format_err!("Input for skip_footer and skip_header greater than or equal to the number of readable lines in the file"));
    }
    //Here we're determining the number of lines that we need to read from our file.
    let num_lines_read = match &params.max_rows{
        //If we've set a maximum number of rows. We need to first see if that number is greater than
        //the number of readable none skipped lines.
        //If we didn't then the maximum number is set to our number of readable non skipped lines.
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
    //temporary variable to create our comment string
    let tmp = [params.comments.clone()];
    //convert over from binary to a str type for our comment
    let comment = str::from_utf8(&tmp).unwrap();
    
    //File line number used for Error information
    let mut fln = 0;
    //If we skip any header lines then we need to skip forward through the file by
    //the correct number of lines when not taking into account commented lines.
    if sk_h > 0{
        let mut n_line_skip = 0;
        while reader.read_line(&mut line).unwrap() > 0{
            fln += 1;
            //Checking to see if the line is a comment or not
            //if it is increment our counter
            if !line.starts_with(&comment){
                n_line_skip += 1; 
            }
            //If we've reached the number of lines to skip
            //if so we first clear our string and then break
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
            //It also forces the issue of having to potentially reallocate a string each time.
            //If it wasn't for the line split issue down below this probably wouldn't be an issue at all.
            //All of this should hopefully be cleaned up once I get a proper parser that just takes advantage of
            //streaming bytes instead of using strings.
            let tmp_line = line.clone();
            //Increment the number of lines that we've read
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
            //If this is the first valid field that we're reading then we will use its length to determine
            //what is a valid length for all of the other lines.
            if results.num_lines == 1 {
                results.num_fields = line_split_vec.len();
            }
            //If the line we parsed does not have the same number of fields as our initial value then it is an error.
            if line_split_vec.len() != results.num_fields {
                println!("Contents of line_split_vec {:?}", line_split_vec);
                return Err(format_err!("Number of fields provided at line {} is different than the initial field number of {}", fln, results.num_fields));
            }

            //Here we need to parse only the data that we want from string to the designated data format.
            //It should be noted that Rust's parser from string to data will fail if the string provided is:
            //outside the minimum or maximum range of the data type. The data type for example is an int/uint but
            //the string shows a float. The data type for example is an int/uint but the string is written in scientific
            //notation.
            //I would like to find some way to create better way to either percolate these errors up or provide a better failure statement.
            match &params.usecols{
                Some(x) =>{
                    results.results.extend({
                            x.iter().map(|y| i64::from_str(line_split_vec[*y].trim()).unwrap())
                        });
                }
                None =>{
                    results.results.extend({
                        line_split_vec.iter().map(|x| i64::from_str(x.trim()).unwrap())
                    });
                }
            }
        }
        //Break out of the loop early if we've read all of the lines that we need to read.
        if results.num_lines == num_lines_read {
            break;
        }
        //Here we clear out the contents of our string vector.
        line.clear();
    }
    //We've finished parsing the file successfully. It is now time to pass the results on.
    Ok(results)
}

///load_txt_i128 reads in a data file that is made up of i128 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to i128. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///Input - f is simply the location of the file.
///        params is ReaderParams structure. An example for what this looks like can be found in the test directory.
///Output - A Result type that either contains a ReaderResults structure or an error. 
///Temporary solution but once this has been written we should be able to create a macro that generates all of this for us...
///A note needs to be added that this needs to better commented at this point.
pub fn load_txt_i128(f: &str, params: &ReaderParams) -> Result<ReaderResults<i128>, Error>{

    let mut file = File::open(f)?;

    //We are finding how many lines in our data file are actually readable and are not commented lines.
    let num_lines = read_num_file_lines(&mut file, params.comments);
    //We need to rewind our file back to the start.
    file.seek(SeekFrom::Start(0))?;
    //We are now creating a reader buffer to easily read through our file
    let mut reader = BufReader::new(file);
    //An allocated string to read in the buffer file.
    let mut line = String::new();

    //We are initializing our ReaderResult structure
    let mut results = ReaderResults{
        num_fields: 0,
        num_lines: 0,
        results: Vec::<i128>::new(),
    };

    //The next portion of lines is some minor error handling to make sure our parameters we provided were valid for our data file.
    //We're checking to see if we have a valid number of skipped lines for the header.
    match &params.skip_header{
        Some(x) => {
            if *x >= num_lines{
                return Err(format_err!("Input for skip_header greater than the number of readable lines in the file"));
            }
        }
        None => (),
    }

    //Now that we know our number is valid we are setting a variable for our skipped header lines to be equal to our skippable lines.
    let sk_h = if let Some(x) = params.skip_header{
        x
    }else{
        0
    };

    //We're checking to see if we have a valid number of skipped lines for the footer.
    match &params.skip_footer{
        Some(x) => {
            if *x >= num_lines {
                return Err(format_err!("Input for skip_footer greater than the number of readable lines in the file"));
            }
        }
        None => (),
    }

    //Now that we know our number is valid we are setting a variable for our skipped footer lines to be equal to our skippable lines.
    let sk_f = if let Some(x) = params.skip_footer{
        x
    }else{
        0
    };
    //We need to error if the number of lines we can read is equal to or less than the number of skipped header and footer lines. 
    if num_lines <= (sk_h + sk_f) {
        return Err(format_err!("Input for skip_footer and skip_header greater than or equal to the number of readable lines in the file"));
    }
    //Here we're determining the number of lines that we need to read from our file.
    let num_lines_read = match &params.max_rows{
        //If we've set a maximum number of rows. We need to first see if that number is greater than
        //the number of readable none skipped lines.
        //If we didn't then the maximum number is set to our number of readable non skipped lines.
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
    //temporary variable to create our comment string
    let tmp = [params.comments.clone()];
    //convert over from binary to a str type for our comment
    let comment = str::from_utf8(&tmp).unwrap();
    
    //File line number used for Error information
    let mut fln = 0;
    //If we skip any header lines then we need to skip forward through the file by
    //the correct number of lines when not taking into account commented lines.
    if sk_h > 0{
        let mut n_line_skip = 0;
        while reader.read_line(&mut line).unwrap() > 0{
            fln += 1;
            //Checking to see if the line is a comment or not
            //if it is increment our counter
            if !line.starts_with(&comment){
                n_line_skip += 1; 
            }
            //If we've reached the number of lines to skip
            //if so we first clear our string and then break
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
            //It also forces the issue of having to potentially reallocate a string each time.
            //If it wasn't for the line split issue down below this probably wouldn't be an issue at all.
            //All of this should hopefully be cleaned up once I get a proper parser that just takes advantage of
            //streaming bytes instead of using strings.
            let tmp_line = line.clone();
            //Increment the number of lines that we've read
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
            //If this is the first valid field that we're reading then we will use its length to determine
            //what is a valid length for all of the other lines.
            if results.num_lines == 1 {
                results.num_fields = line_split_vec.len();
            }
            //If the line we parsed does not have the same number of fields as our initial value then it is an error.
            if line_split_vec.len() != results.num_fields {
                println!("Contents of line_split_vec {:?}", line_split_vec);
                return Err(format_err!("Number of fields provided at line {} is different than the initial field number of {}", fln, results.num_fields));
            }

            //Here we need to parse only the data that we want from string to the designated data format.
            //It should be noted that Rust's parser from string to data will fail if the string provided is:
            //outside the minimum or maximum range of the data type. The data type for example is an int/uint but
            //the string shows a float. The data type for example is an int/uint but the string is written in scientific
            //notation.
            //I would like to find some way to create better way to either percolate these errors up or provide a better failure statement.
            match &params.usecols{
                Some(x) =>{
                    results.results.extend({
                            x.iter().map(|y| i128::from_str(line_split_vec[*y].trim()).unwrap())
                        });
                }
                None =>{
                    results.results.extend({
                        line_split_vec.iter().map(|x| i128::from_str(x.trim()).unwrap())
                    });
                }
            }
        }
        //Break out of the loop early if we've read all of the lines that we need to read.
        if results.num_lines == num_lines_read {
            break;
        }
        //Here we clear out the contents of our string vector.
        line.clear();
    }
    //We've finished parsing the file successfully. It is now time to pass the results on.
    Ok(results)
}