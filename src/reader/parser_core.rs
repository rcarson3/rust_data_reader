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

use super::*;

use memchr::Memchr2;

pub(crate) trait ReadCore: std::io::Read + std::io::BufRead + std::io::Seek {}
impl<T: std::io::Read + std::io::BufRead + std::io::Seek> ReadCore for T {}

pub(crate) struct CoreData<'a, RRP>
where
    RRP: RawReaderParse,
{
    pub length: usize,
    pub offset: usize,
    pub cmt: u8,
    pub delim_ws: bool,
    pub delim: u8,
    pub fln: usize,
    pub cols: &'a Vec::<usize>,
    pub field_counter: usize,
    pub current_field: usize,
    pub tot_fields: usize,
    pub results: &'a mut RRP,
}

pub(crate) trait Parser
{
    fn next<RRP: RawReaderParse>(&self, buffer: &[u8], newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error>;
    fn parse_delim<RRP: RawReaderParse>(&self, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error>;
    fn parse_whitespace<RRP: RawReaderParse>(&self, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error>;
    fn parse_newline<RRP: RawReaderParse>(&self, newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error>;
    fn parse_comment<RRP: RawReaderParse>(&self, newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error>;
    fn parse_others<RRP: RawReaderParse>(&self, buf_val: u8, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error>;
}

pub(crate) struct NwLine {}
pub(crate) struct Delim {}
pub(crate) struct Space {}
pub(crate) struct Field {}
pub(crate) struct SkField {}

pub(crate) enum ParserState {
    NwLine(NwLine),
    Delim(Delim),
    Space(Space),
    Field(Field),
    SkField(SkField),
}

impl ParserState {
    #[inline(always)]
    pub(crate) fn next<RRP: RawReaderParse>(&self, buffer: &[u8], newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        match self {
            ParserState::NwLine(ps) => ps.next(buffer, newline, core_data),
            ParserState::Delim(ps) => ps.next(buffer, newline, core_data),
            ParserState::Space(ps) => ps.next(buffer, newline, core_data),
            ParserState::Field(ps) => ps.next(buffer, newline, core_data),
            ParserState::SkField(ps) => ps.next(buffer, newline, core_data),
        }
    }    
}

impl Parser for NwLine {
    #[inline(always)]
    fn next<RRP: RawReaderParse>(&self, buffer: &[u8], newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let buf_val = buffer[core_data.offset];
        if (buf_val == core_data.delim) & !core_data.delim_ws
        {
            self.parse_delim(core_data)
        }
        else if (buf_val == b' ') | (buf_val == b'\t') {
            return self.parse_whitespace(core_data);
        }
        else if (buf_val == b'\n') | (buf_val == b'\r') | (buf_val == core_data.cmt) {
            return self.parse_newline(newline, core_data);
        } else {
            return self.parse_others(buf_val, core_data);
        }
    }

    #[inline(always)]
    fn parse_delim<RRP: RawReaderParse>(&self, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        core_data.field_counter = 1;
        core_data.offset += 1;
        Ok(ParserState::Delim(Delim{}))
    }

    #[inline(always)]
    fn parse_whitespace<RRP: RawReaderParse>(&self, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        if core_data.delim_ws {
            core_data.field_counter = 1;
            core_data.offset += 1;
            Ok(ParserState::Delim(Delim{}))
        }
        else {
            core_data.offset += 1;
            Ok(ParserState::Space(Space{}))
        }
    }

    #[inline(always)]
    fn parse_newline<RRP: RawReaderParse>(&self, newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let val = newline.next();
        core_data.offset = match val {
            Some(val) => val + 1,
            None => core_data.length,
        };
        core_data.fln += 1;
        Ok(ParserState::NwLine(NwLine{}))
    }

    #[inline(always)]
    fn parse_comment<RRP: RawReaderParse>(&self, newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        self.parse_newline(newline, core_data)
    }

    #[inline(always)]
    fn parse_others<RRP: RawReaderParse>(&self, buf_val: u8, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        core_data.field_counter = 1;
        core_data.offset += 1;
        match &core_data.cols.len() {
            0 => {
                core_data.current_field = 1;
                core_data.results.set_results(buf_val, core_data.field_counter);
                Ok(ParserState::Field(Field{}))
            }
            _ => {
                let pos = core_data.cols.iter().position(|&x| x == core_data.field_counter);
                match pos {
                    Some(x) => {
                        core_data.current_field = x + 1;
                        core_data.results.set_results(buf_val, core_data.current_field);
                        Ok(ParserState::Field(Field{}))
                    }
                    None => {
                        Ok(ParserState::SkField(SkField{}))
                    }
                }
            }
        }
    }
}

impl Parser for Delim {
    #[inline(always)]
    fn next<RRP: RawReaderParse>(&self, buffer: &[u8], newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let buf_val = buffer[core_data.offset];
        if (buf_val == core_data.delim) & !core_data.delim_ws
        {
            self.parse_delim(core_data)
        }
        else if (buf_val == b' ') | (buf_val == b'\t') {
            return self.parse_whitespace(core_data);
        }
        else if (buf_val == b'\n') | (buf_val == b'\r') {
            return self.parse_newline(newline, core_data);
        } 
        else if buf_val == core_data.cmt {
            return self.parse_comment(newline, core_data);
        }
        else {
            return self.parse_others(buf_val, core_data);
        }
    }

    #[inline(always)]
    fn parse_delim<RRP: RawReaderParse>(&self, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        core_data.offset += 1;
        Ok(ParserState::Delim(Delim{}))
    }

    #[inline(always)]
    fn parse_whitespace<RRP: RawReaderParse>(&self, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        if core_data.delim_ws {
            core_data.offset += 1;
            Ok(ParserState::Delim(Delim{}))
        }
        else {
            core_data.offset += 1;
            Ok(ParserState::Space(Space{}))
        }
    }

    #[inline(always)]
    fn parse_newline<RRP: RawReaderParse>(&self, newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let val = newline.next();
        core_data.offset = match val {
            Some(val) => val + 1,
            None => core_data.length,
        };
        core_data.fln += 1;
    
        if core_data.delim_ws {
            core_data.field_counter -= 1;
            if (core_data.field_counter != core_data.tot_fields) & (core_data.field_counter != 0) {
                return Err(format_err!(
                    "Newline (delim) Number of fields,{}, provided at line {} 
                    is different than the initial field number of {}",
                    core_data.field_counter,
                    core_data.fln,
                    core_data.tot_fields
                ));
            }
            core_data.field_counter = 0;
            core_data.results.incr_num_lines();
        } else {
            return Err(format_err!(
                "Number of fields provided at line {} 
                ends with a delimiter instead of a field or white space",
                core_data.fln
            ));
        }
        Ok(ParserState::NwLine(NwLine{}))
    }

    #[inline(always)]
    fn parse_comment<RRP: RawReaderParse>(&self, newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let val = newline.next();
        core_data.offset = match val {
            Some(val) => val + 1,
            None => core_data.length,
        };
        core_data.fln += 1;

        if core_data.delim_ws {
            core_data.field_counter -= 1;
            if (core_data.field_counter != core_data.tot_fields) & (core_data.field_counter != 0) {
                return Err(format_err!(
                    "Cmt (delim) Number of fields,{}, provided at line {} 
                    is different than the initial field number of {}",
                    core_data.field_counter,
                    core_data.fln,
                    core_data.tot_fields
                ));
            }
            if core_data.field_counter > 0 {
                core_data.field_counter = 0;
                core_data.results.incr_num_lines();
            }
        } else {
            return Err(format_err!(
                "Number of fields provided at line {} 
                ends with a delimiter instead of a field or white space",
                core_data.fln
            ));
        }
        Ok(ParserState::NwLine(NwLine{}))
    }

    #[inline(always)]
    fn parse_others<RRP: RawReaderParse>(&self, buf_val: u8, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        core_data.offset += 1;
        match &core_data.cols.len() {
            0 => {
                core_data.current_field = core_data.field_counter;
                core_data.results.set_results(buf_val, core_data.field_counter);
                Ok(ParserState::Field(Field{}))
            }
            _ => {
                let pos = core_data.cols.iter().position(|&x| x == core_data.field_counter);
                match pos {
                    Some(x) => {
                        core_data.current_field = x + 1;
                        core_data.results.set_results(buf_val, core_data.current_field);
                        Ok(ParserState::Field(Field{}))
                    }
                    None => {
                        Ok(ParserState::SkField(SkField{}))
                    }
                }
            }
        }
    }
}

impl Parser for Space {
    #[inline(always)]
    fn next<RRP: RawReaderParse>(&self, buffer: &[u8], newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let buf_val = buffer[core_data.offset];
        if (buf_val == core_data.delim) & !core_data.delim_ws
        {
            self.parse_delim(core_data)
        }
        else if (buf_val == b' ') | (buf_val == b'\t') {
            return self.parse_whitespace(core_data);
        }
        else if (buf_val == b'\n') | (buf_val == b'\r') {
            return self.parse_newline(newline, core_data);
        } 
        else if buf_val == core_data.cmt {
            return self.parse_comment(newline, core_data);
        }
        else {
            return self.parse_others(buf_val, core_data);
        }
    }

    #[inline(always)]
    fn parse_delim<RRP: RawReaderParse>(&self, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        core_data.field_counter += 1;
        core_data.offset += 1;
        Ok(ParserState::Delim(Delim{}))
    }

    #[inline(always)]
    fn parse_whitespace<RRP: RawReaderParse>(&self, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        if core_data.delim_ws {
            core_data.field_counter += 1;
            core_data.offset += 1;
            Ok(ParserState::Delim(Delim{}))
        }
        else {
            core_data.offset += 1;
            Ok(ParserState::Space(Space{}))
        }
    }

    #[inline(always)]
    fn parse_newline<RRP: RawReaderParse>(&self, newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let val = newline.next();
        core_data.offset = match val {
            Some(val) => val + 1,
            None => core_data.length,
        };
        core_data.fln += 1;
        Ok(ParserState::NwLine(NwLine{}))
    }

    #[inline(always)]
    fn parse_comment<RRP: RawReaderParse>(&self, newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let val = newline.next();
        core_data.offset = match val {
            Some(val) => val + 1,
            None => core_data.length,
        };
        core_data.fln += 1;
        Ok(ParserState::NwLine(NwLine{}))
    }

    #[inline(always)]
    fn parse_others<RRP: RawReaderParse>(&self, buf_val: u8, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        core_data.offset += 1;
        //The case where we start out with spaces before our 1st field at the start of a line
        if core_data.field_counter == 0 {
            core_data.field_counter += 1;
        }
        match &core_data.cols.len() {
            0 => {
                core_data.current_field = core_data.field_counter;
                core_data.results.set_results(buf_val, core_data.field_counter);
                Ok(ParserState::Field(Field{}))
            }
            _ => {
                let pos = core_data.cols.iter().position(|&x| x == core_data.field_counter);
                match pos {
                    Some(x) => {
                        core_data.current_field = x + 1;
                        core_data.results.set_results(buf_val, core_data.current_field);
                        Ok(ParserState::Field(Field{}))
                    }
                    None => {
                        Ok(ParserState::SkField(SkField{}))
                    }
                }
            }
        }
    }
}

impl Parser for Field {
    #[inline(always)]
    fn next<RRP: RawReaderParse>(&self, buffer: &[u8], newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let buf_val = buffer[core_data.offset];
        if (buf_val == core_data.delim) & !core_data.delim_ws
        {
            self.parse_delim(core_data)
        }
        else if (buf_val == b' ') | (buf_val == b'\t') {
            return self.parse_whitespace(core_data);
        }
        else if (buf_val == b'\n') | (buf_val == b'\r') {
            return self.parse_newline(newline, core_data);
        } 
        else if buf_val == core_data.cmt {
            return self.parse_comment(newline, core_data);
        }
        else {
            return self.parse_others(buf_val, core_data);
        }
    }

    #[inline(always)]
    fn parse_delim<RRP: RawReaderParse>(&self, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        core_data.field_counter += 1;
        core_data.offset += 1;
        core_data.results.set_index(core_data.current_field);
        Ok(ParserState::Delim(Delim{}))
    }

    #[inline(always)]
    fn parse_whitespace<RRP: RawReaderParse>(&self, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        if core_data.delim_ws {
            core_data.field_counter += 1;
            core_data.results.set_index(core_data.current_field);
            core_data.offset += 1;
            Ok(ParserState::Delim(Delim{}))
        }
        else {
            core_data.offset += 1;
            Ok(ParserState::Field(Field{}))
        }
    }

    #[inline(always)]
    fn parse_newline<RRP: RawReaderParse>(&self, newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let val = newline.next();
        core_data.offset = match val {
            Some(val) => val + 1,
            None => core_data.length,
        };
        core_data.fln += 1;
        core_data.results.set_index(core_data.current_field);
        if core_data.field_counter != core_data.tot_fields {
            return Err(format_err!(
                "Newline (field) Number of fields,{}, provided at line {} 
                is different than the initial field number of {}",
                core_data.field_counter,
                core_data.fln,
                core_data.tot_fields
            ));
        }
        core_data.results.incr_num_lines();
        core_data.field_counter = 0;
        Ok(ParserState::NwLine(NwLine{}))
    }

    #[inline(always)]
    fn parse_comment<RRP: RawReaderParse>(&self, newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let val = newline.next();
        core_data.offset = match val {
            Some(val) => val + 1,
            None => core_data.length,
        };
        core_data.fln += 1;
        core_data.results.set_index(core_data.current_field);
        if core_data.field_counter != core_data.tot_fields {
            return Err(format_err!(
                "Cmt (field) Number of fields,{}, provided at line {} 
                is different than the initial field number of {}",
                core_data.field_counter,
                core_data.fln,
                core_data.tot_fields
            ));
        }
        core_data.results.incr_num_lines();
        core_data.field_counter = 0;
        Ok(ParserState::NwLine(NwLine{}))
    }

    #[inline(always)]
    fn parse_others<RRP: RawReaderParse>(&self, buf_val: u8, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        core_data.offset += 1;
        core_data.results.set_results(buf_val, core_data.current_field);
        Ok(ParserState::Field(Field{}))
    }


}

impl Parser for SkField {
    #[inline(always)]
    fn next<RRP: RawReaderParse>(&self, buffer: &[u8], newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let buf_val = buffer[core_data.offset];
        if (buf_val == core_data.delim) & !core_data.delim_ws
        {
            self.parse_delim(core_data)
        }
        else if (buf_val == b' ') | (buf_val == b'\t') {
            return self.parse_whitespace(core_data);
        }
        else if (buf_val == b'\n') | (buf_val == b'\r') {
            return self.parse_newline(newline, core_data);
        } 
        else if buf_val == core_data.cmt {
            return self.parse_comment(newline, core_data);
        }
        else {
            return self.parse_others(buf_val, core_data);
        }
    }

    #[inline(always)]
    fn parse_delim<RRP: RawReaderParse>(&self, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        core_data.field_counter += 1;
        core_data.offset += 1;
        Ok(ParserState::Delim(Delim{}))
    }

    #[inline(always)]
    fn parse_whitespace<RRP: RawReaderParse>(&self, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        if core_data.delim_ws {
            core_data.offset += 1;
            core_data.field_counter += 1;
            Ok(ParserState::Delim(Delim{}))
        }
        else {
            core_data.offset += 1;
            Ok(ParserState::SkField(SkField{}))
        }
    }

    #[inline(always)]
    fn parse_newline<RRP: RawReaderParse>(&self, newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let val = newline.next();
        core_data.offset = match val {
            Some(val) => val + 1,
            None => core_data.length,
        };
        core_data.fln += 1;
        if core_data.field_counter != core_data.tot_fields {
            return Err(format_err!(
                "Newline (skip field) Number of fields,{}, provided at line {} 
                is different than the initial field number of {}",
                core_data.field_counter,
                core_data.fln,
                core_data.tot_fields
            ));
        }
        core_data.results.incr_num_lines();
        core_data.field_counter = 0;
        Ok(ParserState::NwLine(NwLine{}))
    }

    #[inline(always)]
    fn parse_comment<RRP: RawReaderParse>(&self, newline: &mut Memchr2, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        let val = newline.next();
        core_data.offset = match val {
            Some(val) => val + 1,
            None => core_data.length,
        };
        core_data.fln += 1;
        if core_data.field_counter != core_data.tot_fields {
            return Err(format_err!(
                "Cmt (skip field) Number of fields,{}, provided at line {} 
                is different than the initial field number of {}",
                core_data.field_counter,
                core_data.fln,
                core_data.tot_fields
            ));
        }
        core_data.results.incr_num_lines();
        core_data.field_counter = 0;
        Ok(ParserState::NwLine(NwLine{}))
    }

    #[inline(always)]
    fn parse_others<RRP: RawReaderParse>(&self, _buf_val: u8, core_data: &mut CoreData<RRP>) -> Result<ParserState, Error> {
        core_data.offset += 1;
        Ok(ParserState::SkField(SkField{}))
    }
}

