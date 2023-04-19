fn pipelines_constructing_creating() {
    // [START pipelines_constructing_creating]
    // let options = PipelineOptionsBuilder::build().unwrap();
    // let p = Pipeline::new(options);
    // [END pipelines_constructing_creating]
}

fn pipeline_options() {
    // [START pipeline_options]
    // TODO
    // [END pipeline_options]
}

fn pipeline_options_define_custom() {
    // [START pipeline_options_define_custom]
    // TODO
    // [END pipeline_options_define_custom]
}

fn pipelines_constructing_reading() {
    // // Create the pipeline.
    // let options =
    //     PipelineOptionsBuilder::from_args(...).build().unwrap();
    // let pipeline = Pipeline::new(options);

    // [START pipelines_constructing_reading]
    // // Create the PCollection 'lines' by applying a 'Read' transform.
    // let lines = pipeline.apply(
    //     TextIORead::new("gs://some/inputData.txt").unwrap());
    // [END pipelines_constructing_reading]
}

fn create_pcollection() {
    // // Create the pipeline.
    // let options =
    //     PipelineOptionsBuilder::from_args(...).build().unwrap();
    // let pipeline = Pipeline::new(options);

    // [START create_pcollection]
    let lines = vec![
        "To be, or not to be: that is the question: ".to_string(),
        "Whether 'tis nobler in the mind to suffer ".to_string(),
        "The slings and arrows of outrageous fortune, ".to_string(),
        "Or to take arms against a sea of troubles, ".to_string(),
    ];

    // // Apply Create, passing the list and the coder, to create the PCollection.
    // pipeline.apply(Create::from(lines));
    // [END create_pcollection]
}

fn model_pardo_dofn() {
    // // Create the pipeline.
    // let options =
    //     PipelineOptionsBuilder::from_args(...).build().unwrap();
    // let pipeline = Pipeline::new(options);

    // let words = pipeline.apply(Create::from(vec![
    //     "apple ".to_string(),
    //     "banana".to_string(),
    //     "cherry".to_string(),
    // ]));

    // [START model_pardo_dofn]
    // // The DoFn to perform on each element in the input PCollection.
    struct ComputeWordLengthFn;
    // impl DoFn for ComputeWordLengthFn {
    //     type In = String;
    //     type Out = usize;

    //     fn process(&self, word: &Self::In, out: mut OutputReceiver<Self::Out>) {
    //         out.output(word.len());
    //     }
    // }

    // // Apply a ParDo to the PCollection "words" to compute lengths for each word.
    // let word_lengths = words.apply(ParDo::new::<ComputeWordLengthFn>());
    // [END model_pardo_dofn]
}

fn model_pardo_closure() {
    // // Create the pipeline.
    // let options =
    //     PipelineOptionsBuilder::from_args(...).build().unwrap();
    // let pipeline = Pipeline::new(options);

    // let words = pipeline.apply(Create::from(vec![
    //     "apple ".to_string(),
    //     "banana".to_string(),
    //     "cherry".to_string(),
    // ]));

    // [START model_pardo_closure]
    // // Apply a ParDo to the PCollection "words" to compute lengths for each word.
    // let word_lengths = words.apply(ParDo::from_flatmap(|word| vec![word.len()]));
    // [END model_pardo_closure]
}

fn model_pardo_closure_map() {
    // // Create the pipeline.
    // let options =
    //     PipelineOptionsBuilder::from_args(...).build().unwrap();
    // let pipeline = Pipeline::new(options);

    // let words = pipeline.apply(Create::from(vec![
    //     "apple ".to_string(),
    //     "banana".to_string(),
    //     "cherry".to_string(),
    // ]));

    // [START model_pardo_closure_map]
    // // Apply a ParDo to the PCollection "words" to compute lengths for each word.
    // let word_lengths = words.apply(ParDo::from_map(|word| word.len()));
    // [END model_pardo_closure_map]
}

fn groupby() {
    // [START groupby]
    // // The input PCollection.
    // let mapped: PCollection<KV<String, String>> = ...;

    // // Apply GroupByKey to the PCollection mapped.
    // // Save the result as the PCollection reduced.
    // let reduced: PCollection<KV<String, Vec<String>>> =
    //     mapped.apply(GroupByKey::<String, String>::new());
    // [END groupby]
}

fn cogroupbykey() {
    // // Create the pipeline.
    // let options =
    //     PipelineOptionsBuilder::from_args(...).build().unwrap();
    // let pipeline = Pipeline::new(options);

    // [START cogroupbykey_inputs]
    // let emails_list = vec![
    //     KV::new("amy".to_string(), "amy@example.com".to_string()),
    //     KV::new("carl".to_string(), "carl@example.com".to_string()),
    //     KV::new("julia".to_string(), "julia@example.com".to_string()),
    //     KV::new("carl".to_string(), "carl@email.com".to_string()),
    // ];
    // let phones_list = vec![
    //     KV::new("amy".to_string(), "111-222-3333".to_string()),
    //     KV::new("james".to_string(), "222-333-4444".to_string()),
    //     KV::new("jamy".to_string(), "333-444-5555".to_string()),
    //     KV::new("carl".to_string(), "444-555-6666".to_string()),
    // ];

    // let emails = pipeline.apply(Create::from(emails_list));
    // let phones = pipeline.apply(Create::from(phones_list));
    // [END cogroupbykey_inputs]

    // [START cogroupbykey_raw_outputs]
    // let expected_results = vec![
    //     KV::new(
    //         "amy".to_string(),
    //         CoGbk2Result::new(
    //             vec!["amy@example.com".to_string()],
    //             vec!["111-222-3333".to_string(), "333-444-5555".to_string()],
    //         ),
    //     ),
    //     KV::new(
    //         "carl".to_string(),
    //         CoGbk2Result::new(
    //             vec!["carl@example.com", "carl@email.com"],
    //             vec!["444-555-6666"],
    //         ),
    //     ),
    //     KV::new(
    //         "james".to_string(),
    //         CoGbk2Result::new(vec![], vec!["222-333-4444"]),
    //     ),
    //     KV::new(
    //         "julia".to_string(),
    //         CoGbk2Result::new(vec!["julia@example.com"], vec![]),
    //     ),
    // ];
    // [END cogroupbykey_raw_outputs]

    // [START cogroupbykey]
    // let formatted_results_pcoll = (emails, phones)
    //     .apply(CoGroupByKey2::<String, String>::new())
    //     .apply(ParDo::from_map(|result| {
    //         let key = result.as_key();
    //         let emails = &result.as_value().0;
    //         let phones = &result.as_value().1;
    //         format!("{}; {:?}; {:?}", key, emails, phones)
    //     }));
    // [END cogroupbykey]

    // [START cogroupbykey_formatted_outputs]
    let expected_formatted_results = [
        "amy; [amy@example.com]; [111-222-3333,333-444-5555]",
        "carl; [carl@email.com,carl@example.com]; [444-555-6666]",
        "james; []; [222-333-4444]",
        "julia; [julia@example.com]; []",
    ];
    // [END cogroupbykey_formatted_outputs]

    // TODO asset formatted_results_pcoll == expected_formatted_results
}

fn combine_simple_sum() {
    // // Create the pipeline.
    // let options =
    //     PipelineOptionsBuilder::from_args(...).build().unwrap();
    // let pipeline = Pipeline::new(options);

    // [START combine_simple_sum]
    // let pcoll = pipeline.apply(Create::from(vec![1, 10, 100, 1000]));
    // let result = pcoll
    //     .apply(GroupGlobally::new())
    //     .apply(Combine::from_fold(0, |acc, x| acc + x));
    // [END combine_simple_sum]
}

fn combine_custom_average() {
    // [START combine_custom_average]
    struct AverageAccum {
        sum: i32,
        count: i32,
    }
    // impl ElemType for AverageAccum {}

    struct AverageFn;
    // impl CombineFn<i32, AverageAccum, f32> for AverageFn {
    //     fn create_accumulator(&self) -> AverageAccum {
    //         AverageAccum { sum: 0, count: 0 }
    //     }

    //     fn add_input(&self, acc: AverageAccum, input: i32) -> AverageAccum {
    //         AverageAccum {
    //             sum: acc.sum + input,
    //             count: acc.count + 1,
    //         }
    //     }

    //     fn merge_accumulators(&self, acc1: AverageAccum, acc2: AverageAccum) -> AverageAccum {
    //         AverageAccum {
    //             sum: acc1.sum + acc2.sum,
    //             count: acc1.count + acc2.count,
    //         }
    //     }

    //     fn extract_output(&self, acc: AverageAccum) -> f32 {
    //         acc.sum / acc.count
    //     }
    // }
    // [END combine_custom_average]

    // [START combine_global_average]
    // let pcoll = pipeline.apply(Create::new(vec![4, 5, 6]));
    // let result = pcoll
    //     .apply(GroupGlobally::new())
    //     .apply(Combine::<AverageFn>::new());
    // [END combine_global_average]
}

fn combine_per_key() {
    // // Create the pipeline.
    // let options =
    //     PipelineOptionsBuilder::from_args(...).build().unwrap();
    // let pipeline = Pipeline::new(options);

    // [START combine_per_key]
    // // PCollection is grouped by key and the f64 values associated with each key are combined into a f64.
    // let sales_records: PCollection<KV<String, f64>> = ...;
    // let total_sales_per_person: PCollection<KV<String, f64>> = sales_records
    //     .apply(GroupByKey::new())
    //     .apply(Combine::<SumFn::<f64, f64>>::new());

    // // The combined value is of a different type than the original collection of values per key. PCollection has
    // // keys of type String and values of type i32, and the combined value is a f64.
    // let player_accuracy: PCollection<KV<String, i32>> = ...;
    // let avg_accuracy_per_player: PCollection<KV<String, f64>> = player_accuracy
    //     .apply(Combine::<Mean::<i32, f64>>::new());
    // [END combine_per_key]
}

fn model_multiple_pcollections_flatten() {
    // // Create the pipeline.
    // let options =
    //     PipelineOptionsBuilder::from_args(...).build().unwrap();
    // let pipeline = Pipeline::new(options);

    // [START model_multiple_pcollections_flatten]
    // let fib = pipeline.apply(Create::new(vec![1, 1, 2, 3, 5, 8]));
    // let pow = pipeline.apply(Create::new(vec![1, 2, 4, 8, 16, 32]));
    // let result = (fib, pow).apply(Flatten::new());
    // [END model_multiple_pcollections_flatten]
}

fn main() {}
