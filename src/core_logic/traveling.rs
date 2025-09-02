use std::collections::{HashMap, HashSet, VecDeque};

use bevy::prelude::*;

use super::{
    MovementTime,
    setting::{
        ExplorerState, HiddenFloorSwitch, LogicalCoordinates, RoomObject, WorldTileDimensions,
    },
};

#[derive(Clone, Copy, Debug, Component, PartialEq)]
pub enum ExitDoorState {
    Closed,
    Open,
}

#[derive(Clone)]
pub struct NodeData {
    location: LogicalCoordinates,
    tile_type: RoomObject,
}

impl NodeData {
    pub fn new(location: LogicalCoordinates, tile_type: RoomObject) -> Self {
        Self {
            location,
            tile_type,
        }
    }

    pub fn get_location(&self) -> &LogicalCoordinates {
        &self.location
    }

    pub fn get_type(&self) -> &RoomObject {
        &self.tile_type
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
    edges: HashMap<usize, Vec<usize>>,
}

fn insert_edge(parent_node: usize, child_node: usize, edges: &mut HashMap<usize, Vec<usize>>) {
    edges.entry(parent_node).or_default();

    edges
        .get_mut(&parent_node)
        .expect("insert_edge: Could not find the edge for the given parent node")
        .push(child_node);
}

impl AdjacencyList {
    pub fn from_tile_nodes(nodes: &Vec<WorldNode>, world_size: &WorldTileDimensions) -> Self {
        let mut edges = HashMap::new();

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
                if left_node_found_in_nodes {
                    insert_edge(current_node_id, left_node_id, &mut edges);
                    insert_edge(left_node_id, current_node_id, &mut edges);
                }
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
                if top_node_found_in_nodes {
                    insert_edge(current_node_id, top_node_id, &mut edges);
                    insert_edge(top_node_id, current_node_id, &mut edges);
                }
            }
        }

        Self { edges }
    }

    pub fn get_edges(&self, node_id: usize) -> &Vec<usize> {
        &self.edges[&node_id]
    }
}

#[derive(Component)]
pub struct Graph {
    world_size: WorldTileDimensions,
    nodes: Vec<WorldNode>,
    edges: AdjacencyList,
}

impl Graph {
    pub fn from_tiles(
        tiles: &Vec<(LogicalCoordinates, RoomObject)>,
        world_size: &WorldTileDimensions,
    ) -> Self {
        let mut nodes = Vec::new();

        for (tile_location, tile_type) in tiles {
            if tile_type == &RoomObject::Wall {
                continue;
            }

            let node_data = NodeData::new(*tile_location, *tile_type);
            let node_id = tile_location.to_1d(world_size);
            let tile_node = WorldNode::new(node_id, node_data);
            nodes.push(tile_node);
        }
        nodes.sort_by_key(|node1| node1.get_id());

        let edges = AdjacencyList::from_tile_nodes(&nodes, world_size);

        Self {
            world_size: *world_size,
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

    pub fn get_node_by_id(&self, desired_node_id: usize) -> &WorldNode {
        self.nodes
            .iter()
            .find(|node| node.get_id() == desired_node_id)
            .expect("get_node_by_id: Could not find node.")
    }

    pub fn get_edges(&self, node: &WorldNode) -> Vec<&WorldNode> {
        let mut node_edges = Vec::new();

        let found_node_edges = self.edges.get_edges(node.get_id());

        for node_id in found_node_edges {
            let found_node = self.get_node_by_id(*node_id);
            node_edges.push(found_node);
        }

        node_edges
    }
}

#[derive(Component)]
pub struct Pathfinding {
    path: VecDeque<NodeData>,
}

fn get_bfs_path(
    source_node: &WorldNode,
    target_node: &WorldNode,
    discovered_by: &HashMap<usize, usize>,
    world_graph: &Graph,
) -> VecDeque<NodeData> {
    let mut path = VecDeque::new();
    path.push_front(target_node.get_data().clone());

    let mut current_node_id = target_node.get_id();
    while let Some(parent_node_id) = discovered_by.get(&current_node_id) {
        if current_node_id == source_node.get_id() {
            break;
        }

        let parent_node = world_graph.get_node_by_id(*parent_node_id);
        let parent_node_data = parent_node.get_data().clone();
        path.push_front(parent_node_data);

        current_node_id = parent_node.get_id();
    }

    path
}

fn visit_dfs_recursive(
    current_node: &WorldNode,
    path: &mut VecDeque<NodeData>,
    visited_nodes: &mut HashSet<usize>,
    world_graph: &Graph,
) {
    let current_node_id = current_node.get_id();
    if visited_nodes.contains(&current_node_id) {
        return;
    }

    let current_node_data = current_node.get_data();
    path.push_back(current_node_data.clone());
    visited_nodes.insert(current_node_id);

    let current_node_edges = world_graph.get_edges(current_node);
    for next_node in current_node_edges {
        visit_dfs_recursive(next_node, path, visited_nodes, world_graph);
        path.push_back(current_node_data.clone());
    }
}

impl Pathfinding {
    pub fn explore_all(source: &LogicalCoordinates, world_graph: &Graph) -> Self {
        let source_node = world_graph.get_node_at(source);

        let mut path = VecDeque::new();
        let mut visited_nodes = HashSet::new();

        visit_dfs_recursive(source_node, &mut path, &mut visited_nodes, world_graph);

        Self { path }
    }

    pub fn shortest_path(
        source: &LogicalCoordinates,
        destination: &LogicalCoordinates,
        world_graph: &Graph,
    ) -> Self {
        let mut nodes_to_visit: VecDeque<&WorldNode> = VecDeque::new();
        let source_node = world_graph.get_node_at(source);
        nodes_to_visit.push_back(source_node);

        let mut visited_nodes = HashSet::new();
        let mut discovered_by = HashMap::new();
        let mut found_target_node = None;
        while let Some(node_to_visit) = nodes_to_visit.pop_front() {
            if visited_nodes.contains(&node_to_visit.get_id()) {
                continue;
            }

            let node_data = node_to_visit.get_data();
            visited_nodes.insert(node_to_visit.get_id());

            if node_data.get_location() == destination {
                found_target_node = Some(node_to_visit);
                break;
            }

            let node_edges = world_graph.get_edges(node_to_visit);
            for next_node in node_edges {
                nodes_to_visit.push_back(next_node);

                if !visited_nodes.contains(&next_node.get_id()) {
                    discovered_by.insert(next_node.get_id(), node_to_visit.get_id());
                }
            }
        }

        let target_node = found_target_node.unwrap_or_else(|| {
            panic!(
                r#"shortest_path: Could not find destination for node {}, {}"#,
                destination.get_x(),
                destination.get_y()
            )
        });

        let path = get_bfs_path(source_node, target_node, &discovered_by, world_graph);

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

    pub fn get_types(&self) -> Vec<RoomObject> {
        self.path
            .iter()
            .map(|path_node| *path_node.get_type())
            .collect()
    }
}

#[derive(Component)]
pub struct PathTarget {
    movement_timer: Timer,

    starting_position: Transform,

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

            starting_position: current_position,

            logical_target,
            physical_target,

            current_position,
        }
    }

    fn compute_expected_position(&self, source: f32, target: f32) -> f32 {
        if self.movement_timer.finished() {
            return target;
        }

        let difference = target - source;
        let time_passed_percentage = self.movement_timer.fraction();
        source + (difference * time_passed_percentage)
    }

    pub fn advance(&mut self, time: &Res<Time>) -> Transform {
        self.movement_timer.tick(time.delta());

        let advanced_x = self.compute_expected_position(
            self.starting_position.translation.x,
            self.physical_target.translation.x,
        );
        let advanced_y = self.compute_expected_position(
            self.starting_position.translation.y,
            self.physical_target.translation.y,
        );
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
    mut explorer: Query<(
        Entity,
        &LogicalCoordinates,
        &mut ExplorerState,
        Option<&PathTarget>,
        Option<&mut Pathfinding>,
    )>,
    room_traversal_graph: Query<&Graph>,
    mut commands: Commands,
) {
    if exit_door.is_empty() || explorer.is_empty() || room_traversal_graph.is_empty() {
        return;
    }

    let (
        explorer_entity,
        explorer_location,
        mut explorer_state,
        explorer_current_target,
        explorer_wandering_path,
    ) = explorer
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

    if *explorer_state != ExplorerState::Wandering {
        return;
    }

    let explorer_path = if let Some(next_explorer_location) = explorer_current_target {
        Pathfinding::shortest_path(
            &next_explorer_location.get_logical_target(),
            exit_door_location,
            room_graph,
        )
    } else {
        Pathfinding::shortest_path(explorer_location, exit_door_location, room_graph)
    };

    if let Some(mut current_explorer_path) = explorer_wandering_path {
        *current_explorer_path = explorer_path;
    } else {
        commands.entity(explorer_entity).insert(explorer_path);
    }

    *explorer_state = ExplorerState::Exiting;
}

pub fn set_explorer_target(
    mut explorer: Query<(Entity, &Transform, &mut Pathfinding), Without<PathTarget>>,
    tiles: Query<(&LogicalCoordinates, &Transform)>,
    movement_time: Res<MovementTime>,
    mut commands: Commands,
) {
    if explorer.is_empty() {
        return;
    }

    let (explorer_entity, explorer_position, mut explorer_path) = explorer
        .single_mut()
        .expect("set_explorer_target: Could not find explorer.");

    if !explorer_path.is_traveling() {
        commands.entity(explorer_entity).remove::<Pathfinding>();
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

pub fn make_explorer_wander(
    mut explorer: Query<
        (Entity, &LogicalCoordinates),
        (Added<ExplorerState>, Without<Pathfinding>),
    >,
    room_traversal_graph: Query<&Graph>,
    mut commands: Commands,
) {
    if explorer.is_empty() || room_traversal_graph.is_empty() {
        return;
    }

    let room_graph = room_traversal_graph
        .single()
        .expect("make_explorer_wander: Could not find graph for the room.");

    let (explorer_entity, explorer_position) = explorer
        .single_mut()
        .expect("make_explorer_wander: Could not find explorer.");

    let explorer_path = Pathfinding::explore_all(explorer_position, room_graph);
    commands.entity(explorer_entity).insert(explorer_path);
}

pub fn unlock_exit_door_with_explorer(
    movement_changes: Query<
        &LogicalCoordinates,
        (With<ExplorerState>, Changed<LogicalCoordinates>),
    >,
    hidden_floor_switch: Query<&LogicalCoordinates, With<HiddenFloorSwitch>>,
    mut exit_door: Query<&mut ExitDoorState>,
) {
    if movement_changes.is_empty() || exit_door.is_empty() || hidden_floor_switch.is_empty() {
        return;
    }

    for movement_coordinates in movement_changes.iter() {
        let hidden_floor_switch_coordinates = hidden_floor_switch.single().expect(
            "unlock_hidden_door_with_explorer: Could not find the coordinates of the hidden floor switch.",
        );
        let mut exit_door_state = exit_door
            .single_mut()
            .expect("unlock_exit_door_with_explorer: Could not find the exit door.");

        if *movement_coordinates == *hidden_floor_switch_coordinates {
            *exit_door_state = ExitDoorState::Open;
        }
    }
}

pub fn unlock_exit_door_with_viewer_click(
    mut movement_changes: EventReader<LogicalCoordinates>,
    hidden_floor_switch: Query<&LogicalCoordinates, With<HiddenFloorSwitch>>,
    mut exit_door: Query<&mut ExitDoorState>,
) {
    if movement_changes.is_empty() || exit_door.is_empty() || hidden_floor_switch.is_empty() {
        return;
    }

    for movement_coordinates in movement_changes.read() {
        let hidden_floor_switch_coordinates = hidden_floor_switch.single().expect(
            "unlock_hidden_door_with_viewer_click: Could not find the coordinates of the hidden floor switch.",
        );
        let mut exit_door_state = exit_door
            .single_mut()
            .expect("unlock_exit_door_with_viewer_click: Could not find the exit door.");

        if *movement_coordinates == *hidden_floor_switch_coordinates {
            *exit_door_state = ExitDoorState::Open;
        }
    }
}
