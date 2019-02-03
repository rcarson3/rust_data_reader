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

//!# Rust Data Reader
//!
//!So far this code provides some similar capabilities as Numpy's loadtxt to Rust. It is currently intended to read in data that you know how it's been generated. The default delimiter is any whitespace character. The following caviates currently exist:
//!
//!1.  New line and commented lines are not counted in the lines that you want skipped or that have been read.
//!2.  If the code fails to convert from a string to the supported type it will fail.
//!3.  Whitespaces are stripped from the front and end of whatever string is between delimeters.
//!4.  All of the data being read in needs to be the same type. 
//!
//!It provides support for the following types:
//!
//!```Rust
//!u8 u16 u32 u64 u128 usize
//!i8 i16 i32 i64 i128
//!f32 f64
//!char bool String
//!```
//!
//!The primitive uint, int, and floats use the lexical crate to provide a faster conversion from string to the given type. The other types use the built in standard library from_str conversion. The read in data is all stored into a vector. A struct is returned from the method ```load_text_*``` that provides the number of lines read, the number of columns read from the data, and a vector containing the data. This struct is wrapped into a Result that is returned to the user. For a 1GB float64 type file read from an SSD, you should expect roughly a 88MB/s for the read in. 
//!
//!If your data file doesn't meet these types you might want to look into BurntSushi's CSV crate for your needs.
//!
//!# Roadmap
//!Update the backend such that it makes it possible to have multiple data types in the file being read.
//!
//!Update the backend to provide better performance. 
//!
//!New backend should look closer to what's currently being done in BurntSushi's CSV crate.
//!
//!Provide access to a macro to build the reader into their own code which could give the compiler the necessary information to optimize away dead branches.
//!
//!# Example
//!An example of how to use the code can be seen down below:
//!
//!```Rust
//!//This example shows us how we might skip a footer file
//!fn load_txt_i32_test_sk_f(){
//!    //The file here is the one included in the main folder.
//!    let file = String::from("int_testv2.txt");
//!
//!    let params = ReaderParams{
//!        comments: b'%',
//!        delimiter: Delimiter::WhiteSpace,
//!        skip_header: None,
//!        skip_footer: Some(5),
//!        usecols: None,
//!        max_rows: None,
//!    };
//!
//!    let results = load_txt_i32(&file, &params);
//!
//!    // Pattern matching for our results could look something like this.
//!    // match results{
//!    //     Ok(results) => println!("Number of lines {}\nNumber of fields {}\nResults {:?}",results.num_lines, results.num_fields, results.results),
//!    //     Err(err) => println!("Error {:?}", err),
//!    // }
//!
//!    assert_eq!(results.unwrap().results, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
//!
//!}
//!```

extern crate bytecount;
#[macro_use]
extern crate failure;
extern crate lexical;

///Contains all of the functions related to the different readers that will be generated.
pub mod reader; 
