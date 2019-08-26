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

///load_txt! reads in a data file that is made up of primitive type data. If this assumption is not made then the parser will fail
///during the conversion between &[u8] to primitive. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///Input - f is simply the location of the file.
///        params is ReaderParams structure. An example for what this looks like can be found in the test directory.
///        type - the data type we'll be using
///Output - A Result type that either contains a ReaderResults structure or an error.
#[doc(hidden)]
macro_rules! load_text_lexical {
    ($f:expr, $params:expr, $type: ident) => {{
        //Get the raw results
        let raw_results = parse_txt($f, $params)?;

        //We are initializing our ReaderResult structure
        let num_items = raw_results.index.len();

        let mut results = ReaderResults {
            num_fields: 0,
            num_lines: 0,
            results: Vec::<$type>::with_capacity(num_items + 1),
        };

        results.num_fields = raw_results.num_fields;
        results.num_lines = raw_results.num_lines;
        //Converting all of the data over using lexical
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
            let temp: $type = lexical::parse::<$type, _>(slice).unwrap();
            results.results.push(temp);
        }

        Ok(results)
    }};
}

///load_txt_lossy! reads in a data file that is made up of primitive type data. If this assumption is not made then the parser will fail
///during the conversion between &[u8] to primitive. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///It uses lexical lossy formula for potentially quicker conversions of types.
///Input - f is simply the location of the file.
///        params is ReaderParams structure. An example for what this looks like can be found in the test directory.
///        type - the data type we'll be using
///Output - A Result type that either contains a ReaderResults structure or an error.
#[doc(hidden)]
macro_rules! load_text_lossy {
    ($f:expr, $params:expr, $type: ident) => {{
        //Get the raw results
        let raw_results = parse_txt($f, $params)?;

        //We are initializing our ReaderResult structure
        let num_items = raw_results.index.len();

        let mut results = ReaderResults {
            num_fields: 0,
            num_lines: 0,
            results: Vec::<$type>::with_capacity(num_items + 1),
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
            let temp: $type = lexical::parse_lossy::<$type, _>(slice).unwrap();
            results.results.push(temp);
        }

        Ok(results)
    }};
}
///load_txt! reads in a data file that is made up of any type data that supports FromStr trait. If this assumption is not made then the parser will fail
///during the conversion between &[u8] to the type. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///Input - f is simply the location of the file.
///        params is ReaderParams structure. An example for what this looks like can be found in the test directory.
///        type - the data type we'll be using
///Output - A Result type that either contains a ReaderResults structure or an error.
#[macro_export]
macro_rules! load_text {
    ($f:expr, $params:expr, $type: ident) => {{
        //Get the raw results
        let raw_results = parse_txt($f, $params)?;

        //We are initializing our ReaderResult structure
        let num_items = raw_results.index.len();

        let mut results = ReaderResults {
            num_fields: 0,
            num_lines: 0,
            results: Vec::<$type>::with_capacity(num_items + 1),
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
            let temp_str = str::from_utf8(slice)?;
            let temp = $type::from_str(&temp_str)?;
            results.results.push(temp);
        }

        Ok(results)
    }};
}
