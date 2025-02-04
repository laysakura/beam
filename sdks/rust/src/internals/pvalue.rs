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

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::coders::urns::{ITERABLE_CODER_URN, UNIT_CODER_URN};
use crate::coders::{CoderForPipeline, CoderUrnTree};
use crate::elem_types::{DefaultCoder, ElemType};
use crate::proto::pipeline_v1;

use crate::internals::pipeline::Pipeline;

/// The base object on which one can start building a Beam DAG.
/// Generally followed by a source-like transform such as a read or impulse.
pub(crate) type Root = PValue<()>;

pub struct PValue<E>
where
    E: ElemType,
{
    id: String,
    ptype: PType,
    pipeline: Arc<Pipeline>,
    coder_urn: String,

    phantom: PhantomData<E>,
}

impl<E> PValue<E>
where
    E: ElemType,
{
    pub(crate) fn new(
        ptype: PType,
        pipeline: Arc<Pipeline>,
        id: String,
        coder_urn: String,
    ) -> Self {
        Self {
            id,
            ptype,
            pipeline,
            coder_urn,

            phantom: PhantomData::default(),
        }
    }

    pub(crate) fn root() -> Root {
        let pipeline = Arc::new(Pipeline::default());
        PValue::new(
            PType::Root,
            pipeline,
            crate::internals::utils::get_bad_id(),
            UNIT_CODER_URN.to_string(),
        )
    }

    pub fn new_array(pcolls: &[PValue<E>]) -> Self {
        PValue::new(
            PType::PValueArr,
            pcolls[0].pipeline.clone(),
            pcolls
                .iter()
                .map(|pcoll| -> String { pcoll.id.clone() })
                .collect::<Vec<String>>()
                .join(","),
            ITERABLE_CODER_URN.to_string(),
        )
    }

    pub(crate) fn get_pipeline_arc(&self) -> Arc<Pipeline> {
        self.pipeline.clone()
    }

    /// Returns the coder URN for this `PValue`.
    pub fn coder_urn(&self) -> &str {
        &self.coder_urn
    }

    pub fn get_type(&self) -> &PType {
        &self.ptype
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn apply<F, Out>(&self, transform: F) -> PValue<Out>
    where
        Out: ElemType + DefaultCoder + Clone,
        F: PTransform<E, Out> + Send,
    {
        self.pipeline.apply_transform(
            transform,
            self,
            self.pipeline.clone(),
            &Out::default_coder_urn(),
        )
    }

    pub fn apply_with_coder<OverrideCoder, F, Out>(&self, transform: F) -> PValue<Out>
    where
        OverrideCoder: CoderForPipeline,
        Out: ElemType + Clone,
        F: PTransform<E, Out> + Send,
    {
        self.pipeline.apply_transform(
            transform,
            self,
            self.pipeline.clone(),
            &OverrideCoder::coder_urn_tree(),
        )
    }

    // pub fn map(&self, callable: impl Fn() -> PValue) -> PValue {
    //     todo!()
    // }
}

/// Returns a PValue as a flat object with string keys and PCollection id values.
///
/// The full set of PCollections reachable by this PValue will be returned,
/// with keys corresponding roughly to the path taken to get there
pub fn flatten_pvalue<E>(pvalue: &PValue<E>, prefix: Option<String>) -> HashMap<String, String>
where
    E: ElemType,
{
    let mut result: HashMap<String, String> = HashMap::new();
    match pvalue.ptype {
        PType::PCollection => match prefix {
            Some(pr) => {
                result.insert(pr, pvalue.get_id());
            }
            None => {
                result.insert("main".to_string(), pvalue.get_id());
            }
        },
        PType::PValueArr => {
            // TODO: Remove this hack, PValues can have multiple ids.
            for (i, id) in pvalue.get_id().split(',').enumerate() {
                result.insert(i.to_string(), id.to_string());
            }
        }
        PType::PValueMap => todo!(),
        PType::Root => {}
    }

    result
}

// Anonymous sum types would probably be better, if/when they become
// available. https://github.com/rust-lang/rfcs/issues/294
// TODO: use strum discriminants on PValues instead of this
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PType {
    Root,
    PCollection,
    PValueArr,
    PValueMap,
}

// TODO: move this to transforms directory
pub trait PTransform<In, Out>
where
    In: ElemType,
    Out: ElemType + Clone,
{
    fn expand_internal(
        &self,
        input: &PValue<In>,
        _pipeline: Arc<Pipeline>,
        _out_coder_urn: &CoderUrnTree,
        _transform_proto: &mut pipeline_v1::PTransform,
    ) -> PValue<Out>
    where
        Self: Sized;

    fn expand(
        &self,
        input: &PValue<In>,
        pipeline: Arc<Pipeline>,
        out_coder_urn: &CoderUrnTree,
        transform_proto: &mut pipeline_v1::PTransform,
    ) -> PValue<Out>
    where
        Self: Sized,
    {
        let mut pvalue = self.expand_internal(input, pipeline, out_coder_urn, transform_proto);
        pvalue.coder_urn = out_coder_urn.coder_urn.to_owned();
        pvalue
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        internals::pvalue::{PType, PValue},
        transforms::impulse::Impulse,
    };

    #[tokio::test]
    async fn run_impulse_expansion() {
        let root = PValue::<()>::root();

        let pcoll = root.apply(Impulse::new());

        // TODO: test proto coders
        // let pipeline_proto = runner.pipeline.proto.lock().unwrap();
        // let proto_coders = pipeline_proto.components.unwrap().coders;
        // let coder = *proto_coders
        //     .get(&root_clone.pcoll_proto.coder_id)
        //     .unwrap();

        assert_eq!(*pcoll.get_type(), PType::PCollection);
    }
}
