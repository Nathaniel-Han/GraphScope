//
//! Copyright 2020 Alibaba Group Holding Limited.
//! 
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//! 
//! http://www.apache.org/licenses/LICENSE-2.0
//! 
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use crate::errors::BuildJobError;
use crate::stream::Stream;
use crate::Data;

pub trait Multiplexing<D: Data> {
    fn scope_by<F>(&self, key: F) -> Result<Stream<D>, BuildJobError>
    where
        F: FnMut(&D) -> Option<u32> + Send + Clone + 'static;

    fn scope_by_size(&self, length: usize) -> Result<Stream<D>, BuildJobError>;
}

pub mod subtask;
