use std::io;

use crate::{
    coders::{urns::UNIT_CODER_URN, Coder, CoderForPipeline, CoderUrn, Context},
    elem_types::ElemType,
};

/// Serialize/Deserialize a unit type.
#[derive(Debug, Default)]
pub struct UnitCoder;

impl CoderUrn for UnitCoder {
    const URN: &'static str = UNIT_CODER_URN;
}

impl Coder for UnitCoder {
    fn new(_component_coders: Vec<Box<dyn Coder>>) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn encode(
        &self,
        _element: &dyn ElemType,
        _writer: &mut dyn io::Write,
        _context: &Context,
    ) -> Result<usize, io::Error> {
        Ok(0)
    }

    fn decode(
        &self,
        _reader: &mut dyn io::Read,
        _context: &Context,
    ) -> Result<Box<dyn ElemType>, io::Error> {
        Ok(Box::new(()))
    }
}

impl CoderForPipeline for UnitCoder {
    fn component_coder_urns() -> Vec<crate::coders::CoderUrnTree> {
        vec![]
    }
}
