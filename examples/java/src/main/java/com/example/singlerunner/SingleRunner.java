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
package com.example.singlerunner;

import java.io.IOException;

import org.apache.beam.runners.direct.DirectOptions;
import org.apache.beam.sdk.Pipeline;
import org.apache.beam.sdk.PipelineResult;
import org.apache.beam.sdk.PipelineRunner;
import org.apache.beam.sdk.metrics.MetricResults;
import org.apache.beam.sdk.options.PipelineOptions;
import org.apache.commons.lang3.NotImplementedException;
import org.joda.time.Duration;

/** Copied from DirectRunner. */
public class SingleRunner extends PipelineRunner<SingleRunner.SinglePipelineResult> {

  /** Construct a {@link SingleRunner} from the provided options. */
  public static SingleRunner fromOptions(PipelineOptions options) {
    return new SingleRunner();
  }

  private SingleRunner() {
  }

  @Override
  public SinglePipelineResult run(Pipeline pipeline) {
    return new SinglePipelineResult();
  }



  public static class SinglePipelineResult implements PipelineResult {

    @Override
    public State getState() {
      return State.RUNNING;
    }

    @Override
    public State cancel() throws IOException {
      throw new NotImplementedException("");
    }

    @Override
    public State waitUntilFinish(Duration duration) {
      throw new NotImplementedException("");
    }

    @Override
    public State waitUntilFinish() {
      return State.DONE;
    }

    @Override
    public MetricResults metrics() {
      throw new NotImplementedException("");
    }
  }
}
