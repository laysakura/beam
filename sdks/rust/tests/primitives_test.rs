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

#[cfg(test)]
mod tests {
    use apache_beam::elem_types::kv::KV;
    use apache_beam::internals::pvalue::PValue;
    use apache_beam::runners::direct_runner::DirectRunner;
    use apache_beam::runners::runner::RunnerI;
    use apache_beam::transforms::create::Create;
    use apache_beam::transforms::flatten::Flatten;
    use apache_beam::transforms::group_by_key::GroupByKey;
    use apache_beam::transforms::impulse::Impulse;
    use apache_beam::transforms::pardo::ParDo;
    use apache_beam::transforms::testing::AssertEqualUnordered;

    #[tokio::test]
    async fn run_direct_runner() {
        DirectRunner::new()
            .run(|root| root.apply(Impulse::new()))
            .await;
    }

    #[tokio::test]
    #[should_panic]
    // This tests that AssertEqualUnordered is actually doing its job.
    async fn ensure_assert_fails() {
        DirectRunner::new()
            .run(|root| {
                root.apply(Create::new(vec![1, 2, 3]))
                    .apply(AssertEqualUnordered::new(&[1, 2, 4]))
            })
            .await;
    }

    #[tokio::test]
    #[should_panic]
    async fn ensure_assert_fails_on_empty() {
        DirectRunner::new()
            .run(|root| {
                root.apply(Create::new(vec![]))
                    .apply(AssertEqualUnordered::new(&[1]))
            })
            .await;
    }

    #[tokio::test]
    async fn run_map() {
        DirectRunner::new()
            .run(|root| {
                root.apply(Create::new(vec![1, 2, 3]))
                    .apply(ParDo::from_map(|x: &i32| -> i32 { x * x }))
                    .apply(AssertEqualUnordered::new(&[1, 4, 9]))
            })
            .await;
    }

    #[tokio::test]
    async fn run_gbk() {
        DirectRunner::new()
            .run(|root| {
                root.apply(Create::new(vec![
                    KV::new("a".to_string(), 1),
                    KV::new("a".to_string(), 2),
                    KV::new("b".to_string(), 3),
                ]))
                .apply(GroupByKey::default())
                .apply(AssertEqualUnordered::new(&[
                    KV::new("a".to_string(), vec![1, 2]),
                    KV::new("b".to_string(), vec![3]),
                ]))
            })
            .await;
    }

    #[tokio::test]
    async fn run_flatten() {
        DirectRunner::new()
            .run(|root| {
                let first = root.apply(Create::new(vec![1, 2, 3]));
                let second = root.apply(Create::new(vec![100, 200]));
                PValue::new_array(&[first, second])
                    .apply(Flatten::new())
                    .apply(AssertEqualUnordered::new(&[1, 2, 3, 100, 200]))
            })
            .await;
    }
}
