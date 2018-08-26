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
   
   let mut file = File::open("int_testv3.txt").unwrap();
   let tot_num_lines = read_num_file_tot_lines(&mut file);
   println!("The total number of lines in the file is {}", tot_num_lines);
   //Rewind it back to the start.
   file.seek(SeekFrom::Start(0)).unwrap();
   let num_lines = read_num_file_lines(&mut file, b'%');
   println!("The number of lines in the file minus comments is {}", num_lines);
   assert_eq!((tot_num_lines - num_lines), 3);
}

//The test file for this has 0 commented lines in it
#[test]
fn load_txt_i32_test(){
    let file = String::from("int_testv2.txt");

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

    // match results{
    //     Ok(results) => println!("Number of lines {}\nNumber of fields {}\nResults {:?}",results.num_lines, results.num_fields, results.results),
    //     Err(err) => println!("Error {:?}", err),
    // }

    assert_eq!(results.unwrap().results, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
}

#[test]
fn load_txt_i32_test_sk_f(){
    let file = String::from("int_testv2.txt");

    let params = ReaderParams{
        dtype: String::from("int32"),
        comments: b'%',
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: Some(5),
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(results.unwrap().results, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

}

#[test]
fn load_txt_i32_test_sk_h(){
    let file = String::from("int_testv2.txt");

    let params = ReaderParams{
        dtype: String::from("int32"),
        comments: b'%',
        delimiter: Delimiter::WhiteSpace,
        skip_header: Some(3),
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(results.unwrap().results, vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
}

#[test]
fn load_txt_i32_test_mrows(){
    let file = String::from("int_testv2.txt");

    let params = ReaderParams{
        dtype: String::from("int32"),
        comments: b'%',
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: Some(8),
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(results.unwrap().results, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]);
}

#[test]
fn load_txt_i32_test_sk_f_big(){
    let file = String::from("int_testv2.txt");

    let params = ReaderParams{
        dtype: String::from("int32"),
        comments: b'%',
        delimiter: Delimiter::WhiteSpace,
        skip_header: Some(3),
        skip_footer: Some(11),
        usecols: None,
        max_rows: Some(8),
    };

    let results = load_txt_i32(&file, &params);

    assert!(results.is_err());

}

#[test]
fn load_txt_i32_test_u_cols(){
    let file = String::from("int_testv2.txt");

    let cols: Vec<usize> = vec![2];

    let params = ReaderParams{
        dtype: String::from("int32"),
        comments: b'%',
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: Some(cols),
        max_rows: None,
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(results.unwrap().results, vec![3,6,9,12,15,18,21,24,27,30]);
}

//This file for this test has 3 commented lines in it.
#[test]
fn load_txt_i32_test2(){
    let file = String::from("int_testv3.txt");

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

    assert_eq!(results.unwrap().results,vec![1, 2, 3, 4, 5, 6, 10, 11, 12, 13, 14, 15, 19, 20, 21, 25, 26, 27, 28, 29, 30]);

}