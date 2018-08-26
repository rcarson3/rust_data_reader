extern crate rust_data_reader;
use rust_data_reader::reader::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

#[test]
fn read_num_file_line_test(){
   let mut file = File::open("LICENSE-APACHE").unwrap();
   let tot_num_lines = read_num_file_tot_lines(&mut file);
   println!("The total number of lines in the file is {}", tot_num_lines);
   //Rewind it back to the start.
   file.seek(SeekFrom::Start(0)).unwrap();
   let num_lines = read_num_file_lines(&mut file, b'#');
   println!("The number of lines in the file minus comments is {}", num_lines);
   assert_eq!((tot_num_lines - num_lines), 0);
   
   let mut file = File::open("int_test.txt").unwrap();
   let tot_num_lines = read_num_file_tot_lines(&mut file);
   println!("The total number of lines in the file is {}", tot_num_lines);
   //Rewind it back to the start.
   file.seek(SeekFrom::Start(0)).unwrap();
   let num_lines = read_num_file_lines(&mut file, b'%');
   println!("The number of lines in the file minus comments is {}", num_lines);
   assert_eq!((tot_num_lines - num_lines), 0);
}

#[test]
fn load_txt_i32_test(){
    let file = String::from("int_test.txt");

    let params = ReaderParams{
        dtype: String::from("int32"),
        comments: b'%',
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i32(&file, &params);

    match results{
        Ok(_results) => (),//println!("{:?}", results.results),
        Err(err) => println!("Error {:?}", err),
    }
}