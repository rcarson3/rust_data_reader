# rust_data_reader

An attempt at bringing into Rust data file readers that are similar in scope to those offered by the numpy package in python for genfromtxt and loadtxt. It currently is pretty rough and probably should not used by anyone in production. It is bound to be slow. The erro handling is okay as of right now but it could be better. A vast number of edge cases still need to be tested. The data types currently supported are all of the primitave ints, uints, and floats types supported by Rust. 

The code is very much in a pre-alpha state currently. Once all of the primitative types have been added. The find comment lines has improved then one might start to be able to use on data files without missing data. Data that is missing the option type will be used to wrap the data.

# Example
An example of how to use the code can be seen down below:

```Rust
//This example shows us how we might skip a footer file
fn load_txt_i32_test_sk_f(){
    //The file here is the one included in the main folder.
    let file = String::from("int_testv2.txt");

    let params = ReaderParams{
        comments: b'%',
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

