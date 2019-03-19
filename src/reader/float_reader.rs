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

///load_txt_f32 reads in a data file that is made up of f32 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to f32. It can also fail in a number of other ways related to invalid parameters or the
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
pub fn load_txt_f32(f: &str, params: &ReaderParams) -> Result<ReaderResults<f32>, Error> {
    load_text!(f, params, f32)
}

///load_txt_f64 reads in a data file that is made up of f64 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to f64. It can also fail in a number of other ways related to invalid parameters or the
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

pub fn load_txt_f64(f: &str, params: &ReaderParams) -> Result<ReaderResults<f64>, Error> {
    load_text!(f, params, f64)
}

pub fn load_txt_old_f64(f: &str, params: &ReaderParams) -> Result<ReaderResults<f64>, Error> {
    load_text_old!(f, params, f64)
}

///load_txt_lossy_f32 reads in a data file that is made up of f32 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to f32. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///
/// This function makes use of lexical's lossy algorithms, so it's only good to machine precision.
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
pub fn load_txt_lossy_f32(f: &str, params: &ReaderParams) -> Result<ReaderResults<f32>, Error> {
    load_text_lossy!(f, params, f32)
}

///load_txt_lossy_f64 reads in a data file that is made up of f64 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to f64. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///
/// This function makes use of lexical's lossy algorithms, so it's only good to machine precision.
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
pub fn load_txt_lossy_f64(f: &str, params: &ReaderParams) -> Result<ReaderResults<f64>, Error> {
    load_text_lossy!(f, params, f64)
}
