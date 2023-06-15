<!--
    Licensed to the Apache Software Foundation (ASF) under one
    or more contributor license agreements.  See the NOTICE file
    distributed with this work for additional information
    regarding copyright ownership.  The ASF licenses this file
    to you under the Apache License, Version 2.0 (the
    "License"); you may not use this file except in compliance
    with the License.  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing,
    software distributed under the License is distributed on an
    "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
    KIND, either express or implied.  See the License for the
    specific language governing permissions and limitations
    under the License.
-->

# Beam Rust SDK (experimental)

This repo is emerged form the [Rust SDK issue in apache/beam repo](https://github.com/apache/beam/issues/21089).

Anyone who is interested in the development of the Beam Rust SDK, you can freely join [discord](https://discord.gg/DVbKujUsyp).

Reading (and even writing) [design docs](https://github.com/laysakura/beam/wiki/Design-docs) would also be a good first step to get ready for contributions.

## API
TODO

## Development

### Approach

The Rust SDK leans heavily on the Typescript SDK as a reference given its modern design compared to the other SDKs as well as the fact that part of its purpose was to serve as a better reference for future SDKs, as [stated by the developers](https://www.youtube.com/watch?v=uQ7_eTXNB8M&t=7s&ab_channel=ApacheBeam). The [current implementation](https://github.com/apache/beam/tree/master/sdks/typescript) is the main reference for the Rust SDK. The initial commits for the Typescript SDK ([1](https://github.com/apache/beam/pull/17005), [2](https://github.com/apache/beam/pull/17341)) have also been useful to help guide the early stages of the Rust SDK.

### Current state and roadmap

We are currently in a very early stage, with a minimally functional end-to-end execution for the simplest possible pipeline. Much work still remains in the following areas:

- User-facing API: a minimal structure is in place just to kick things off, but it still lacks some basic definitions and will be revisited a bit later into the implementation.
- Transforms: the pipelines can use a minimally functional Impulse transform to initiate execution. An initial map + ParDo implementation would be the next logical step. Additional features such as windowing and side inputs are still a bit far into the future.
- Coders: we currently have a testable early version of BytesCoder. New coders will be added gradually as required by new transforms.
- Worker: the workers are currently able to perform basic operator construction and bundle processing using mock operators. The first production operators will be added soon.
- Runners: an early async DirectRunner implementation is in place to ensure that all modules succesfully run in an end-to-end execution within an asynchronous context. Work on production runners will begin when a few additional transforms (and their pre-requisites) have been implemented.

#### Additional TODOs

There are many other tasks remaining, but the core of the above modules will be prioritized for now. The current priority is to enable and test a larger number of features at a basic level, so many shortcuts have been taken and need to be revisited in the future:

- Make the SDK more idiomatic for Rust, instead of naively mirroring the Typescript SDK.
- Refactor to stop the widespread usage of inneficient cloning done to simplify things with the borrow checker early on.
- Revisit function signatures and make input parameters less reliant on owned types when possible.
- Add robust logging.
- Implement proper error handling.
- Create Docker container for SDK execution.
- Increase test coverage.
- Verify and improve performance overall.
- Organize module structure.
- Use macros to avoid some of the boilerplate code.
- Incorporate linting and testing into the build.
- Add documentation in code and later on in user guides.
- Include pipeline examples.

### Contributions

Any contributions are more than welcome, regardless of size. Early code reviews are also greatly appreciated, even if they are very localized. Feel free to get in touch with me to discuss, suggest anything or coordinate any changes: nivaldo.humbertoo@gmail.com.

### References

- [Rust SDK Github issue](https://github.com/apache/beam/issues/21089)
- [General Rust guidelines](https://rust-lang.github.io/api-guidelines/about.html)
- [Current Typescript SDK implementation](https://github.com/apache/beam/tree/master/sdks/typescript)
- [Typescript SDK presentation](https://www.youtube.com/watch?v=uQ7_eTXNB8M&t=7s&ab_channel=ApacheBeam)
- Initial commits for the Typescript SDK ([1](https://github.com/apache/beam/pull/17005), [2](https://github.com/apache/beam/pull/17341))
- [Writing a Beam SDK](https://www.youtube.com/watch?v=VsGQ2LFeTHY&t=806s&ab_channel=ApacheBeam)
- [Go SDK overview](https://www.youtube.com/watch?v=WcuS8ojHfyU&t=3s&ab_channel=ApacheBeam)
- [Beam design docs](https://cwiki.apache.org/confluence/display/BEAM/Design+Documents)
- [PTransform style guide](https://beam.apache.org/contribute/ptransform-style-guide/)

### Getting started

#### Setup

Install rustup and execute the commands from `sdks/rust`.

Install clippy

```
rustup component add clippy
```

Install rustfmt

```
rustup component add rustfmt
```

I also recommend using VS Code with rust-analyzer enabled (including some of its optional features such as lifetime elision display).

#### Build

Prepare the environment and build the Rust project:

```
./build.sh
```

#### Test

At the moment, unit tests are contained in the lib file for each module and integration tests can be found in sdks/rust/src/tests.

The following command (also available in the build script) can be used to run the available tests:

```
cargo test -- --skip target/debug
```

#### Style

rustfmt can be used in dry runs to verify what needs to be changed:

```
cargo fmt --all -- --check
```

The changes can be applied automatically through the following command:

```
cargo fmt --all
```

#### Protos

The auto-generated proto files with the Beam API can be found after compilation at:
`rust/target/debug/build/apache-beam-[hash]/out`
