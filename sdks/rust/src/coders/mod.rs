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

pub mod coder_resolver;
pub mod required_coders;
pub mod rust_coders;
pub mod standard_coders;
pub mod urns;

use crate::proto::beam_api::pipeline as proto_pipeline;
use std::fmt;
use std::io::{self, Read, Write};

/// This is the base interface for coders, which are responsible in Apache Beam to encode and decode
/// elements of a PCollection.
///
/// # Example
///
/// ```
/// use apache_beam::coders::{Coder, standard_coders::StrUtf8Coder, Context};
/// use bytes::buf::BufMut;
/// use std::io::Write;
///
/// let coder = StrUtf8Coder::default();
///
/// let mut w1 = vec![].writer();
/// coder
///     .encode("my string".to_string(), &mut w1, &Context::WholeStream)
///     .unwrap();
/// w1.flush().unwrap();
/// println!("{:?}", w1.into_inner()); // <= Prints the pure byte-encoding of the string
///
/// let mut w2 = vec![].writer();
/// coder
///     .encode("my string".to_string(), &mut w2, &Context::NeedsDelimiters)
///     .unwrap();
/// w2.flush().unwrap();
/// println!("{:?}", w2.into_inner()); // <= Prints a length-prefix string of bytes
/// ```
///
/// # Coders' lifecycle
///
/// Coders, including their associated element types, are statically defined in the SDK (here we call them preset coders) or SDK users' code (custom coders).
/// They are serialized in proto and sent to the runner, and then to the SDK harness.
/// Then the SDK harness deserializes a proto coder with an `Any` associated element type and instantiate it.
///
/// Preset coders and custom coders have a bit different lifecycle, in that the latter needs to be serialized with the encode/decode functions.
///
/// ## Preset (required, standard, etc...) coders
///
/// 1. A preset coder is defined in the SDK.
/// 2. The coder is instantiated by an SDK user's code on pipeline construction time, along with its coder ID.
/// 3. The coder's URN and its ID are serialized. Note that an instance of the coder is not serialized for preset coders.
/// 4. The serialized coder and its ID are sent to the SDK harness via Runner API -> Fn API.
/// 5. The SDK harness receives the serialized coder's URN and its ID from Fn API.
/// 6. The SDK harness deserializes the coder's URN and creates an instance of the coder specified by the URN.
/// 7. The SDK harness stores the coder instance in a map, with the coder ID as the key.
///
/// ## Custom coders
///
/// 1. A custom coder is defined by an SDK user.
/// 2. The coder is instantiated by an SDK user's code on pipeline construction time, along with its coder ID.
/// 3. The coder's trait object (including the encode/decode functions) and its ID are serialized.
/// 4. The serialized coder and its ID are sent to the SDK harness via Runner API -> Fn API.
/// 5. The SDK harness receives the serialized coder's trait object and its ID from Fn API.
/// 6. The SDK harness deserializes the coder's trait object coder's to get an instance.
/// 7. The SDK harness stores the coder instance in a map, with the coder ID as the key.
pub trait Coder: fmt::Debug + Default {
    /// The type of the elements to be encoded/decoded.
    type E;

    fn get_coder_urn() -> &'static str
    where
        Self: Sized;

    /// Encode an element into a stream of bytes
    ///
    /// # Arguments
    ///
    /// - `element` - an element within a PCollection
    /// - `writer` - a writer that interfaces the coder with the output byte stream
    /// - `context` - the context within which the element should be encoded
    fn encode(
        &self,
        element: Self::E,
        writer: &mut dyn Write,
        context: &Context,
    ) -> Result<usize, io::Error>;

    /// Decode an element from an incoming stream of bytes
    ///
    /// # Arguments
    ///
    /// - `reader` - a reader that interfaces the coder with the input byte stream
    /// - `context` - the context within which the element should be encoded
    fn decode(&self, reader: &mut dyn Read, context: &Context) -> Result<Self::E, io::Error>;

    /// Convert this coder into its protocol buffer representation for the Runner API.
    /// A coder in protobuf format can be shared with other components such as Beam runners,
    /// SDK workers; and reconstructed into its runtime representation if necessary.
    fn to_proto(&self) -> proto_pipeline::Coder;
}

/// The context for encoding a PCollection element.
/// For example, for strings of utf8 characters or bytes, `WholeStream` encoding means
/// that the string will be encoded as-is; while `NeedsDelimiter` encoding means that the
/// string will be encoded prefixed with its length.
pub enum Context {
    /// Whole stream encoding/decoding means that the encoding/decoding function does not need to worry about
    /// delimiting the start and end of the current element in the stream of bytes.
    WholeStream,

    /// Needs-delimiters encoding means that the encoding of data must be such that when decoding,
    /// the coder is able to stop decoding data at the end of the current element.
    NeedsDelimiters,
}
