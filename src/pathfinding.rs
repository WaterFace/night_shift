use std::collections::VecDeque;

use bevy::{prelude::*, transform::TransformSystem, utils::HashMap};
use bevy_rapier2d::prelude::*;

use crate::{
    debug::DebugOverlay,
    map::{self, PathNode, Region},
    physics,
    player::Player,
    states::AppState,
};

#[derive(Resource, Debug)]
pub struct Pathfinder {
    pub nodes: Vec<Vec2>,
    pub visible: HashMap<usize, Vec<usize>>,
    pub distance: HashMap<(usize, usize), f32>,
    pub regions: Vec<map::Region>,
    pub region_to_nodes: HashMap<usize, Vec<usize>>,
    pub node_to_region: HashMap<usize, usize>,
    pub collision_groups: CollisionGroups,
    pub player_region: Option<usize>,
    /// Stores the shortest paths between nodes as sequences of nodes to follow
    pub paths: HashMap<(usize, usize), (f32, Vec<usize>)>,
}

impl Default for Pathfinder {
    fn default() -> Self {
        Pathfinder {
            nodes: Default::default(),
            visible: Default::default(),
            regions: Default::default(),
            region_to_nodes: Default::default(),
            player_region: Default::default(),
            node_to_region: Default::default(),
            distance: Default::default(),
            paths: Default::default(),
            collision_groups: CollisionGroups::new(physics::WALL_GROUP, physics::WALL_GROUP),
        }
    }
}

impl Pathfinder {
    fn compute_visibility(&mut self, rapier_context: &RapierContext) {
        self.visible.clear();
        for idx_a in 0..self.nodes.len() {
            for idx_b in 0..self.nodes.len() {
                if idx_a == idx_b {
                    continue;
                }
                let node_a = self.nodes[idx_a];
                let node_b = self.nodes[idx_b];
                if rapier_context
                    .cast_ray(
                        node_a,
                        node_b - node_a,
                        1.0,
                        true,
                        self.collision_groups.into(),
                    )
                    .is_none()
                {
                    // node_a can see node_b and vice versa
                    self.visible.entry(idx_a).or_default().push(idx_b);
                    self.visible.entry(idx_b).or_default().push(idx_a);
                    self.distance
                        .insert((idx_a, idx_b), (node_b - node_a).length());
                    self.distance
                        .insert((idx_b, idx_a), (node_b - node_a).length());
                }
            }
        }
    }

    fn compute_regions(&mut self) {
        self.region_to_nodes.clear();

        for (i, region) in self.regions.iter().enumerate() {
            for (j, node) in self.nodes.iter().enumerate() {
                if region.area.contains(*node) {
                    self.region_to_nodes.entry(i).or_default().push(j);
                    self.node_to_region.insert(j, i);
                }
            }
        }
    }

    fn compute_paths(&mut self) {
        let mut queue = VecDeque::new();
        let mut explored = Vec::new();
        let mut parents = HashMap::new();
        for starting_node in 0..self.nodes.len() {
            for goal_node in 0..self.nodes.len() {
                if starting_node == goal_node {
                    continue;
                }
                queue.clear();
                explored.clear();
                parents.clear();

                explored.push(starting_node);
                queue.push_back(starting_node);
                while let Some(node) = queue.pop_front() {
                    if node == goal_node {
                        let mut current = node;
                        let mut path_length = 0.0;
                        let mut path = vec![node];
                        while let Some(parent) = parents.get(&current) {
                            path_length += self.distance[&(current, *parent)];
                            path.push(*parent);
                            current = *parent;
                        }
                        match self.paths.get(&(starting_node, goal_node)) {
                            None => {
                                path.reverse();
                                self.paths
                                    .insert((starting_node, goal_node), (path_length, path));
                            }
                            Some((prev_length, _)) => {
                                if path_length < *prev_length {
                                    path.reverse();
                                    self.paths
                                        .insert((starting_node, goal_node), (path_length, path));
                                }
                            }
                        }
                    }
                    if !self.visible.contains_key(&node) {
                        continue;
                    }
                    for adjacent_node in self.visible[&node].iter().copied() {
                        if explored.contains(&adjacent_node) {
                            continue;
                        }
                        explored.push(adjacent_node);
                        parents.insert(adjacent_node, node);
                        queue.push_back(adjacent_node);
                    }
                }
            }
        }
    }

    pub fn get_path(&self, start_node: usize, goal_node: usize) -> &[usize] {
        match self.paths.get(&(start_node, goal_node)) {
            Some((_, path)) => path,
            None => &[],
        }
    }

    pub fn closest_node(&self, point: Vec2) -> Option<usize> {
        match self.get_region_index(point) {
            Some(region_idx) => {
                let mut shortest = f32::INFINITY;
                let mut shortest_at = usize::MAX;
                for node in self.region_to_nodes[&region_idx].iter().cloned() {
                    let dist = self.nodes[node].distance_squared(point);
                    if dist < shortest {
                        shortest = dist;
                        shortest_at = node;
                    }
                }
                Some(shortest_at)
            }
            None => None,
        }
    }

    fn get_region_index(&self, point: Vec2) -> Option<usize> {
        for (i, region) in self.regions.iter().enumerate() {
            if region.area.contains(point) {
                return Some(i);
            }
        }
        None
    }

    pub fn nodes_in_player_region(&self) -> &[usize] {
        let Some(player_region) = self.player_region else {
            return &[];
        };

        return &self.region_to_nodes[&player_region];
    }

    pub fn get_region(&self, point: Vec2) -> Option<&Region> {
        match self.get_region_index(point) {
            None => None,
            Some(i) => Some(&self.regions[i]),
        }
    }
}

fn start_pathfinder(mut commands: Commands) {
    commands.insert_resource(Pathfinder::default());
}

#[derive(Debug, Default, Resource)]
pub struct Precomputed;

pub fn precompute(
    mut commands: Commands,
    state: Option<Res<Precomputed>>,
    mut pathfinder: ResMut<Pathfinder>,
    rapier_context: Res<RapierContext>,
) {
    if state.is_none() {
        debug!("Computing pathfinding");
        pathfinder.compute_visibility(&rapier_context);
        pathfinder.compute_regions();
        pathfinder.compute_paths();
        commands.insert_resource(Precomputed);
    }
}

fn cleanup_pathfinding(mut commands: Commands) {
    commands.remove_resource::<Precomputed>();
}

pub fn add_nodes_and_regions(
    node_query: Query<&Transform, Added<PathNode>>,
    region_query: Query<&Region, (Added<Region>, Without<PathNode>)>,
    mut pathfinder: ResMut<Pathfinder>,
) {
    pathfinder
        .nodes
        .extend(node_query.iter().map(|t| t.translation.truncate()));

    pathfinder.regions.extend(region_query.iter().cloned());
}

fn find_player_region(
    player_query: Query<&Transform, With<Player>>,
    mut pathfinder: ResMut<Pathfinder>,
    mut previous_region: Local<Option<usize>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let Some(current_region) = pathfinder.get_region_index(player_transform.translation.truncate())
    else {
        return;
    };

    if let Some(i) = pathfinder.player_region {
        if i != current_region {
            *previous_region = Some(i);
            pathfinder.player_region = Some(current_region);
        }
    } else {
        pathfinder.player_region = Some(current_region);
    }
}

fn debug_pathfinding(
    pathfinder: Res<Pathfinder>,
    mut gizmos: Gizmos,
    debug_overlay: Res<DebugOverlay>,
) {
    if debug_overlay.enabled {
        for (from, tos) in pathfinder.visible.iter() {
            for to in tos {
                let node1 = pathfinder.nodes[*from];
                let node2 = pathfinder.nodes[*to];

                if node1.x >= node2.x {
                    gizmos.line_2d(node1, node2, Color::ORANGE);
                }
            }
        }
    }
}

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (add_nodes_and_regions, debug_pathfinding, find_player_region)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnEnter(AppState::InGame), start_pathfinder)
        .add_systems(OnExit(AppState::InGame), cleanup_pathfinding)
        .add_systems(
            PostUpdate,
            precompute
                .run_if(in_state(AppState::InGame))
                .after(TransformSystem::TransformPropagate),
        );
    }
}
