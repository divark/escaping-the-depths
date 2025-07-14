use std::collections::{HashSet, VecDeque};

use bevy::prelude::*;

use crate::{ExitDoorState, LogicalCoordinates, WorldTileDimensions};

use super::{MovementTime, room_generating::ExplorerState};

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
    path: VecDeque<NodeData>,
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

        let mut path = VecDeque::new();
        let mut visited_nodes = HashSet::new();
        while let Some(node_to_visit) = nodes_to_visit.pop() {
            if visited_nodes.contains(&node_to_visit.get_id()) {
                continue;
            }

            let node_data = node_to_visit.get_data();
            path.push_back(node_data.clone());
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
            .iter()
            .last()
            .expect("get_destination: Path is empty.")
            .get_location()
    }

    pub fn is_traveling(&self) -> bool {
        !self.path.is_empty()
    }

    pub fn pop_front(&mut self) -> NodeData {
        self.path.pop_front().expect("pop_front: No nodes found.")
    }

    pub fn get_locations(&self) -> Vec<LogicalCoordinates> {
        self.path
            .iter()
            .map(|path_node| *path_node.get_location())
            .collect()
    }
}

#[derive(Component)]
pub struct PathTarget {
    movement_timer: Timer,

    logical_target: LogicalCoordinates,
    physical_target: Transform,

    current_position: Transform,
}

impl PathTarget {
    pub fn new(
        logical_target: LogicalCoordinates,
        physical_target: Transform,
        current_position: Transform,
        movement_timer: Timer,
    ) -> Self {
        Self {
            movement_timer,

            logical_target,
            physical_target,

            current_position,
        }
    }

    fn compute_expected_position(&self, target: f32) -> f32 {
        if self.movement_timer.finished() {
            return target;
        }

        let tile_difference = 16.0;
        target - (tile_difference - self.movement_timer.fraction())
    }

    pub fn advance(&mut self, time: &Res<Time>) -> Transform {
        self.movement_timer.tick(time.delta());

        let advanced_x = self.compute_expected_position(self.physical_target.translation.x);
        let advanced_y = self.compute_expected_position(self.physical_target.translation.y);
        let existing_z = self.current_position.translation.z;

        let current_position = Transform::from_xyz(advanced_x, advanced_y, existing_z);
        self.current_position = current_position;

        self.current_position
    }

    pub fn has_been_reached(&self) -> bool {
        let at_x = self.current_position.translation.x == self.physical_target.translation.x;
        let at_y = self.current_position.translation.y == self.physical_target.translation.y;

        at_x && at_y
    }

    pub fn get_logical_target(&self) -> LogicalCoordinates {
        self.logical_target
    }
}

pub fn make_explorer_go_to_exit_door(
    exit_door: Query<(&ExitDoorState, &LogicalCoordinates), Changed<ExitDoorState>>,
    mut explorer: Query<(Entity, &LogicalCoordinates, &mut ExplorerState), Without<Pathfinding>>,
    room_traversal_graph: Query<&Graph>,
    mut commands: Commands,
) {
    if exit_door.is_empty() || explorer.is_empty() || room_traversal_graph.is_empty() {
        return;
    }

    let (explorer_entity, explorer_location, mut explorer_state) = explorer
        .single_mut()
        .expect("make_explorer_go_to_exit_door: Could not find explorer.");
    let (exit_door_state, exit_door_location) = exit_door
        .single()
        .expect("make_explorer_go_to_exit_door: Could not get exit door.");
    let room_graph = room_traversal_graph
        .single()
        .expect("make_explorer_go_to_exit_door: Could not get room graph");

    if *exit_door_state == ExitDoorState::Closed {
        return;
    }

    let explorer_path =
        Pathfinding::shortest_path(explorer_location, exit_door_location, room_graph);
    commands.entity(explorer_entity).insert(explorer_path);

    *explorer_state = ExplorerState::Traveling;
}

pub fn set_explorer_target(
    mut explorer: Query<
        (Entity, &Transform, &mut Pathfinding, &mut ExplorerState),
        Without<PathTarget>,
    >,
    tiles: Query<(&LogicalCoordinates, &Transform)>,
    movement_time: Res<MovementTime>,
    mut commands: Commands,
) {
    if explorer.is_empty() {
        return;
    }

    let (explorer_entity, explorer_position, mut explorer_path, mut explorer_state) = explorer
        .single_mut()
        .expect("set_explorer_target: Could not find explorer.");

    if !explorer_path.is_traveling() {
        commands.entity(explorer_entity).remove::<Pathfinding>();
        *explorer_state = ExplorerState::Alive;
        return;
    }

    let explorer_logical_target = *explorer_path.pop_front().get_location();
    let tile_position = *tiles
        .iter()
        .find(|tile| *tile.0 == explorer_logical_target)
        .map(|tile| tile.1)
        .expect("set_explorer_target: Could not find tile's transform.");
    let explorer_target = PathTarget::new(
        explorer_logical_target,
        tile_position,
        *explorer_position,
        movement_time.get_timer(),
    );
    commands.entity(explorer_entity).insert(explorer_target);
}

pub fn move_explorer_to_next_tile(
    mut explorer: Query<
        (
            Entity,
            &mut Transform,
            &mut LogicalCoordinates,
            &mut PathTarget,
        ),
        With<ExplorerState>,
    >,
    time: Res<Time>,
    mut commands: Commands,
) {
    if explorer.is_empty() {
        return;
    }

    let (
        explorer_entity,
        mut explorer_position,
        mut explorer_logical_position,
        mut explorer_target,
    ) = explorer
        .single_mut()
        .expect("move_explorer: Could not find explorer.");

    let new_explorer_position = explorer_target.advance(&time);
    *explorer_position = new_explorer_position;

    if explorer_target.has_been_reached() {
        commands.entity(explorer_entity).remove::<PathTarget>();
        *explorer_logical_position = explorer_target.get_logical_target();
    }
}
