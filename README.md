# Rust Data Reader

So far this code provides similar capabilities as Numpy's loadtxt to Rust. You can read up on the documentation at [doc.rs](https://docs.rs/data_reader/0.2.0/data_reader/). It is currently intended to read in data that you know how it's been generated. The default delimiter is any whitespace character. The following caviates currently exist:

1.  New line and commented lines are not counted in the lines that you want skipped or that have been read.
2.  If the code fails to convert from a string to the supported type it will fail.
3.  Whitespaces are stripped from the front and end of whatever string is between delimeters.
4.  All of the data being read in needs to be the same type when converted to that type. 

It provides support for the following primitive types:

```Rust
u8 u16 u32 u64 u128 usize
i8 i16 i32 i64 i128
f32 f64
char bool String
```

The primitive uint, int, and floats use the lexical crate to provide a faster conversion from string to the given type. The other types use the built in standard library from_str conversion. The read in data is all stored into a vector. A struct is returned from the method ```load_text_*``` that provides the number of lines read, the number of columns read from the data, and a vector containing the data. This struct is wrapped into a Result that is returned to the user. For a 1GB float64 type file read from an SSD, I was able to obtain 135MB/s for the read in speeds. 

If the type you're intrested in supports the ```FromStr``` trait you can also use this crate you can use the bottom example for how to use the ```load_txt!``` macro to load up a custom data type.

# Roadmap
Update the backend such that it makes it possible to have multiple data types in the file being read.

Provide access to a macro to build the reader into their own code which could give the compiler the necessary information to optimize away dead branches.

# Example
An example of how to use the code can be seen down below:

```Rust
//This example shows us how we might skip a footer file
fn load_txt_i32_test_sk_f(){
    //The file here is the one included in the main folder.
    let file = String::from("int_testv2.txt");

    //A default constructor could look like this:
    //let params = ReaderParams::default();
    //The below could also look like the following:
    //let params = ReaderParams{
    //     comments: Some(b'%'),
    //     skip_footer: Some(5),
    //     ..Default::default()
    //};
    let params = ReaderParams{
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: Some(5),
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i32(&file, &params);

    // Pattern matching for our results could look something like this.
    // match results{
    //     Ok(results) => println!("Number of lines {}\nNumber of fields {}\nResults {:?}",results.num_lines, results.num_fields, results.results),
    //     Err(err) => println!("Error {:?}", err),
    // }

    assert_eq!(results.unwrap().results, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

}
```

Here's a more extensive example showing how to use custom types.

```Rust
#[macro_use]
extern crate data_reader;
extern crate failure;
use data_reader::reader::*;
use failure::Error;

use std::str;
use std::str::FromStr;
use std::vec::*;

//Everything needed for our custom type
#[derive(Debug, PartialEq, Clone)]
struct MinInt{
    x: i32,
}
//A simple example of implementing the FromStr trait for our custom type
impl FromStr for MinInt{
    type Err = Error;

    fn from_str(s: &str) -> Result<MinInt, failure::Error> {
        let temp = -1 * i32::from_str(s)?;
        Ok(MinInt{x: temp})
    }
}

//The test file for this has 0 commented lines in it but using a custom type
//The returned error is needed if we doing anything that's not in a function
fn load_txt_custom_test() -> Result<(), failure::Error> {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let ref_file = &file;
    let ref_params = &params;

    //I found the type annotation was needed for this to compile
    let results: Result<ReaderResults<MinInt>, Error> = load_text!(ref_file, ref_params, MinInt);

    let temp = results.unwrap().results.clone();

    let vals: Vec<i32> = temp.iter().map(|x| x.x).collect();

    assert_eq!(
        vals,
        vec![
            -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14,
            -15, -16, -17, -18, -19, -20, -21, -22, -23, -24, -25, -26, -27,
            -28, -29, -30
        ]
    );

    Ok(())
}

```

# Versions
* 0.3.0 - A bug was noted in the use_cols field of the ReaderParams struct that allowed you to input values that weren't useable. Also, the ReaderParams comment field was updated to being an option. Additional documentation was also added to note that the use_cols field assumes values start with an index of 1.
* 0.2.0 - A new parsing backend has been added which saw a 40% improvement parsing/reading in a large 1GB file of all f64s. Exposed the parser to the end user so the user can deal with the raw bytes if they would enjoy doing so. Any type that now supports the ```FromStr``` trait can be converted over.  

* 0.1.3 - Updated the code to provide a bug fix that was within the v2.0 of the lexical crate.

* 0.1.2 - Updated the comment and newline tracking portion of the code. The code now properly skips over new lines and commented lines that start with whitespace. It also can no handle lines with multiple comment characters in it without counting that line multiple times. A performance regression was created by properly handling these cases from the 0.1.1 and 0.1.0 releases.

* 0.1.1 - Needed to update documentation for docs.rs

* 0.1.0 - Initial crates.io release

