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

use crate::structure::element::{Element, Label, ID};
use crate::structure::property::DynDetails;
use crate::structure::Details;

#[derive(Clone)]
pub struct Edge {
    pub id: ID,
    pub src_id: ID,
    pub dst_id: ID,
    pub label: Option<Label>,
    src_label: Option<Label>,
    dst_label: Option<Label>,
    properties: DynDetails,
}

impl Element for Edge {
    fn id(&self) -> ID {
        self.id
    }

    fn label(&self) -> &Label {
        if let Some(ref s) = self.label {
            s
        } else {
            self.properties.get_label()
        }
    }

    fn details(&self) -> &DynDetails {
        &self.properties
    }
}

impl Edge {
    pub fn new(
        id: u128, label: Option<Label>, src: u128, dst: u128, properties: DynDetails,
    ) -> Self {
        Edge { id, label, src_id: src, dst_id: dst, src_label: None, dst_label: None, properties }
    }

    pub fn set_src_label(&mut self, label: Label) {
        self.src_label = Some(label);
    }

    pub fn set_dst_label(&mut self, label: Label) {
        self.dst_label = Some(label);
    }
}

// #[derive(Default)]
// pub struct EdgeBuilder {
//     id          : Option<u128>,
//     label       : Option<String>,
//     src_id      : Option<u128>,
//     src_label   : Option<String>,
//     dst_id      : Option<u128>,
//     dst_label   : Option<String>,
//     properties  : Option<DynProperties>
// }
//
// impl EdgeBuilder {
//     pub fn new() -> Self {
//         EdgeBuilder::default()
//     }
//
//     pub fn set_id(&mut self, id: u128) -> &mut Self {
//         self.id = Some(id);
//         self
//     }
//
//     pub fn set_label(&mut self, label: String) -> &mut Self {
//         self.label = Some(label);
//         self
//     }
//
//     pub fn set_src_id(&mut self, id: u128) -> &mut Self {
//         self.src_id = Some(id);
//         self
//     }
//
//     pub fn set_src_label(&mut self, label: String) -> &mut Self {
//         self.src_label = Some(label);
//         self
//     }
//
//     pub fn set_dst_id(&mut self, id: u128) -> &mut Self {
//         self.dst_id = Some(id);
//         self
//     }
//
//     pub fn set_dst_label(&mut self, label: String) -> &mut Self {
//         self.dst_label = Some(label);
//         self
//     }
//
//     pub fn set_properties(&mut self, p: DynProperties) -> &mut Self {
//         self.properties = Some(p);
//         self
//     }
//
//     pub fn build(self) -> Option<Edge> {
//         unimplemented!()
//     }
//
//
// }
