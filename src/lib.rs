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

//!# rust_data_reader
//!
//!An attempt at bringing into Rust data file readers that are similar in scope to those offered by 
//!the numpy package in python for genfromtxt and loadtxt. It currently is pretty rough and should not 
//!used by anyone. It is bound to be slow. The erro handling is okay as of right now but it could be better.
//! A vast number of edge cases still need to be tested. It also has currently only been examined for type data of int32. 
//!
//!The code is very much in a pre-alpha state currently. Once all of the primitative types have been added.
//! The find comment lines has improved then one might start to be able to use on data files without missing data.
//! Data that is missing the option type will be used to wrap the data.

extern crate bytecount;
#[macro_use]
extern crate failure;

///Contains all of the functions related to the different readers that will be generated.
pub mod reader; 
