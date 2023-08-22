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

use std::str::FromStr;
use std::vec::*;

///A structure that contains all of the results in row major order. It tells us the number of fields we had
///along with the number of lines that we read. Finally, the results are stored in a single Vec of
///type T. Type T is what type one called load_txt_* for.
#[derive(Debug, Clone)]
pub struct ReaderResultsRow<T> 
where 
    T: FromStr + Clone,
{
    pub num_fields: usize,
    pub num_lines: usize,
    pub results: Vec<T>,
}

///A structure that contains all of the results in column major order. It tells us the number of fields we had
///along with the number of lines that we read. Finally, the results are stored in a single Vec of
///type T. Type T is what type one called load_txt_* for.
#[derive(Debug, Clone)]
pub struct ReaderResultsCol<T>
where 
    T: FromStr + Clone,
{
    pub num_fields: usize,
    pub num_lines: usize,
    pub results: Vec<T>,
}

pub trait ReaderResults<T> 
where 
    T: FromStr + Clone,
{
    /// Return the number of fields
    fn get_num_fields(&self) -> usize;
    /// Return the number of lines
    fn get_num_lines(&self) -> usize;
    /// Return a reference to the results 
    fn get_results(&self) -> &Vec<T>;
    /// Returns a mutable reference to the results
    fn get_mut_results(&mut self) -> &mut Vec<T>;
    /// Returns a slice to the desired lane
    /// Note a lane here is defined as the fastest iterating direction
    /// aka either row or column direction
    fn get_inner_lanes(&self, lane_index: usize) -> &[T];
    /// Returns a mutable slice to the desired lane
    /// Note a lane here is defined as the fastest iterating direction
    fn get_mut_inner_lane(&mut self, lane_index: usize) -> &mut [T];
    /// Returns the value at a given row and column index
    fn get_value(&self, row_index: usize, col_index: usize) -> T;
    /// Returns a copy of a desired row
    fn get_row(&self, row_index: usize) -> Vec<T>;
    /// Returns a copy of a desired rows
    fn get_rows(&self, row_indices: Vec<usize>) -> Vec<Vec<T>>;
    /// Returns a copy of a desired column
    fn get_col(&self, col_index: usize) -> Vec<T>;
    /// Returns a copy of a desired columns
    fn get_cols(&self, col_indices: Vec<usize>) -> Vec<Vec<T>>;

}

impl<T> ReaderResults<T> for ReaderResultsRow<T> 
where 
    T: FromStr + Clone,
{
    /// Return the number of fields
    fn get_num_fields(&self) -> usize {
        self.num_fields
    }
    /// Return the number of lines
    fn get_num_lines(&self) -> usize {
        self.num_lines
    }
    /// Return a reference to the results 
    fn get_results(&self) -> &Vec<T> {
        &self.results
    }
    /// Returns a mutable reference to the results
    fn get_mut_results(&mut self) -> &mut Vec<T> {
        &mut self.results
    }
    /// Returns a slice to the desired lane
    /// Note a lane here is defined as the fastest iterating direction
    /// aka either row or column direction
    fn get_inner_lanes(&self, lane_index: usize) -> &[T] {
        assert!(lane_index < self.num_lines);
        let start_index = lane_index * self.num_fields;
        let end_index = start_index + self.num_fields;
        &self.results[start_index .. end_index]
    }
    /// Returns a mutable slice to the desired lane
    /// Note a lane here is defined as the fastest iterating direction
    fn get_mut_inner_lane(&mut self, lane_index: usize) -> &mut [T] {
        assert!(lane_index < self.num_lines);
        let start_index = lane_index * self.num_fields;
        let end_index = start_index + self.num_fields;
        &mut self.results[start_index .. end_index]
    }

    ///Obtains a value for a given row and column where both indices inputted
    ///are 0 based.
    fn get_value(&self, row_index: usize, col_index: usize) -> T {
        assert!(row_index < self.num_lines);
        assert!(col_index < self.num_fields);
        self.results[row_index * self.num_fields + col_index].clone()
    }

    ///Returns a row given a valid index that is 0 based and less than the
    ///number of lines read in.
    fn get_row(&self, row_index: usize) -> Vec<T> {
        assert!(row_index < self.num_lines);

        let out: Vec<T> = self
            .results
            .iter()
            .skip(row_index * self.num_fields)
            .take(self.num_fields)
            .cloned()
            .collect();

        out
    }
    ///Returns a series of rows given a valid indices that are 0 based and less than the
    ///number of lines read in.
    fn get_rows(&self, row_indices: Vec<usize>) -> Vec<Vec<T>> {
        let mut out: Vec<Vec<T>> = Vec::new();

        for index in row_indices.iter() {
            out.push(self.get_row(*index));
        }

        out
    }
    ///Returns a given column given a valid index that is 0 based and less than the
    ///number of fields read in.
    fn get_col(&self, col_index: usize) -> Vec<T> {
        assert!(col_index < self.num_fields);
        //We should just be able to use a slice to obtaine the values we want
        let out: Vec<T> = self
            .results
            .iter()
            .skip(col_index)
            .step_by(self.num_fields)
            .cloned()
            .collect();

        out
    }
    ///Returns a series of rows given a valid indices that are 0 based and less than the
    ///number of lines read in.
    fn get_cols(&self, col_indices: Vec<usize>) -> Vec<Vec<T>> {
        let mut out: Vec<Vec<T>> = Vec::new();

        for index in col_indices.iter() {
            out.push(self.get_col(*index));
        }

        out
    }
}

impl<T> ReaderResults<T> for ReaderResultsCol<T> 
where 
    T: FromStr + Clone,
{
    /// Return the number of fields
    fn get_num_fields(&self) -> usize {
        self.num_fields
    }
    /// Return the number of lines
    fn get_num_lines(&self) -> usize {
        self.num_lines
    }
    /// Return a reference to the results 
    fn get_results(&self) -> &Vec<T> {
        &self.results
    }
    /// Returns a mutable reference to the results
    fn get_mut_results(&mut self) -> &mut Vec<T> {
        &mut self.results
    }
    /// Returns a slice to the desired lane
    /// Note a lane here is defined as the fastest iterating direction
    /// aka either row or column direction
    fn get_inner_lanes(&self, lane_index: usize) -> &[T] {
        assert!(lane_index < self.num_fields);
        let start_index = lane_index * self.num_lines;
        let end_index = start_index + self.num_lines;
        &self.results[start_index .. end_index]
    }
    /// Returns a mutable slice to the desired lane
    /// Note a lane here is defined as the fastest iterating direction
    fn get_mut_inner_lane(&mut self, lane_index: usize) -> &mut [T] {
        assert!(lane_index < self.num_lines);
        let start_index = lane_index * self.num_lines;
        let end_index = start_index + self.num_lines;
        &mut self.results[start_index .. end_index]
    }

    ///Obtains a value for a given row and column where both indices inputted
    ///are 0 based.
    fn get_value(&self, row_index: usize, col_index: usize) -> T {
        assert!(row_index < self.num_lines);
        assert!(col_index < self.num_fields);
        self.results[col_index * self.num_lines + row_index].clone()
    }

    ///Returns a row given a valid index that is 0 based and less than the
    ///number of lines read in.
    fn get_row(&self, row_index: usize) -> Vec<T> {
        assert!(row_index < self.num_lines);
        //We should just be able to use a slice to obtain the values we want
        let out: Vec<T> = self
            .results
            .iter()
            .skip(row_index)
            .step_by(self.num_lines)
            .cloned()
            .collect();

        out
    }
    ///Returns a series of rows given a valid indices that are 0 based and less than the
    ///number of lines read in.
    fn get_rows(&self, row_indices: Vec<usize>) -> Vec<Vec<T>> {
        let mut out: Vec<Vec<T>> = Vec::new();

        for index in row_indices.iter() {
            out.push(self.get_row(*index));
        }

        out
    }
    ///Returns a given column given a valid index that is 0 based and less than the
    ///number of fields read in.
    fn get_col(&self, col_index: usize) -> Vec<T> {
        assert!(col_index < self.num_fields);

        let out: Vec<T> = self
            .results
            .iter()
            .skip(col_index * self.num_lines)
            .take(self.num_lines)
            .cloned()
            .collect();

        out
    }
    ///Returns a series of rows given a valid indices that are 0 based and less than the
    ///number of lines read in.
    fn get_cols(&self, col_indices: Vec<usize>) -> Vec<Vec<T>> {
        let mut out: Vec<Vec<T>> = Vec::new();

        for index in col_indices.iter() {
            out.push(self.get_col(*index));
        }

        out
    }
}

///A structure that contains all of the raw results. It tells us the number of fields we had
///along with the number of lines that we read. Results contains all of the data that was read in
///from the file in its raw u8 format. The index field contains the starting index for each field
///that was read in.
pub struct RawReaderResultsRows {
    pub num_fields: usize,
    pub num_lines: usize,
    pub results: Vec<u8>,
    pub index: Vec<usize>,
}

///A structure that contains all of the raw results. It tells us the number of fields we had
///along with the number of lines that we read. Results contains all of the data that was read in
///from the file in its raw u8 format. The index field contains the starting index for each field
///that was read in.
pub struct RawReaderResultsCols {
    pub num_fields: usize,
    pub num_lines: usize,
    pub results: Vec<Vec<u8>>,
    pub index: Vec<Vec<usize>>,
}

pub trait RawReaderParse {
    fn new(field: usize, num_lines: usize) -> Self;
    fn get_num_lines(&self) -> usize;
    fn incr_num_lines(&mut self);
    fn set_num_lines(&mut self, num_lines: usize);
    fn set_results(&mut self, value: u8, field: usize);
    fn set_index(&mut self, field: usize);
}

impl RawReaderParse for RawReaderResultsRows {

    #[inline(always)]
    fn new(field: usize, num_lines: usize) -> Self {

        let mut rr = 
        RawReaderResultsRows {
            num_fields: field,
            num_lines: 0,
            results: Vec::<u8>::new(),
            index: Vec::<usize>::new(), 
        };
        rr.results.reserve(num_lines * field);
        rr.index.reserve(num_lines);

        rr
    }

    #[inline(always)]
    fn get_num_lines(&self) -> usize {
        self.num_lines
    }

    #[inline(always)]
    fn incr_num_lines(&mut self) {
        self.num_lines += 1;
    }

    #[inline(always)]
    fn set_num_lines(&mut self, num_lines: usize) {
        self.num_lines = num_lines;
    }

    #[inline(always)]
    fn set_results(&mut self, value: u8, _field: usize) {
        self.results.push(value);
    }

    #[inline(always)]
    fn set_index(&mut self, _field: usize) {
        self.index.push(self.results.len());
    }
}

impl RawReaderParse for RawReaderResultsCols {

    #[inline(always)]
    fn new(field: usize, num_lines: usize) -> Self {
        let mut rr = 
        RawReaderResultsCols {
            num_fields: field,
            num_lines: 0,
            results: Vec::<Vec<u8>>::new(),
            index: Vec::<Vec<usize>>::new(), 
        };

        for icol in 0..field {
            rr.results.push(Vec::<u8>::new());
            rr.results[icol].reserve(num_lines);
            rr.index.push(Vec::<usize>::new());
            rr.index[icol].reserve(num_lines);
        }

        rr
    }

    #[inline(always)]
    fn get_num_lines(&self) -> usize {
        self.num_lines
    }

    #[inline(always)]
    fn incr_num_lines(&mut self) {
        self.num_lines += 1;
    }

    #[inline(always)]
    fn set_num_lines(&mut self, num_lines: usize) {
        self.num_lines = num_lines;
    }

    #[inline(always)]
    fn set_results(&mut self, value: u8, field: usize) {
        assert!(field - 1 < self.num_fields);
        self.results[field - 1].push(value);
    }

    #[inline(always)]
    fn set_index(&mut self, field: usize) {
        assert!(field - 1 < self.num_fields);
        self.index[field - 1].push(self.results[field - 1].len());
    }
}