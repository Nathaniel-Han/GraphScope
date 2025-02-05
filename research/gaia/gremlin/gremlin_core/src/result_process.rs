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

use crate::generated::common as common_pb;
use crate::generated::protobuf as result_pb;
use crate::object::Primitives;
use crate::process::traversal::path::{PathItem, ResultPath};
use crate::process::traversal::step::ResultProperty;
use crate::process::traversal::traverser::{ShadeSync, Traverser};
use crate::structure::{Edge, GraphElement, Label, Vertex, VertexOrEdge};
use crate::Object;
use pegasus_server::factory::HashKey;

fn vertex_to_pb(v: &Vertex) -> result_pb::Vertex {
    result_pb::Vertex {
        id: v.id as i64,
        label: if let Some(label) = v.label.clone() {
            match label {
                Label::Str(s) => s,
                // TODO(longbin) should turn back to its actual string
                Label::Id(id) => id.to_string(),
            }
        } else {
            String::new()
        },
        properties: vec![],
    }
}
fn edge_to_pb(e: &Edge) -> result_pb::Edge {
    result_pb::Edge {
        id: e.id as i64,
        label: if let Some(label) = e.label.clone() {
            match label {
                Label::Str(s) => s,
                // TODO(longbin) should turn back to its actual string
                Label::Id(id) => id.to_string(),
            }
        } else {
            String::new()
        },
        src_id: e.src_id as i64,
        src_label: "".to_string(),
        dst_id: e.dst_id as i64,
        dst_label: "".to_string(),
        properties: vec![],
    }
}

fn element_to_pb(g: &GraphElement) -> result_pb::GraphElement {
    let inner = match g.get() {
        VertexOrEdge::V(v) => result_pb::graph_element::Inner::Vertex(vertex_to_pb(v)),
        VertexOrEdge::E(e) => result_pb::graph_element::Inner::Edge(edge_to_pb(e)),
    };
    result_pb::GraphElement { inner: Some(inner) }
}

fn path_to_pb(path: &ResultPath) -> result_pb::Path {
    let mut path_pb = vec![];
    for item in path.iter() {
        match item {
            PathItem::OnGraph(graph_element) => {
                path_pb.push(element_to_pb(graph_element));
            }
            PathItem::Detached(_) => unimplemented!(),
        }
    }
    result_pb::Path { path: path_pb }
}

fn property_to_pb(result_property: &ResultProperty) -> result_pb::TagProperties {
    let mut tag_props_pb = vec![];
    for (tag, props_key_value) in result_property.properties.iter() {
        let mut props_pb = vec![];
        for (key, value) in props_key_value {
            let pb_value = object_to_pb_value(value);
            let property = result_pb::Property { key: key.to_string(), value: Some(pb_value) };
            props_pb.push(property);
        }
        let tag_property = result_pb::TagProperty { tag: tag.to_string(), props: props_pb };
        tag_props_pb.push(tag_property);
    }
    result_pb::TagProperties { item: tag_props_pb }
}

fn object_to_pb_value(value: &Object) -> common_pb::Value {
    let item = match value {
        Object::Primitive(v) => {
            match v {
                Primitives::Byte(_) => {
                    // TODO: check
                    unimplemented!()
                }
                Primitives::Integer(v) => common_pb::value::Item::I32(*v),
                Primitives::Long(v) => common_pb::value::Item::I64(*v),
                Primitives::Float(v) => common_pb::value::Item::F64(*v),
            }
        }
        Object::String(s) => common_pb::value::Item::Str(s.clone()),
        Object::Blob(b) => common_pb::value::Item::Blob(b.to_vec()),
        Object::UnknownOwned(_) => unimplemented!(),
        Object::UnknownRef(_) => unimplemented!(),
    };
    common_pb::Value { item: Some(item) }
}

pub fn result_to_pb(data: Vec<Traverser>) -> result_pb::Result {
    let mut paths_encode = vec![];
    let mut elements_encode = vec![];
    let mut properties_encode = vec![];
    for t in data {
        if let Some(e) = t.get_element() {
            println!("element: {:?}", e);
            elements_encode.push(element_to_pb(e));
        } else if let Some(o) = t.get_object() {
            match o {
                Object::Primitive(p) => println!("object result {:?}", p),
                Object::String(s) => println!("object result {:?}", s),
                Object::Blob(b) => println!("object result {:?}", b),
                Object::UnknownOwned(x) => {
                    if let Some(p) = x.try_downcast_ref::<ResultPath>() {
                        println!("path: {:?}", p);
                        paths_encode.push(path_to_pb(p));
                    } else if let Some(result_prop) = x.try_downcast_ref::<ResultProperty>() {
                        println!("property: {:?}", result_prop);
                        properties_encode.push(property_to_pb(result_prop));
                    } else if let Some(result_prop) =
                        x.try_downcast_ref::<ShadeSync<(HashKey<Traverser>, u64)>>()
                    {
                        println!("group count result {:?}", result_prop);
                    } else if let Some(result_prop) = x.try_downcast_ref::<ShadeSync<u64>>() {
                        println!("count result {:?}", result_prop);
                    } else {
                        println!("object result {:?}", x);
                    }
                }
                _ => unreachable!(),
            }
        } else {
            println!("object result is none!");
        };
    }
    if !elements_encode.is_empty() {
        let elements = result_pb::GraphElementArray { item: elements_encode };
        result_pb::Result { inner: Some(result_pb::result::Inner::Elements(elements)) }
    } else if !paths_encode.is_empty() {
        let paths = result_pb::PathArray { item: paths_encode };
        result_pb::Result { inner: Some(result_pb::result::Inner::Paths(paths)) }
    } else {
        let properties = result_pb::TagPropertiesArray { item: properties_encode };
        result_pb::Result { inner: Some(result_pb::result::Inner::TagProperties(properties)) }
    }
}
