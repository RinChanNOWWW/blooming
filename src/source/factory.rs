// Copyright 2022 RinChanNOWWW.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::mikan::MikanSource;
use super::SourcePtr;
use crate::Config;

pub fn register(factory: &mut SourceFactory, config: &Config) {
    if let Some(config) = &config.mikan {
        factory.register(MikanSource::create(config))
    }
}

#[derive(Default)]
pub struct SourceFactory {
    sources: Vec<SourcePtr>,
}

impl SourceFactory {
    pub fn register(&mut self, source: SourcePtr) {
        self.sources.push(source);
    }

    pub fn sources(&self) -> &Vec<SourcePtr> {
        &self.sources
    }
}
