use std::collections::HashSet;

use bevy::prelude::*;

use crate::{LogicalCoordinates, WorldTileDimensions};

#[derive(Clone)]
pub struct NodeData {
    location: LogicalCoordinates,
}

impl NodeData {
    pub fn new(location: LogicalCoordinates) -> Self {
        Self { location }
    }

    pub fn get_location(&self) -> &LogicalCoordinates {
        &self.location
    }
}

pub struct WorldNode {
    id: usize,
    data: NodeData,
}

impl WorldNode {
    pub fn new(id: usize, data: NodeData) -> Self {
        Self { id, data }
    }

    pub fn get_data(&self) -> &NodeData {
        &self.data
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

pub struct AdjacencyList {
    edges: Vec<Vec<usize>>,
}

impl AdjacencyList {
    pub fn from_tile_nodes(nodes: &Vec<WorldNode>, world_size: &WorldTileDimensions) -> Self {
        let mut edges = vec![Vec::new(); nodes.len()];

        for node in nodes {
            let current_node_location = node.get_data().get_location();
            let current_node_id = node.get_id();
            if current_node_location.get_x() > 0 {
                let left_node_position = LogicalCoordinates::new(
                    current_node_location.get_x() - 1,
                    current_node_location.get_y(),
                );
                let left_node_id = left_node_position.to_1d(world_size);

                let left_node_found_in_nodes = nodes
                    .binary_search_by(|node| node.get_id().cmp(&left_node_id))
                    .is_ok();
                if !left_node_found_in_nodes {
                    continue;
                }

                edges[current_node_id].push(left_node_id);
                edges[left_node_id].push(current_node_id);
            }

            if current_node_location.get_y() > 0 {
                let top_node_position = LogicalCoordinates::new(
                    current_node_location.get_x(),
                    current_node_location.get_y() - 1,
                );
                let top_node_id = top_node_position.to_1d(world_size);

                let top_node_found_in_nodes = nodes
                    .binary_search_by(|node| node.get_id().cmp(&top_node_id))
                    .is_ok();
                if !top_node_found_in_nodes {
                    continue;
                }

                edges[current_node_id].push(top_node_id);
                edges[top_node_id].push(current_node_id);
            }
        }

        Self { edges }
    }

    pub fn get_edges(&self, node_id: usize) -> &Vec<usize> {
        &self.edges[node_id]
    }
}

#[derive(Component)]
pub struct Graph {
    world_size: WorldTileDimensions,
    nodes: Vec<WorldNode>,
    edges: AdjacencyList,
}

impl Graph {
    pub fn from_tiles(tiles: &Vec<LogicalCoordinates>, world_size: &WorldTileDimensions) -> Self {
        let mut nodes = Vec::new();

        for tile_location in tiles {
            let node_data = NodeData::new(*tile_location);
            let node_id = tile_location.to_1d(world_size);
            let tile_node = WorldNode::new(node_id, node_data);
            nodes.push(tile_node);
        }
        nodes.sort_by(|node1, node2| node1.get_id().cmp(&node2.get_id()));

        let edges = AdjacencyList::from_tile_nodes(&nodes, &world_size);

        Self {
            world_size: world_size.clone(),
            nodes,
            edges,
        }
    }

    pub fn get_node_at(&self, position: &LogicalCoordinates) -> &WorldNode {
        let position_1d = position.to_1d(&self.world_size);

        self.nodes
            .iter()
            .find(|node| node.get_id() == position_1d)
            .expect("get_node_at: Could not find node")
    }

    pub fn get_edges(&self, node: &WorldNode) -> Vec<&WorldNode> {
        let mut node_edges = Vec::new();

        let found_node_edges = self.edges.get_edges(node.get_id());

        for node_id in found_node_edges {
            node_edges.push(&self.nodes[*node_id]);
        }

        node_edges
    }
}

#[derive(Component)]
pub struct Pathfinding {
    path: Vec<NodeData>,
}

impl Pathfinding {
    pub fn shortest_path(
        source: &LogicalCoordinates,
        destination: &LogicalCoordinates,
        world_graph: &Graph,
    ) -> Self {
        let mut nodes_to_visit: Vec<&WorldNode> = Vec::new();
        let source_node = world_graph.get_node_at(source);
        nodes_to_visit.push(&source_node);

        let mut path = Vec::new();
        let mut visited_nodes = HashSet::new();
        while let Some(node_to_visit) = nodes_to_visit.pop() {
            if visited_nodes.contains(&node_to_visit.get_id()) {
                continue;
            }

            let node_data = node_to_visit.get_data();
            path.push(node_data.clone());
            visited_nodes.insert(node_to_visit.get_id());

            if node_data.get_location() == destination {
                break;
            }

            let node_edges = world_graph.get_edges(node_to_visit);
            for next_node in node_edges {
                nodes_to_visit.push(next_node);
            }
        }

        Self { path }
    }

    pub fn get_destination(&self) -> &LogicalCoordinates {
        self.path
            .last()
            .expect("get_destination: Path is empty.")
            .get_location()
    }
}
