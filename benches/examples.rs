extern crate data_reader;
use data_reader::reader::*;

#[macro_use]
extern crate criterion;

use criterion::{Criterion};

//The test file is in scientific notation to test functions ability to parse floating point numbers
fn load_txt_f64_large_test() {
    let file = String::from("grainData_LOFEM.rods");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None, //Some(100000),//None,
        skip_footer: None,
        usecols: None,  //Some(cols),
        max_rows: None, //Some(100000),//None,
        ..Default::default()
    };

    let _results = load_txt_f64(&file, &params);
}
/*
//The test file is in scientific notation to test functions ability to parse floating point numbers
fn load_txt_f64_col_large_test() {
    let file = String::from("grainData_LOFEM.rods");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None, //Some(100000),//None,
        skip_footer: None,
        usecols: None,  //Some(cols),
        max_rows: None, //Some(100000),//None,
        row_format: false,
    };

    let _results = load_txt_f64(&file, &params);
}

//The test file is in scientific notation to test functions ability to parse floating point numbers
fn load_txt_f64_col_get_col_large_test() {
    let file = String::from("grainData_LOFEM.rods");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None, //Some(100000),//None,
        skip_footer: None,
        usecols: None,  //Some(cols),
        max_rows: None, //Some(100000),//None,
        row_format: false,
    };

    let results = load_txt_f64(&file, &params);
    let _cols = results.unwrap().get_inner_lanes(2);
}

//The test file is in scientific notation to test functions ability to parse floating point numbers
fn load_txt_f64_row_get_col_large_test() {
    let file = String::from("grainData_LOFEM.rods");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None, //Some(100000),//None,
        skip_footer: None,
        usecols: None,  //Some(cols),
        max_rows: None, //Some(100000),//None,
        row_format: false,
    };

    let results = load_txt_f64(&file, &params);
    let _cols = results.unwrap().get_col(2);
}
*/

//The test file is in scientific notation to test functions ability to parse floating point numbers
fn parser_txt_large_test() {
    let file = String::from("grainData_LOFEM.rods");

    let params = ReaderParams {
        comments: Some(b'%'),
        delimiter: Delimiter::WhiteSpace,
        skip_header: None, //Some(100000),//None,
        skip_footer: None,
        usecols: None,  //Some(cols),
        max_rows: None, //Some(100000),//None,
        ..Default::default()
    };

    let _results = parse_txt::<RawReaderResultsRows>(&file, &params);
}

fn test_wrapper(c: &mut Criterion) {
    let mut group = c.benchmark_group("Load_or_Parse_Sci_Txt");

    group.sample_size(10);
    group.bench_function("Load_txt_float", |b| {
        b.iter(load_txt_f64_large_test)
    });
    group.bench_function("Parse_txt_float", |b| {
        b.iter(parser_txt_large_test)
    });
    // group.bench_function("Load_txt_float_col", |b| {
    //     b.iter(|| load_txt_f64_col_large_test())
    // });
    // group.bench_function("Load_txt_float_col_get_col", |b| {
    //     b.iter(|| load_txt_f64_col_get_col_large_test())
    // });
    // group.bench_function("Load_txt_float_row_get_col", |b| {
    //     b.iter(|| load_txt_f64_row_get_col_large_test())
    // });
    group.finish();
}

criterion_group!(benches, test_wrapper);
criterion_main!(benches);
