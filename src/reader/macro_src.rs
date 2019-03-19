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

macro_rules! parse_txt {
    ($f: expr, $params:expr) => {
        {
        //So, we have a couple different states we could be in
        //nw_line = the beginning of a new line
        //delim = a delimiter character
        //space = any white space
        //cmt = any comment line
        //field = a field we want to keep
        //sk_field = a field we want to skip
        #[derive(Debug, Clone)]
        enum ParseState{
            NwLine,
            Delim,
            Space,
            Cmt,
            Field,
            SkField,
        };
        let mut file = File::open($f)?;

        //We are finding how many lines in our data file are actually readable and are not commented lines.
        let num_lines = read_num_file_lines(&mut file, $params.comments);
        //We need to rewind our file back to the start.
        file.seek(SeekFrom::Start(0))?;

        //We're explicitly using the raw bytes here
        let mut reader = BufReader::with_capacity(BUF_SIZE, file);

        //We are initializing our ReaderResult structure
        let mut results = RawReaderResults{
            num_fields: 0,
            num_lines: 0,
            results: Vec::<u8>::new(),
            index: Vec::<usize>::new(),
        };

        //The next portion of lines is some minor error handling to make sure our parameters we provided were valid for our data file.
        //We're checking to see if we have a valid number of skipped lines for the header.
        match &$params.skip_header{
            Some(x) => {
                if *x >= num_lines{
                    return Err(format_err!("Input for skip_header greater than the number of readable lines in the file"));
                }
            }
            None => (),
        }

        //Now that we know our number is valid we are setting a variable for our skipped header lines to be equal to our skippable lines.
        let sk_h = if let Some(x) = $params.skip_header{
            x
        }else{
            0
        };

        //We're checking to see if we have a valid number of skipped lines for the footer.
        match &$params.skip_footer{
            Some(x) => {
                if *x >= num_lines {
                    return Err(format_err!("Input for skip_footer greater than the number of readable lines in the file"));
                }
            }
            None => (),
        }

        //Now that we know our number is valid we are setting a variable for our skipped footer lines to be equal to our skippable lines.
        let sk_f = if let Some(x) = $params.skip_footer{
            x
        }else{
            0
        };
        //We need to error if the number of lines we can read is equal to or less than the number of skipped header and footer lines.
        if num_lines <= (sk_h + sk_f) {
            return Err(format_err!("Input for skip_footer and skip_header greater than or equal to the number of readable lines in the file"));
        }
        //Here we're determining the number of lines that we need to read from our file.
        let num_lines_read = match &$params.max_rows{
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

        //our comment string
        let cmt = $params.comments.clone();
        let delim_ws = match &$params.delimiter{
                Delimiter::WhiteSpace => {
                    true
                }
                Delimiter::Any(_b) => {
                    false
                }
        };

        let delim = match &$params.delimiter{
            Delimiter::WhiteSpace => {
                b' '
            }
            Delimiter::Any(b) => {
                *b
            }
        };

        //File line number used for Error information
        let mut fln = 0;
        //If we skip any header lines then we need to skip forward through the file by
        //the correct number of lines when not taking into account commented lines.
        if sk_h > 0{
            let mut count = 0;

            //We loop over until the file has been completely read
            loop{
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
                    while i < length{
                        //Here's where the main magic occurs
                        //If we come across a space or tab we move to the next item in the buffer
                        //If we come across a newline character we advance our iterator and move onto the
                        //next index essentially
                        //If we come across a comment character first (white spaces aren't counted) we completely skip the line
                        //If we come across any other character first (white spaces aren't counted) we increment our line counter
                        //and then skip the rest of the contents of the line.
                        //If we no longer have an item in our newline iterator we're done with everything in our buffer, and so
                        //we can exit the loop.
                        if (buffer[i] == b' ') | (buffer[i] == b'\t')  {
                            i += 1;
                        } else if (buffer[i] == b'\n') | (buffer[i] == b'\r') {
                            let val = newline.next();
                            i = match val {
                                Some(val) => val + 1,
                                None => length + 1,
                            };
                            fln += 1;
                        }else if buffer[i] == cmt {
                            let val = newline.next();
                            i = match val {
                                Some(val) => val + 1,
                                None => length + 1,
                            };
                            fln += 1;
                        } else{
                            count += 1;
                            let val = newline.next();
                            i = match val {
                                Some(val) => val + 1,
                                None => length + 1,
                            };
                            fln += 1;
                        }

                        if count == sk_h{
                            break;
                        }
                    }
                    //Pass off our length to set our length outside of this block of code
                    (i - 1)
                };
                //We now need to consume everything upto length in our buffer, so it's marked off as no longer being needed
                reader.consume(length);
                //If our length is less than our fixed buffer size we've reached the end of our file and can now exit.
                if count == sk_h{
                    break;
                }
            }
        }

        //We are now going to loop through everything until we either reach the last line we need or we reach EOF.
        //We need to first set our state to being start of a line.

        let mut state = ParseState::NwLine;

        let cols = match &$params.usecols{
            Some(x) => {x.clone()}
            None => {Vec::<usize>::new()}
        };

        //We need to count our field variables
        let mut field_counter  = 0;
        loop{
            //We first find the length of our buffer
            let length = {
                //We fill the buffer up. Our buffer is mutable which is why it's in this block
                let buffer = reader.fill_buf().unwrap();
                //We're now going to use an explicit loop.
                //I know this isn't idiomatic rust, but I couldn't really see a good way of skipping my iterator
                //to a location of my choosing.
                let mut i = 0;
                //We also need a way to track our state so we can do different things depending on what our previous state was
                //I'll need to write  a simple enum for this that we can traverse over for a variable called state.
                //We're using the memchr crate to locate all of the most common newline characters
                //It provides a nice iterator over our buffer that we can now use.
                let mut newline = memchr2_iter(b'\n', b'\r', buffer);

                //We don't want our loop index to go past our buffer length or else bad things could occur
                let length = buffer.len();
                //Keeping it old school with some nice wild loops
                while i < length{
                    //Here's where the main magic occurs
                    //If we come across a space or tab we move to the next item in the buffer
                    //If we come across a newline character we advance our iterator and move onto the
                    //next index essentially
                    //If we come across a comment character first (white spaces aren't counted) we completely skip the line
                    //If we come across any other character first (white spaces aren't counted) then we're in a line we care about
                    //If we no longer have an item in our newline iterator we're done with everything in our buffer, and so
                    //we can exit the loop.
                    if (buffer[i] == delim) & !delim_ws {
                        state = match state{
                            ParseState::NwLine => {
                                field_counter = 1;
                                ParseState::Delim
                            }
                            ParseState::Delim => {
                                ParseState::Delim
                            }
                            ParseState::SkField => {
                                field_counter += 1;
                                ParseState::Delim
                            }
                            ParseState::Field => {
                                field_counter += 1;
                                results.index.push(results.results.len());
                                ParseState::Delim
                            }
                            ParseState::Cmt => {
                                ParseState::Cmt
                            }
                            ParseState::Space => {
                                field_counter += 1;
                                ParseState::Delim
                            }
                        };
                        i += 1;
                    }else if (buffer[i] == b' ') | (buffer[i] == b'\t')  {
                        if delim_ws{
                            state = match state{
                                ParseState::NwLine => {
                                    field_counter = 1;
                                    ParseState::Delim
                                }
                                ParseState::Delim => {
                                    ParseState::Delim
                                }
                                ParseState::SkField => {
                                    field_counter += 1;
                                    ParseState::Delim
                                }
                                ParseState::Field => {
                                    field_counter += 1;
                                    results.index.push(results.results.len());
                                    ParseState::Delim
                                }
                                ParseState::Cmt => {
                                    ParseState::Cmt
                                }
                                ParseState::Space => {
                                    field_counter += 1;
                                    ParseState::Delim
                                }
                            };
                        } else{
                            state = match state{
                                ParseState::NwLine => {
                                    ParseState::Space
                                }
                                ParseState::Delim => {
                                    ParseState::Space
                                }
                                ParseState::SkField => {
                                    ParseState::Space
                                }
                                ParseState::Field => {
                                    results.index.push(results.results.len());
                                    ParseState::Space
                                }
                                ParseState::Cmt => {
                                    ParseState::Cmt
                                }
                                ParseState::Space => {
                                    ParseState::Space
                                }
                            };
                        }
                        i += 1;
                    }else if (buffer[i] == b'\n') | (buffer[i] == b'\r') {
                        let val = newline.next();
                        i = match val {
                            Some(val) => val + 1,
                            None => length,
                        };
                        fln += 1;
                        state = match state{
                            ParseState::NwLine => {
                                ParseState::NwLine
                            }
                            ParseState::Delim => {
                                if delim_ws {
                                    field_counter = field_counter - 1;
                                    if results.num_lines == 0 {
                                        results.num_fields = field_counter;
                                    }
                                    if (field_counter != results.num_fields) & (field_counter != 0) {
                                        return Err(format_err!("Newline (delim) Number of fields,{}, provided at line {} 
                                        is different than the initial field number of {}", field_counter, fln, results.num_fields));
                                    }
                                    field_counter = 0;
                                    results.num_lines += 1;
                                }else{
                                    return Err(format_err!("Number of fields provided at line {} 
                                        ends with a delimiter instead of a field or white space", fln));
                                }
                                ParseState::NwLine
                            }
                            ParseState::SkField => {
                                if results.num_lines == 0 {
                                    results.num_fields = field_counter;
                                }
                                if field_counter != results.num_fields {
                                    return Err(format_err!("Newline (skip field) Number of fields,{}, provided at line {} 
                                    is different than the initial field number of {}", field_counter, fln, results.num_fields));
                                }
                                results.num_lines += 1;
                                field_counter = 0;
                                ParseState::NwLine
                            }
                            ParseState::Field => {
                                results.index.push(results.results.len());
                                if results.num_lines == 0 {
                                    results.num_fields = field_counter;
                                }
                                if field_counter != results.num_fields {
                                    return Err(format_err!("Newline (field) Number of fields,{}, provided at line {} 
                                    is different than the initial field number of {}", field_counter, fln, results.num_fields));
                                }
                                results.num_lines += 1;
                                field_counter = 0;
                                ParseState::NwLine
                            }
                            ParseState::Cmt => {
                                ParseState::NwLine
                            }
                            ParseState::Space => {
                                ParseState::NwLine
                            }
                        };
                    }else if buffer[i] == cmt {
                        let val = newline.next();
                        i = match val {
                            Some(val) => val + 1,
                            None => length,
                        };
                        fln += 1;
                        state = match state{
                            ParseState::NwLine => {
                                ParseState::NwLine
                            }
                            ParseState::Delim => {
                                if delim_ws {
                                    field_counter = field_counter - 1;
                                    if (results.num_lines == 0) & (field_counter != 0){
                                        results.num_fields = field_counter;
                                    }
                                    if (field_counter != results.num_fields) & (field_counter != 0) {
                                        return Err(format_err!("Cmt (delim) Number of fields,{}, provided at line {} 
                                        is different than the initial field number of {}", field_counter, fln, results.num_fields));
                                    }
                                    if field_counter > 0{
                                        field_counter = 0;
                                        results.num_lines += 1;
                                    }

                                }else{
                                    return Err(format_err!("Number of fields provided at line {} 
                                        ends with a delimiter instead of a field or white space", fln));
                                }
                                ParseState::NwLine
                            }
                            ParseState::SkField => {
                                if results.num_lines == 0 {
                                    results.num_fields = field_counter;
                                }
                                if field_counter != results.num_fields {
                                    return Err(format_err!("Cmt (skip field) Number of fields,{}, provided at line {} 
                                    is different than the initial field number of {}", field_counter, fln, results.num_fields));
                                }
                                results.num_lines += 1;
                                field_counter = 0;
                                ParseState::NwLine
                            }
                            ParseState::Field => {
                                if results.num_lines == 0 {
                                    results.num_fields = field_counter;
                                }
                                results.index.push(results.results.len());
                                if field_counter != results.num_fields {
                                    return Err(format_err!("Cmt (field) Number of fields,{}, provided at line {} 
                                    is different than the initial field number of {}", field_counter, fln, results.num_fields));
                                }
                                results.num_lines += 1;
                                field_counter = 0;
                                ParseState::NwLine
                            }
                            ParseState::Cmt => {
                                ParseState::NwLine
                            }
                            ParseState::Space => {
                                ParseState::NwLine
                            }
                        };
                    }else{
                        state = match state{
                            ParseState::NwLine => {
                                field_counter = 1;
                                match &cols.len(){
                                    0 =>{
                                        results.results.push(buffer[i]);

                                        ParseState::Field
                                    }
                                    _ =>{
                                        let pos = cols.iter().position(|&x| x == field_counter);
                                        match pos{
                                            Some(_x) =>{
                                                results.results.push(buffer[i]);

                                                ParseState::Field
                                            }
                                            None => {
                                                ParseState::SkField
                                            }
                                        }
                                    }
                                }
                            }
                            ParseState::Delim => {
                                match &cols.len(){
                                    0 =>{
                                        results.results.push(buffer[i]);

                                        ParseState::Field
                                    }
                                    _ =>{
                                        let pos = cols.iter().position(|&x| x == field_counter);
                                        match pos{
                                            Some(_x) =>{
                                                results.results.push(buffer[i]);

                                                ParseState::Field
                                            }
                                            None => {
                                                ParseState::SkField
                                            }
                                        }
                                    }
                                }
                            }
                            ParseState::SkField => {
                                ParseState::SkField
                            }
                            ParseState::Field =>{
                                results.results.push(buffer[i]);

                                ParseState::Field
                            }
                            ParseState::Cmt => {
                                ParseState::Cmt
                            }
                            ParseState::Space => {
                                //The case where we start out with spaces before our 1st field at the start of a line
                                if field_counter == 0 {
                                    field_counter += 1;
                                }
                                match &cols.len(){
                                    0 =>{
                                        results.results.push(buffer[i]);

                                        ParseState::Field
                                    }
                                    _ =>{
                                        let pos = cols.iter().position(|&x| x == field_counter);
                                        match pos{
                                            Some(_x) =>{
                                                results.results.push(buffer[i]);

                                                ParseState::Field
                                            }
                                            None => {
                                                ParseState::SkField
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        i += 1;
                    }
                    //Check to see if we've read enough lines in if so break out of the loop
                    if results.num_lines == num_lines_read {
                        break;
                    }
                }
                length
            };
            //We now need to consume everything in our buffer, so it's marked off as no longer being needed
            reader.consume(length);
            //If our length is less than our fixed buffer size we've reached the end of our file and can now exit.
            if length < BUF_SIZE{
                break;
            }else if results.num_lines == num_lines_read {
                break;
            }
        }

        Ok(results)

        }
    };
}

macro_rules! load_text {
    ($f:expr, $params:expr, $type: ident) => {{
        //Get the raw results
        let raw_results = parse_txt!($f, $params);

        let raw_results = match raw_results {
            Ok(x) => x,
            Err(err) => {
                return err;
            }
        };

        //We are initializing our ReaderResult structure
        let num_items = raw_results.index.len();

        let mut results = ReaderResults {
            num_fields: 0,
            num_lines: 0,
            results: Vec::<$type>::with_capacity(num_items),
        };

        results.num_fields = raw_results.num_fields;
        results.num_lines = raw_results.num_lines;

        for i in 0..num_items {
            let j: usize = {
                if i == 0 {
                    0
                } else {
                    raw_results.index[i - 1]
                }
            };
            let k: usize = raw_results.index[i];
            assert!(k <= raw_results.results.len());
            assert!(j < raw_results.results.len());
            assert!(j < k);
            let slice = &raw_results.results[j..k];
            let temp: $type = lexical::try_parse::<$type, _>(slice).unwrap();
            results.results.push(temp);
        }

        Ok(results)
    }};
}

///load_txt! reads in a data file that is made up of primitive type data. If this assumption is not made then the parser will fail
///during the conversion between strings to primitive. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///Input - f is simply the location of the file.
///        params is ReaderParams structure. An example for what this looks like can be found in the test directory.
///        type - the data type we'll be using
///Output - A Result type that either contains a ReaderResults structure or an error.
///Temporary solution but once this has been written we should be able to create a macro that generates all of this for us...
///A note needs to be added that this needs to better commented at this point.
#[doc(hidden)]
macro_rules! load_text_old {
    ($f:expr, $params:expr, $type: ident) => {
        {
        let mut file = File::open($f)?;

        //We are finding how many lines in our data file are actually readable and are not commented lines.
        let num_lines = read_num_file_lines(&mut file, $params.comments);
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
            results: Vec::<$type>::new(),
        };

        //The next portion of lines is some minor error handling to make sure our parameters we provided were valid for our data file.
        //We're checking to see if we have a valid number of skipped lines for the header.
        match &$params.skip_header{
            Some(x) => {
                if *x >= num_lines{
                    return Err(format_err!("Input for skip_header greater than the number of readable lines in the file"));
                }
            }
            None => (),
        }

        //Now that we know our number is valid we are setting a variable for our skipped header lines to be equal to our skippable lines.
        let sk_h = if let Some(x) = $params.skip_header{
            x
        }else{
            0
        };

        //We're checking to see if we have a valid number of skipped lines for the footer.
        match &$params.skip_footer{
            Some(x) => {
                if *x >= num_lines {
                    return Err(format_err!("Input for skip_footer greater than the number of readable lines in the file"));
                }
            }
            None => (),
        }

        //Now that we know our number is valid we are setting a variable for our skipped footer lines to be equal to our skippable lines.
        let sk_f = if let Some(x) = $params.skip_footer{
            x
        }else{
            0
        };
        //We need to error if the number of lines we can read is equal to or less than the number of skipped header and footer lines.
        if num_lines <= (sk_h + sk_f) {
            return Err(format_err!("Input for skip_footer and skip_header greater than or equal to the number of readable lines in the file"));
        }
        //Here we're determining the number of lines that we need to read from our file.
        let num_lines_read = match &$params.max_rows{
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
        let tmp = [$params.comments.clone()];
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
                if (!line.trim_start().starts_with(&comment)) && (!line.trim_start().is_empty()){
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
            // let tline = line.trim_start().to_string();
            if (!line.trim_start().starts_with(&comment)) && (!line.trim_start().is_empty()){
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
                let line_split_vec: Vec<&str> = match &$params.delimiter{
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
                match &$params.usecols{
                    Some(x) =>{
                        results.results.extend({
                                x.iter().map(|y| {
                                    lexical::try_parse::<$type, _>(line_split_vec[*y].trim()).unwrap()
                                    //$type::from_str(line_split_vec[*y].trim()).unwrap())
                                })
                            });
                    }
                    None =>{
                        results.results.extend({
                            line_split_vec.iter().map(|x| {
                                lexical::try_parse::<$type, _>(x.trim()).unwrap()
                                //$type::from_str(x.trim()).unwrap()
                            })
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
    }};
}

#[doc(hidden)]
macro_rules! load_text_lossy {
    ($f:expr, $params:expr, $type: ident) => {
        {
        let mut file = File::open($f)?;

        //We are finding how many lines in our data file are actually readable and are not commented lines.
        let num_lines = read_num_file_lines(&mut file, $params.comments);
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
            results: Vec::<$type>::new(),
        };

        //The next portion of lines is some minor error handling to make sure our parameters we provided were valid for our data file.
        //We're checking to see if we have a valid number of skipped lines for the header.
        match &$params.skip_header{
            Some(x) => {
                if *x >= num_lines{
                    return Err(format_err!("Input for skip_header greater than the number of readable lines in the file"));
                }
            }
            None => (),
        }

        //Now that we know our number is valid we are setting a variable for our skipped header lines to be equal to our skippable lines.
        let sk_h = if let Some(x) = $params.skip_header{
            x
        }else{
            0
        };

        //We're checking to see if we have a valid number of skipped lines for the footer.
        match &$params.skip_footer{
            Some(x) => {
                if *x >= num_lines {
                    return Err(format_err!("Input for skip_footer greater than the number of readable lines in the file"));
                }
            }
            None => (),
        }

        //Now that we know our number is valid we are setting a variable for our skipped footer lines to be equal to our skippable lines.
        let sk_f = if let Some(x) = $params.skip_footer{
            x
        }else{
            0
        };
        //We need to error if the number of lines we can read is equal to or less than the number of skipped header and footer lines.
        if num_lines <= (sk_h + sk_f) {
            return Err(format_err!("Input for skip_footer and skip_header greater than or equal to the number of readable lines in the file"));
        }
        //Here we're determining the number of lines that we need to read from our file.
        let num_lines_read = match &$params.max_rows{
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
        let tmp = [$params.comments.clone()];
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
                if (!line.trim_start().starts_with(&comment)) && (!line.trim_start().is_empty()){
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
            if (!line.trim_start().starts_with(&comment)) && (!line.trim_start().is_empty()){
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
                let line_split_vec: Vec<&str> = match &$params.delimiter{
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
                match &$params.usecols{
                    Some(x) =>{
                        results.results.extend({
                                x.iter().map(|y| {
                                    lexical::try_parse_lossy::<$type, _>(line_split_vec[*y].trim()).unwrap()
                                    //$type::from_str(line_split_vec[*y].trim()).unwrap())
                                })
                            });
                    }
                    None =>{
                        results.results.extend({
                            line_split_vec.iter().map(|x| {
                                lexical::try_parse_lossy::<$type, _>(x.trim()).unwrap()
                                //$type::from_str(x.trim()).unwrap()
                            })
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
    }};
}

#[doc(hidden)]
macro_rules! load_text_other {
    ($f:expr, $params:expr, $type: ident) => {
        {
        let mut file = File::open($f)?;

        //We are finding how many lines in our data file are actually readable and are not commented lines.
        let num_lines = read_num_file_lines(&mut file, $params.comments);
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
            results: Vec::<$type>::new(),
        };

        //The next portion of lines is some minor error handling to make sure our parameters we provided were valid for our data file.
        //We're checking to see if we have a valid number of skipped lines for the header.
        match &$params.skip_header{
            Some(x) => {
                if *x >= num_lines{
                    return Err(format_err!("Input for skip_header greater than the number of readable lines in the file"));
                }
            }
            None => (),
        }

        //Now that we know our number is valid we are setting a variable for our skipped header lines to be equal to our skippable lines.
        let sk_h = if let Some(x) = $params.skip_header{
            x
        }else{
            0
        };

        //We're checking to see if we have a valid number of skipped lines for the footer.
        match &$params.skip_footer{
            Some(x) => {
                if *x >= num_lines {
                    return Err(format_err!("Input for skip_footer greater than the number of readable lines in the file"));
                }
            }
            None => (),
        }

        //Now that we know our number is valid we are setting a variable for our skipped footer lines to be equal to our skippable lines.
        let sk_f = if let Some(x) = $params.skip_footer{
            x
        }else{
            0
        };
        //We need to error if the number of lines we can read is equal to or less than the number of skipped header and footer lines.
        if num_lines <= (sk_h + sk_f) {
            return Err(format_err!("Input for skip_footer and skip_header greater than or equal to the number of readable lines in the file"));
        }
        //Here we're determining the number of lines that we need to read from our file.
        let num_lines_read = match &$params.max_rows{
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
        let tmp = [$params.comments.clone()];
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
                if (!line.trim_start().starts_with(&comment)) && (!line.trim_start().is_empty()){
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
            if (!line.trim_start().starts_with(&comment)) && (!line.trim_start().is_empty()){
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
                let line_split_vec: Vec<&str> = match &$params.delimiter{
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
                match &$params.usecols{
                    Some(x) =>{
                        results.results.extend({
                                x.iter().map(|y| {
                                    //lexical::try_parse::<$type, _>(line_split_vec[*y].trim()).unwrap()
                                    $type::from_str(line_split_vec[*y].trim()).unwrap()
                                })
                            });
                    }
                    None =>{
                        results.results.extend({
                            line_split_vec.iter().map(|x| {
                                //lexical::try_parse::<$type, _>(x.trim()).unwrap()
                                $type::from_str(x.trim()).unwrap()
                            })
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
    }};
}
