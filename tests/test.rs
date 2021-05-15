#[macro_use]
extern crate data_reader;
extern crate failure;
extern crate lexical;
use data_reader::reader::*;
use failure::Error;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::str;
use std::str::FromStr;
use std::vec::*;

const BUF_SIZE: usize = 8 * (1 << 12);
#[test]
fn read_num_file_line_test() {
    let file = File::open("LICENSE-APACHE").unwrap();
    let mut reader = BufReader::with_capacity(BUF_SIZE, file);
    let tot_num_lines = read_num_file_tot_lines(&mut reader);
    println!("The total number of lines in the file is {}", tot_num_lines);
    //Rewind it back to the start.
    reader.seek(SeekFrom::Start(0)).unwrap();
    let num_lines = read_num_file_lines(&mut reader, b'#');
    println!(
        "The number of lines in the file minus comments is {}",
        num_lines
    );
    assert_eq!((tot_num_lines - num_lines), 32);

    let file = File::open("int_testv3.txt").unwrap();
    let mut reader = BufReader::with_capacity(BUF_SIZE, file);
    let tot_num_lines = read_num_file_tot_lines(&mut reader);
    println!("The total number of lines in the file is {}", tot_num_lines);
    //Rewind it back to the start.
    reader.seek(SeekFrom::Start(0)).unwrap();
    let num_lines = read_num_file_lines(&mut reader, b'%');
    println!(
        "The number of lines in the file minus comments is {}",
        num_lines
    );
    assert_eq!((tot_num_lines - num_lines), 5);
}

//The test file for this has 0 commented lines in it
#[test]
fn load_txt_i32_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30
        ]
    );
}

#[test]
fn load_txt_i32_reader_params_constructor_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        ..Default::default()
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30
        ]
    );
}

#[test]
fn load_txt_i32_reader_params_default_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams::default();

    let results = load_txt_i32(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30
        ]
    );
}

#[test]
fn load_txt_i32_test_sk_f() {
    let file = String::from("int_testv2.txt");

    // let params = ReaderParams {
    //     comments: Some(b'%'),
    //     delimiter: Delimiter::WhiteSpace,
    //     skip_header: None,
    //     skip_footer: Some(5),
    //     usecols: None,
    //     max_rows: None,
    // };

    let params = ReaderParams {
        comments: Some(b'%'),
        skip_footer: Some(5),
        ..Default::default()
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    );
}

#[test]
fn load_txt_i32_test_sk_h() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: Some(3),
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]
    );
}

#[test]
fn load_txt_i32_test_mrows() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: Some(8),
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]
    );
}

#[test]
fn load_txt_i32_test_sk_f_big() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
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
fn load_txt_i32_test_u_cols() {
    let file = String::from("int_testv2.txt");

    let cols: Vec<usize> = vec![2];

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: Some(cols),
        max_rows: None,
    };

    let results = load_txt_i32(&file, &params);

    let results = results.unwrap();

    assert_eq!(results.results, vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30]);

    assert_eq!(results.num_fields, 1);

    assert_eq!(results.num_lines, 10);
}

#[test]
#[should_panic]
fn load_txt_i32_test_u_cols_at_bnds() {
    let file = String::from("int_testv2.txt");

    let cols: Vec<usize> = vec![3];

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: Some(cols),
        max_rows: None,
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30]
    );
}

#[test]
#[should_panic]
fn load_txt_i32_test_u_cols_out_bnds() {
    let file = String::from("int_testv2.txt");

    let cols: Vec<usize> = vec![5];

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: Some(cols),
        max_rows: None,
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30]
    );
}

//This file for this test has 3 commented lines in it.
#[test]
fn load_txt_i32_test2() {
    let file = String::from("int_testv3.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![1, 2, 3, 4, 5, 6, 10, 11, 12, 13, 14, 15, 19, 20, 21, 25, 26, 27, 28, 29, 30]
    );
}

//This file for this test has 3 commented lines in it and uses "," for the delimiter.
//So, it also tests the delimiter cases as well.
//It should return the same results as load_txt_i32_test2()
#[test]
fn load_txt_i32_test3() {
    let file = String::from("int_testv4.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::Any(b','),
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i32(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![1, 2, 3, 4, 5, 6, 10, 11, 12, 13, 14, 15, 19, 20, 21, 25, 26, 27, 28, 29, 30]
    );
}

//The test file for this has 0 commented lines in it
#[test]
fn load_txt_i8_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i8(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30
        ]
    );
}

//The test file for this has 0 commented lines in it
#[test]
fn load_txt_i16_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i16(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30
        ]
    );
}

//The test file for this has 0 commented lines in it
#[test]
fn load_txt_i64_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i64(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30
        ]
    );
}

//The test file for this has 0 commented lines in it
#[test]
fn load_txt_u8_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_u8(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30
        ]
    );
}

//The test file for this has 0 commented lines in it
#[test]
fn load_txt_usize_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_usize(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30
        ]
    );
}

//The test file for this has 0 commented lines in it
#[test]
fn load_txt_usize_no_cmt_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: None,
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_usize(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30
        ]
    );
}

//The test file for this has 0 commented lines in it
#[test]
fn load_txt_f32_test() {
    let file = String::from("float_testv1.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_f32(&file, &params);

    assert_eq!(
        results.unwrap().results,
        vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0
        ]
    );
}

//The test file is in scientific notation to test functions ability to parse floating point numbers
#[test]
fn load_txt_f32_sci_test() {
    let file = String::from("int_testv2_sci.txt");
    let file_ref = &file;

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let params_ref = &params;

    let results = load_txt_f32(file_ref, params_ref);

    assert_eq!(
        results.unwrap().results,
        vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0
        ]
    );
}
#[test]
#[ignore]
fn load_txt_f64_sci_test() {
    let file = String::from("grainData_LOFEM.rods");
    let file_ref = &file;

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let params_ref = &params;

    let results = load_txt_f64(file_ref, params_ref);

    let end_results = vec![
        7.211103267499999192e-02,
        -3.999458292499999401e-01,
        -1.908364359999999982e-01,
        5.252514306249998766e-02,
        -3.486972928750000089e-01,
        -2.022221066250000088e-01,
        3.201516763750000133e-02,
        -3.624113797500000400e-01,
        -2.229083163749999985e-01,
    ];

    let comp = &results.unwrap().results;
    let len = comp.len();
    let slice = &comp[(len - 9)..len];

    assert_eq!(slice, &end_results[..]);
}

//The test file for this has 0 commented lines in it
#[test]
fn load_txt_string_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_string(&file, &params);

    let r_int = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30,
    ];

    let string_vec: Vec<String> = r_int.iter().map(|x| x.to_string()).collect();

    assert_eq!(results.unwrap().results, string_vec);
}

#[test]
fn load_txt_bool_test() {
    let file = String::from("bool_test.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_bool(&file, &params);

    let b_vec = vec![
        true, false, true, true, true, true, true, true, true, false, false, false, true, true,
        true, true, false, true,
    ];

    assert_eq!(results.unwrap().results, b_vec);
}

#[test]
fn load_txt_char_test() {
    let file = String::from("char_test.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_char(&file, &params);

    let c_vec = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'a',
    ];

    assert_eq!(results.unwrap().results, c_vec);
}

//Everything needed for our custom type
#[derive(Debug, PartialEq, Clone)]
struct MinInt {
    x: i32,
}
//A simple example of implementing the FromStr trait for our custom type
impl FromStr for MinInt {
    type Err = Error;

    fn from_str(s: &str) -> Result<MinInt, failure::Error> {
        let temp = -1 * i32::from_str(s)?;
        Ok(MinInt { x: temp })
    }
}

//The test file for this has 0 commented lines in it but using a custom type
#[test]
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

    let results: Result<ReaderResults<MinInt>, Error> = load_text!(ref_file, ref_params, MinInt);

    let temp = results.unwrap().results.clone();

    let vals: Vec<i32> = temp.iter().map(|x| x.x).collect();

    assert_eq!(
        vals,
        vec![
            -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19,
            -20, -21, -22, -23, -24, -25, -26, -27, -28, -29, -30
        ]
    );

    Ok(())
}

#[test]
fn get_value_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i64(&file, &params).unwrap();

    assert_eq!(results.get_value(0, 2), 3);
}

#[test]
#[should_panic]
fn get_value_test_fail() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i64(&file, &params).unwrap();

    let _value = results.get_value(1, 3);
}

#[test]
fn get_row_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i64(&file, &params).unwrap();

    assert_eq!(results.get_row(0), vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn get_row_test_fail() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i64(&file, &params).unwrap();

    let _value = results.get_row(10);
}

#[test]
fn get_rows_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i64(&file, &params).unwrap();

    let row_indices = vec![0, 1];

    let vals = vec![vec![1, 2, 3], vec![4, 5, 6]];

    assert_eq!(results.get_rows(row_indices), vals);
}

#[test]
#[should_panic]
fn get_rows_test_fail() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i64(&file, &params).unwrap();
    let row_indices = vec![0, 10];

    let _value = results.get_rows(row_indices);
}

#[test]
fn get_col_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i64(&file, &params).unwrap();

    assert_eq!(
        results.get_col(2),
        vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30]
    );
}

#[test]
#[should_panic]
fn get_col_test_fail() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i64(&file, &params).unwrap();

    let _value = results.get_col(3);
}

#[test]
fn get_cols_test() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i64(&file, &params).unwrap();

    let col_indices = vec![0, 2];

    let vals = vec![
        vec![1, 4, 7, 10, 13, 16, 19, 22, 25, 28],
        vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30],
    ];

    assert_eq!(results.get_cols(col_indices), vals);
}

#[test]
#[should_panic]
fn get_cols_test_fail() {
    let file = String::from("int_testv2.txt");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,
        skip_footer: None,
        usecols: None,
        max_rows: None,
    };

    let results = load_txt_i64(&file, &params).unwrap();

    let col_indices = vec![0, 3];

    let _value = results.get_cols(col_indices);
}
