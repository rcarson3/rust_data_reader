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

///load_txt_string reads in a data file that is made up of string type data. If this assumption is not made then the parser will fail
///during the conversion between strings to string. It can also fail in a number of other ways related to invalid parameters or the
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
pub fn load_txt_string(f: &str, params: &ReaderParams) -> Result<Box<dyn ReaderResults<String>>, Error> {
    load_text!(f, params, String)
}

///load_txt_bool reads in a data file that is made up of bool type data. If this assumption is not made then the parser will fail
///during the conversion between strings to bool. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///Bool values must be written as true or false for Rust's native from_str to work. If you have it as a series of numbers then you'll want
/// to use int types instead and manually convert things.
///
///Input -
///
/// f is simply the location of the file.
///        
/// params is ReaderParams structure. An example for what this looks like can be found in the test directory
///
///Output -
///
/// A Result type that either contains a ReaderResults structure or an error.
pub fn load_txt_bool(f: &str, params: &ReaderParams) -> Result<Box<dyn ReaderResults<bool>>, Error> {
    load_text!(f, params, bool)
}

///load_txt_char reads in a data file that is made up of char type data. If this assumption is not made then the parser will fail
///during the conversion between strings to char. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///This works as long as your data is simply single chars with a delimiter next to them. It will not return white spaces although since
///those are stripped from each line.
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
pub fn load_txt_char(f: &str, params: &ReaderParams) -> Result<Box<dyn ReaderResults<char>>, Error> {
    load_text!(f, params, char)
}
