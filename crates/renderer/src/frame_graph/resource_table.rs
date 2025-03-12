use std::{collections::HashMap, sync::Arc};

use crate::gfx_base::{AnyFGResource, Handle};

use super::FrameGraph;

#[derive(Default)]
pub struct ResourceTable {
    reads: HashMap<Handle, Arc<AnyFGResource>>,
    writes: HashMap<Handle, Arc<AnyFGResource>>,
}

pub fn extra_resource(
    graph: &FrameGraph,
    resource_handles: &[Handle],
    to: &mut HashMap<Handle, Arc<AnyFGResource>>,
) {
    for resource_handle in resource_handles.iter() {
        let resource_node = graph.get_resource_node(*resource_handle);

        if let Some(resource) = graph
            .get_resource(resource_node.virtual_resource_handle)
            .get_any_resource()
        {
            to.insert(*resource_handle, resource);
        }
    }
}
impl ResourceTable {
    pub fn extra(&mut self, graph: &FrameGraph, pass_node_handle: Handle) {
        let mut pass_node_handle = pass_node_handle;

        loop {
            let reads = graph.get_pass_node(pass_node_handle).reads.clone();
            extra_resource(graph, &reads, &mut self.reads);

            let writes = graph.get_pass_node(pass_node_handle).writes.clone();
            extra_resource(graph, &writes, &mut self.writes);

            let next = graph
                .get_pass_node(pass_node_handle)
                .next_pass_node_handle
                .is_some();

            if !next {
                break;
            } else {
                pass_node_handle = graph
                    .get_pass_node(pass_node_handle)
                    .next_pass_node_handle
                    .unwrap();
            }
        }
    }
}
