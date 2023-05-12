/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! These are the coders necessary for encoding the data types required by
//! the Apache Beam model. They provide standardized ways of encode data for
//! communication between the runner, the Beam workers, and the user's code.
//! For example for any aggregations the runner and the SDK need to agree on
//! the encoding of key-value pairs; so that the SDK will encode keys properly,
//! and the runner will be able to group elements of the
//! same key together.
//!
//! The formal specifications for these coders can be found in
//! model/pipeline/src/main/proto/beam_runner_api.proto

use std::fmt;
use std::io::{self, ErrorKind, Read, Write};

use integer_encoding::{VarInt, VarIntReader, VarIntWriter};
use serde::{Deserialize, Serialize};

use crate::coders::required_coders::BytesCoder;
use crate::coders::urns::*;
use crate::coders::{Coder, Context};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct StrUtf8Coder {}

// TODO: accept string references as well?
impl Coder for StrUtf8Coder {
    type E = String;

    fn get_coder_urn() -> &'static str {
        STR_UTF8_CODER_URN
    }

    fn encode(
        &self,
        element: String,
        writer: &mut dyn Write,
        context: &Context,
    ) -> Result<usize, io::Error> {
        let bytes = element.as_bytes().to_vec();
        let coder = BytesCoder::default();
        coder.encode(bytes, writer, context)
    }

    fn decode(&self, reader: &mut dyn Read, context: &Context) -> Result<String, io::Error> {
        let coder = BytesCoder::default();
        let bytes = coder.decode(reader, context)?;

        let res = String::from_utf8(bytes);

        //TODO: improve error handling
        match res {
            Ok(s) => Ok(s),
            Err(_) => Result::Err(io::Error::new(
                ErrorKind::Other,
                "Unable to convert bytes to string",
            )),
        }
    }

    fn to_proto(&self) -> crate::proto::beam_api::pipeline::Coder {
        todo!()
    }
}

impl fmt::Debug for StrUtf8Coder {
    fn fmt(&self, o: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        o.debug_struct("StrUtf8Coder")
            .field("urn", &Self::get_coder_urn())
            .finish()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VarIntCoder<N: fmt::Debug + VarInt> {
    _var_int_type: std::marker::PhantomData<N>,
}

// TODO: passes tests for -1 if it gets casted to u64 and encoded as such.
// Revisit this later
impl<N> Coder for VarIntCoder<N>
where
    N: fmt::Debug + VarInt,
{
    type E = N;

    fn get_coder_urn() -> &'static str {
        VARINT_CODER_URN
    }

    // TODO: try to adapt Coder such that the context arg is not mandatory
    fn encode(
        &self,
        element: N,
        mut writer: &mut dyn Write,
        _context: &Context,
    ) -> Result<usize, io::Error> {
        writer.write_varint(element)
    }

    fn decode(&self, mut reader: &mut dyn Read, _context: &Context) -> Result<N, io::Error> {
        reader.read_varint()
    }

    fn to_proto(&self) -> crate::proto::beam_api::pipeline::Coder {
        todo!()
    }
}

impl<N: fmt::Debug + VarInt> Default for VarIntCoder<N> {
    fn default() -> Self {
        Self {
            _var_int_type: std::marker::PhantomData,
        }
    }
}

impl<N: fmt::Debug + VarInt> fmt::Debug for VarIntCoder<N> {
    fn fmt(&self, o: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        o.debug_struct("VarIntCoder")
            .field("urn", &Self::get_coder_urn())
            .finish()
    }
}
