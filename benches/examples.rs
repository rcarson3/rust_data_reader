extern crate rust_data_reader;
use rust_data_reader::reader::*;

#[macro_use]
extern crate criterion;

use criterion::{Criterion, Benchmark};

//The test file is in scientific notation to test functions ability to parse floating point numbers
fn load_txt_f64_large_test(){
    let file = String::from("grainData_LOFEM.rods");

    //let cols: Vec<usize> = vec![0, 2];

    let params = ReaderParams{
        comments: b'%',
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,//Some(100000),//None,
        skip_footer: None,
        usecols: None,//Some(cols),
        max_rows: None//Some(100000),//None,
    };

    let _results = load_txt_f64(&file, &params);

    // let comp: Vec<f64> = vec![1.049575902500000102e-01, -1.954995277500000128e-01, 3.713439238749999816e-01, 1.049545937499999915e-01, -1.954939916250000298e-01, 3.713446139999999618e-01, 1.049745187499999954e-01, -1.954979653749999990e-01, 3.713611012499999919e-01];
    // let comp: Vec<f64> = vec![1.049575902500000102e-01, 3.713439238749999816e-01, 1.049545937499999915e-01, 3.713446139999999618e-01, 1.049745187499999954e-01, 3.713611012499999919e-01];
    // let ans = &results.unwrap().results[0..6];

    // assert_eq!(*ans, comp[..]);
}

//The test file is in scientific notation to test functions ability to parse floating point numbers
fn load_txt_f64_large_lossy_test(){
    let file = String::from("grainData_LOFEM.rods");

    //let cols: Vec<usize> = vec![0, 2];

    let params = ReaderParams{
        comments: b'%',
        delimiter: Delimiter::WhiteSpace,
        skip_header: None,//Some(100000),//None,
        skip_footer: None,
        usecols: None,//Some(cols),
        max_rows: None//Some(100000),//None,
    };

    let _results = load_txt_lossy_f64(&file, &params);

    // let comp: Vec<f64> = vec![1.049575902500000102e-01, -1.954995277500000128e-01, 3.713439238749999816e-01, 1.049545937499999915e-01, -1.954939916250000298e-01, 3.713446139999999618e-01, 1.049745187499999954e-01, -1.954979653749999990e-01, 3.713611012499999919e-01];
    // let comp: Vec<f64> = vec![1.049575902500000102e-01, 3.713439238749999816e-01, 1.049545937499999915e-01, 3.713446139999999618e-01, 1.049745187499999954e-01, 3.713611012499999919e-01];
    // let ans = &results.unwrap().results[0..6];

    // assert_eq!(*ans, comp[..]);
}

fn test_wrapper(c: &mut Criterion){

     c.bench("load_txt_float", Benchmark::new("load_txt_f64_large_txt", 
     |b| b.iter(|| load_txt_f64_large_test()))
    .sample_size(10)
    );

}

fn test_wrapper2(c: &mut Criterion){

     c.bench("load_txt_lossy_float", Benchmark::new("load_txt_lossy_f64_large_txt", 
     |b| b.iter(|| load_txt_f64_large_lossy_test()))
    .sample_size(10)
    );

}

criterion_group!(benches, test_wrapper, test_wrapper2);
criterion_main!(benches);