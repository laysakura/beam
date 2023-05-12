use std::any::Any;

use apache_beam::coders::{Coder, Context};
use serde::{Deserialize, Serialize};

#[test]
fn serde_custom_coder() {
    use serde_traitobject as s;

    #[derive(Clone, PartialEq, Eq, Debug)]
    struct MyElement {
        some_field: String,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    struct MyCoder {
        some_state: String,
    }

    impl Coder for MyCoder {
        type E = MyElement;

        fn get_coder_urn() -> &'static str
        where
            Self: Sized,
        {
            "unused"
        }

        fn encode(
            &self,
            element: Self::E,
            writer: &mut dyn std::io::Write,
            _context: &apache_beam::coders::Context,
        ) -> Result<usize, std::io::Error> {
            writer
                .write_all(format!("ENCPREFIX{}", element.some_field).as_bytes())
                .map(|_| 0) // TODO make Result<usize, std::io::Error> to Result<(), std::io::Error>
        }

        fn decode(
            &self,
            reader: &mut dyn std::io::Read,
            _context: &apache_beam::coders::Context,
        ) -> Result<Self::E, std::io::Error> {
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;

            let encoded_element = String::from_utf8(buf).unwrap();
            let element = encoded_element.strip_prefix("ENCPREFIX").unwrap();
            Ok(MyElement {
                some_field: element.to_string(),
            })
        }

        fn to_proto(&self) -> apache_beam::proto::beam_api::pipeline::Coder {
            let erased: s::Box<dyn s::Any> = s::Box::new(self.clone());
            let serialized = serde_json::to_string(&erased).unwrap();

            let spec = apache_beam::proto::beam_api::pipeline::FunctionSpec {
                urn: Self::get_coder_urn().to_string(),
                payload: serialized.into_bytes(),
            };

            apache_beam::proto::beam_api::pipeline::Coder {
                spec: Some(spec),
                component_coder_ids: vec![],
            }
        }
    }

    let my_coder = MyCoder {
        some_state: "state1".to_string(),
    };

    let proto_coder = my_coder.to_proto();
    // serialize `proto_coder` into binary format, send to runners and then to SDK harness, then deserialize back to `proto_coder` agin.

    let payload = proto_coder.spec.unwrap().payload;
    let payload_string = String::from_utf8(payload).unwrap();
    let deserialized_my_coder_any: s::Box<dyn s::Any> =
        serde_json::from_str(&payload_string).unwrap();
    // We might need URN even for custom coders to specify `MyStruct` here.
    let deserialized_my_coder: Box<MyCoder> =
        Box::<dyn Any>::downcast(deserialized_my_coder_any.into_any()).unwrap();

    assert_eq!(deserialized_my_coder.some_state, "state1".to_string());

    let element = MyElement {
        some_field: "some_value".to_string(),
    };
    let mut encoded_element = vec![];
    deserialized_my_coder
        .encode(element.clone(), &mut encoded_element, &Context::WholeStream)
        .unwrap();
    let decoded_element = deserialized_my_coder
        .decode(&mut encoded_element.as_slice(), &Context::WholeStream)
        .unwrap();

    assert_eq!(decoded_element, element);
}
