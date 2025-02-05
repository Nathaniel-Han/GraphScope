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

use crate::process::traversal::traverser::Traverser;
use pegasus::api::function::LeftJoinFunction;
use std::sync::Arc;

pub struct JoinFuncGen {
    func: Arc<dyn LeftJoinFunction<Traverser> + Sync>,
}

impl JoinFuncGen {
    pub fn new(func: Arc<dyn LeftJoinFunction<Traverser> + Sync>) -> Self {
        JoinFuncGen { func }
    }
}

impl JoinFuncGen {
    pub fn gen(&self) -> Box<dyn LeftJoinFunction<Traverser>> {
        let func = self.func.clone();
        Box::new(func)
    }
}

// for e.g., where(out().out().as("a"))
pub struct HasAnyJoin;

impl LeftJoinFunction<Traverser> for HasAnyJoin {
    fn exec(&self, parent: &Traverser, _sub: Traverser) -> Option<Traverser> {
        Some(parent.clone())
    }
}

// for e.g., order().by(out().out().count())
pub struct BySubJoin;

// TODO: throw error
impl LeftJoinFunction<Traverser> for BySubJoin {
    fn exec(&self, parent: &Traverser, sub: Traverser) -> Option<Traverser> {
        let mut parent = parent.clone();
        parent
            .get_element_mut()
            .expect("parent should be element")
            .attach(sub.get_object().expect("object").clone());
        Some(parent)
    }
}
