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
    // const LINES: Vec<String> = vec![
    //   "To be, or not to be: that is the question: ".to_string(),
    //   "Whether 'tis nobler in the mind to suffer ".to_string(),
    //   "The slings and arrows of outrageous fortune, ".to_string(),
    //   "Or to take arms against a sea of troubles, ".to_string()
    // ];

    // // Apply Create, passing the list and the coder, to create the PCollection.
    // pipeline.apply(Create::from(LINES));    
    // [END create_pcollection]
}

fn main() {}
