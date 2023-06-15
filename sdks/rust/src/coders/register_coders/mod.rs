/// Macro to register custom coders in order for SDK harnesses to use them via their URNs.
///
/// Must be called in outside of the main() function.
///
/// # Example
///
/// ```ignore
/// struct MyCustomCoder1;
/// impl Coder MyCustomCoder1 { /* ... */ }
///
/// // ...
///
/// register_coders!(MyCustomCoder1, MyCustomCoder2);
///
/// fn main() {
///    // ...
/// }
/// ```
///
/// # Related doc
///
/// [Design doc: Custom Coders for the Beam Rust SDK](https://docs.google.com/document/d/1tUb8EoajRkxLW3mrJZzx6xxGhoiUSRKwVuT2uxjAeIU/edit#heading=h.mgr8mrx81tnc)
///
/// # Note
///
/// `register_coders!` must not be called twice or more in the whole test cases.
/// It relies on OnceCell and each call to `register_coders!` will try to overwrite the OnceCell, which leads to panic.
#[macro_export]
macro_rules! register_coders {
    ($($coder:ident),*) => {
        $(
            impl $crate::coders::CoderUrn for $coder {
                const URN: &'static str = concat!("beam:coder:rustsdk:1.0:", stringify!($coder));
            }
        )*

        fn custom_coder_from_urn(urn: &str, component_coders: Vec<Box<dyn $crate::coders::Coder>>) -> Option<Box<dyn $crate::coders::Coder>> {
            use $crate::coders::CoderUrn;

            match urn {
                $($coder::URN => Some(Box::new(<$coder>::new(component_coders))),)*
                _ => panic!("unknown urn: {}", urn),
            }
        }

        #[ctor::ctor]
        fn init_custom_coder_from_urn() {
            $crate::worker::CUSTOM_CODER_FROM_URN.set($crate::worker::CustomCoderFromUrn {
                func: Some(custom_coder_from_urn),
            }).expect("CUSTOM_CODER_FROM_URN singleton is already initialized");
        }
    }
}

pub(crate) type CustomCoderFromUrnFn =
    fn(&str, Vec<Box<dyn crate::coders::Coder>>) -> Option<Box<dyn crate::coders::Coder>>;
