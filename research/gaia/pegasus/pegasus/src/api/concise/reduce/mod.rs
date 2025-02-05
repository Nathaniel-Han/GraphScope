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

use pegasus_common::downcast::*;

pub mod accum;
pub mod barrier;
pub mod count;
pub mod group;
pub mod limit;
pub mod order;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Range {
    Local,
    Global,
}

impl_as_any!(Range);
pub const RANGES: [Range; 2] = [Range::Local, Range::Global];

pub use barrier::Barrier;
pub use count::Count;
pub use group::Group;
pub use limit::Limit;
pub use order::{Order, OrderBy, OrderDirect};
