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
///
///Input -
///
/// f is simply the location of the file.
///
/// params is ReaderParams structure. An example for what this looks like can be found in the test directory.
///
///Output -
///
/// A Result type that either contains a ReaderResults structure or an error.
pub fn load_txt_i8(f: &str, params: &ReaderParams) -> Result<ReaderResults<i8>, Error> {
    load_text!(f, params, i8)
}

///load_txt_i16 reads in a data file that is made up of i16 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to i16. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
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
/// A Result type that either contains a ReaderResults structure or an error.
pub fn load_txt_i16(f: &str, params: &ReaderParams) -> Result<ReaderResults<i16>, Error> {
    load_text!(f, params, i16)
}

///load_txt_i32 reads in a data file that is made up of i32 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to i32. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
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
/// A Result type that either contains a ReaderResults structure or an error.
pub fn load_txt_i32(f: &str, params: &ReaderParams) -> Result<ReaderResults<i32>, Error> {
    load_text!(f, params, i32)
}

///load_txt_i64 reads in a data file that is made up of i64 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to i64. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
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
/// A Result type that either contains a ReaderResults structure or an error.
pub fn load_txt_i64(f: &str, params: &ReaderParams) -> Result<ReaderResults<i64>, Error> {
    load_text!(f, params, i64)
}

///load_txt_i128 reads in a data file that is made up of i128 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to i128. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
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
/// A Result type that either contains a ReaderResults structure or an error.
pub fn load_txt_i128(f: &str, params: &ReaderParams) -> Result<ReaderResults<i128>, Error> {
    load_text!(f, params, i128)
}
