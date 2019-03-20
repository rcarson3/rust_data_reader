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

///load_txt_u8 reads in a data file that is made up of u8 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to u8. It can also fail in a number of other ways related to invalid parameters or the
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
pub fn load_txt_u8(f: &str, params: &ReaderParams) -> Result<ReaderResults<u8>, Error> {
    load_text!(f, params, u8)
}

///load_txt_u16 reads in a data file that is made up of u16 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to u16. It can also fail in a number of other ways related to invalid parameters or the
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
pub fn load_txt_u16(f: &str, params: &ReaderParams) -> Result<ReaderResults<u16>, Error> {
    load_text!(f, params, u16)
}

///load_txt_u32 reads in a data file that is made up of u32 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to u32. It can also fail in a number of other ways related to invalid parameters or the
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
pub fn load_txt_u32(f: &str, params: &ReaderParams) -> Result<ReaderResults<u32>, Error> {
    load_text!(f, params, u32)
}

///load_txt_u64 reads in a data file that is made up of u64 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to u64. It can also fail in a number of other ways related to invalid parameters or the
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
pub fn load_txt_u64(f: &str, params: &ReaderParams) -> Result<ReaderResults<u64>, Error> {
    load_text!(f, params, u64)
}

///load_txt_u128 reads in a data file that is made up of u128 type data. If this assumption is not made then the parser will fail
///during the conversion between strings to u128. It can also fail in a number of other ways related to invalid parameters or the
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
pub fn load_txt_u128(f: &str, params: &ReaderParams) -> Result<ReaderResults<u128>, Error> {
    load_text!(f, params, u128)
}

///load_txt_usize reads in a data file that is made up of usize type data. If this assumption is not made then the parser will fail
///during the conversion between strings to usize. It can also fail in a number of other ways related to invalid parameters or the
///data file having malformed fields. These errors are percolated up to whatever is calling this in the form of the Error type.
///One should therefore check to make sure no errors are obtained when examining the file. If a malformed field is seen the error
///does contain information about what line number of the data file has the malformed field.
///
///Input -
///
/// f is simply the location of the file.
///        
///
/// params is ReaderParams structure. An example for what this looks like can be found in the test directory.
///
///Output -
///
/// A Result type that either contains a ReaderResults structure or an error.
pub fn load_txt_usize(f: &str, params: &ReaderParams) -> Result<ReaderResults<usize>, Error> {
    load_text!(f, params, usize)
}
