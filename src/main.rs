use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::math::Vec2;
use bevy::prelude::Commands;
use bevy::ecs::system::EntityCommands;
use bevy::ecs::component::Component;
use bevy::input::{keyboard::KeyCode, Input};
use num_traits::FromPrimitive;
use rand::prelude::*;
use strum_macros::Display;
use bevy::render::camera::Camera;
use bevy_text::Text2dBundle;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime};
use std::path::Path;
use std::io::prelude::*;
use std::fs;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

type PointList = Vec<Vec2>;

lazy_static! {
    static ref ROTATION_TRANSFORMS: Vec<Vec<Vec<Transform>>> = {
        // Create a rhombus centered at the origin
        let r_fat = Rhombus::new_fat();
        let r_skinny = Rhombus::new_skinny();
        let fat_points = r_fat.get_points();
        let skinny_points = r_skinny.get_points();

        let fat_small_diag_len = r_fat.leg_len * (2.0 - 2.0 * r_fat.small_angle.cos()).sqrt();
        let fat_half_small_diag = fat_small_diag_len / 2.0;

        let skinny_small_diag_len = r_skinny.leg_len * (2.0 - 2.0 * r_skinny.small_angle.cos()).sqrt();
        let skinny_half_small_diag = skinny_small_diag_len / 2.0;

        let mut v = Vec::<Vec::<Vec::<Transform>>>::new();
        v.push(Vec::<Vec::<Transform>>::new());
        {
            let fat = &mut v[0];
            {
                fat.push(Vec::<Transform>::new());
                {
                    let fat_vert_angle = f32::to_radians(180.0 - Rhombus::FAT_LARGE_ANGLE / 2.0 - Rhombus::FAT_LARGE_ANGLE / 2.0);
                    let fat_fat_sides = &mut fat[0];
                    {
                        // Fat on Fat on side 0
                        fat_fat_sides.push(
                            make_rotation_transform(fat_vert_angle, fat_points[r_fat.get_top_index()], fat_half_small_diag)
                        );
                    }
                    {
                        // Fat on Fat on side 1
                        fat_fat_sides.push(
                            make_rotation_transform(-fat_vert_angle, fat_points[r_fat.get_top_index()], fat_half_small_diag)
                        );
                    }
                    {
                        // Fat on Fat on side 2
                        fat_fat_sides.push(
                            make_rotation_transform(fat_vert_angle, fat_points[r_fat.get_bottom_index()], -fat_half_small_diag)
                        );
                    }
                    {
                        // Fat on Fat on side 3
                        fat_fat_sides.push(
                            make_rotation_transform(-fat_vert_angle, fat_points[r_fat.get_bottom_index()], -fat_half_small_diag)
                        );
                    }
                }
                fat.push(Vec::<Transform>::new());
                {
                    let skinny_fat_horizontal_angle = f32::to_radians(180.0 + Rhombus::FAT_SMALL_ANGLE / 2.0 - Rhombus::SKINNY_SMALL_ANGLE / 2.0);
                    let skinny_fat_vert_angle = f32::to_radians(180.0 - Rhombus::SKINNY_LARGE_ANGLE / 2.0 - Rhombus::FAT_LARGE_ANGLE / 2.0);
                    let skinny_fat_sides = &mut fat[1];
                    {
                        // Skinny onto fat on side 0
                        skinny_fat_sides.push(
                            make_rotation_transform(skinny_fat_horizontal_angle, fat_points[r_fat.get_left_index()], -skinny_half_small_diag)
                        );
                    }
                    {
                        // Skinny onto fat on side 1
                        skinny_fat_sides.push(
                            make_rotation_transform(-skinny_fat_vert_angle, fat_points[r_fat.get_top_index()], skinny_half_small_diag)
                        );
                    }
                    {
                        // Skinny onto fat on side 2
                        skinny_fat_sides.push(
                            make_rotation_transform(skinny_fat_vert_angle + f32::to_radians(180.0), fat_points[r_fat.get_bottom_index()], skinny_half_small_diag)
                        );
                    }
                    {
                        // Skinny onto fat on side 3
                        skinny_fat_sides.push(
                            make_rotation_transform(-f32::to_radians(Rhombus::SKINNY_SMALL_ANGLE / 2.0), fat_points[r_fat.get_left_index()], -skinny_half_small_diag)
                        );
                    }
                }
            }
        }

        v.push(Vec::<Vec::<Transform>>::new());
        {
            let skinny = &mut v[1];
            {
                skinny.push(Vec::<Transform>::new());
                {
                    let fat_skinny_horizontal_angle = f32::to_radians(180.0 + Rhombus::FAT_LARGE_ANGLE / 2.0 - Rhombus::SKINNY_LARGE_ANGLE / 2.0);

                    let skinny_fat_sides = &mut skinny[0];
                    {
                        // Fat on Skinny on side 0
                        skinny_fat_sides.push(
                            make_rotation_transform(fat_skinny_horizontal_angle, skinny_points[r_skinny.get_left_index()], -fat_half_small_diag)
                        );
                    }
                    {
                        // Fat on Skinny on side 1
                        skinny_fat_sides.push(
                            make_rotation_transform(f32::to_radians(Rhombus::SKINNY_SMALL_ANGLE / 2.0), skinny_points[r_skinny.get_right_index()], fat_half_small_diag)
                        );
                    }
                    {
                        // Fat on Skinny on side 2
                        skinny_fat_sides.push(
                            make_rotation_transform(f32::to_radians(Rhombus::FAT_LARGE_ANGLE / 2.0), skinny_points[r_skinny.get_bottom_index()], -fat_half_small_diag)
                        );
                    }
                    {
                        // Fat on Skinny on side 3
                        skinny_fat_sides.push(
                            make_rotation_transform(f32::to_radians(90.0 + Rhombus::FAT_LARGE_ANGLE / 2.0 - Rhombus::SKINNY_SMALL_ANGLE / 2.0), skinny_points[r_skinny.get_bottom_index()], fat_half_small_diag)
                        );
                    }
                }
                skinny.push(Vec::<Transform>::new());
                {
                    let skinny_skinny_angle = f32::to_radians(180.0 + 180.0 - Rhombus::SKINNY_LARGE_ANGLE / 2.0 - Rhombus::SKINNY_LARGE_ANGLE / 2.0);
                    let skinny_skinny_sides = &mut skinny[1];
                    {
                        // Skinny -> Skinny on side 0
                        skinny_skinny_sides.push(
                            make_rotation_transform(skinny_skinny_angle, skinny_points[r_skinny.get_top_index()], -skinny_half_small_diag)
                        );
                    }
                    {
                        // Skinny -> Skinny on side 1
                        skinny_skinny_sides.push(
                            make_rotation_transform(-skinny_skinny_angle, skinny_points[r_skinny.get_top_index()], -skinny_half_small_diag)
                        );
                    }
                    {
                        // Skinny -> Skinny on side 2
                        skinny_skinny_sides.push(
                            make_rotation_transform(skinny_skinny_angle, skinny_points[r_skinny.get_bottom_index()], skinny_half_small_diag)
                        );
                    }
                    {
                        // Skinny -> Skinny on side 3
                        skinny_skinny_sides.push(
                            make_rotation_transform(-skinny_skinny_angle, skinny_points[r_skinny.get_bottom_index()], skinny_half_small_diag)
                        );
                    }
                }
            }
        }
        v
    };
}


trait PenroseEnum : Clone + Copy + std::fmt::Display {
    fn get_all() -> Vec<Self>;
}

trait Tile<P: PenroseEnum> : Component + Clone {
    fn new_random() -> Self;
    fn new(penrose_type: P) -> Self;
    fn get_num_sides() -> usize;
    fn get_matching_side(onto_type: P, onto_side: u8, other_type: P) -> u8;
    fn insert_shape_component(&self, transform: Transform, entity_commands: &mut EntityCommands);
    fn spawn_dots_entities(&self, parent: Entity, commands: &mut Commands, asset_server: &Res<AssetServer>);
    fn has_free_sides(&self) -> bool;
    fn get_free_sides(&self) -> Vec<u8>;
    fn set_side_used(&mut self, side: u8);
    fn set_side_free(&mut self, side: u8);
    fn get_side_used(&self, side: u8) -> bool;
    //fn set_invalid_placement(&mut self, side: u8, penrose_type: P);
    fn get_points(&self) -> PointList;
    fn get_type(&self) -> P;
    fn get_connection_transform(&self, side: u8, other_type: P) -> Transform;
}

struct TileWithTransform<'a, T> {
    tile: &'a T,
    transform: &'a Transform,
}

impl<'a, T> TileWithTransform<'a, T> {
    fn new(tile: &'a T, transform: &'a Transform) -> Self {
        TileWithTransform {
            tile: tile,
            transform: transform,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct TilerHandle(usize);

impl Default for TilerHandle {
    fn default() -> Self {
        TilerHandle(usize::MAX)
    }
}

impl TilerHandle {
    fn is_valid(&self) -> bool {
        self.0 != usize::MAX
    }
}

struct PlacedTile<'a, T> {
    tile: &'a T,
    transform: &'a Transform,
    handle: TilerHandle
}

#[derive(Clone, Copy)]
struct Edge {
    start: Vec2,
    end: Vec2
}

impl Edge {
    fn new(p1: Vec2, p2: Vec2) -> Self {
        if p1 < p2 {
            Edge {
                start: p1,
                end: p2,
            }
        } else {
            Edge {
                start: p2,
                end: p1,
            }
        }
    }
}

fn get_points_for_tile<P: PenroseEnum, T: Tile<P> >(tile: &TileWithTransform<T>) -> Vec<Vec2> {
    let origin_points = tile.tile.get_points();
    let mut transformed_points = Vec::new();
    for p in origin_points {
        transformed_points.push((*tile.transform * p.extend(0.0)).truncate());
    }

    transformed_points
}

fn get_edge_vectors_for_tile<P: PenroseEnum, T: Tile<P> >(tile: &TileWithTransform<T>) -> Vec<Vec2> {
    let points = get_points_for_tile(tile);
    let mut vectors = Vec::new();
    for i in 0..points.len() {
        let next_i = (i + 1) % points.len();
        vectors.push(points[next_i] - points[i]);
    }
    vectors
}

fn get_edges_for_tile<P: PenroseEnum, T: Tile<P> >(tile: &TileWithTransform<T>) -> Vec<Edge> {
    let mut edges = Vec::new();
    let points = tile.tile.get_points();
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        let world_point_0 = (*tile.transform * points[i].extend(0.0)).truncate();
        let world_point_1 = (*tile.transform * points[j].extend(0.0)).truncate();
        edges.push(Edge::new(world_point_0, world_point_1));
    }

    edges
}

#[derive(Clone, Copy)]
struct EdgeData<P: PenroseEnum> {
    entity: Entity,
    side: u8,
    penrose_type: P
}

#[derive(Default)]
struct EdgeLookup<P: PenroseEnum> {
    edges: Vec<Edge>,
    tiles: Vec<Vec<EdgeData<P>> >
}

struct EdgeResult<P: PenroseEnum> {
    edge: Edge,
    data: Vec<EdgeData<P>>
}

impl<P: PenroseEnum> EdgeLookup<P> {
    fn edge_search_fuzzy(&self, edge: &Edge) -> Result<usize, usize> {
        let epsilon = 0.004;
        /*self.edges.binary_search_by(|probe| {
            let close_start = probe.start - edge.start;
            let close_end = probe.end - edge.end;
            if close_start.x.abs() <= epsilon && close_start.y.abs() <= epsilon &&
                close_end.x.abs() <= epsilon && close_end.y.abs() <= epsilon {
                Ordering::Equal
            } else {
                if probe.start < edge.start && probe.end < edge.end {
                    Ordering::Less
                } else if probe.start < edge.start && probe.end > edge.end {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        })*/
        for i in 0..self.edges.len() {
            let probe = &self.edges[i];
            let close_start = probe.start - edge.start;
            let close_end = probe.end - edge.end;
            if close_start.x.abs() <= epsilon && close_start.y.abs() <= epsilon &&
                close_end.x.abs() <= epsilon && close_end.y.abs() <= epsilon {
                return Ok(i);
            }
        }

        Err(self.edges.len())
    }

    fn get_tiles_for_edge(&self, edge: &Edge) -> Option<EdgeResult<P>> {
        self.get_tiles_for_edge_excluding(edge, Entity::new(0))
    }

    fn get_tiles_for_edge_excluding(&self, edge: &Edge, exclude: Entity) -> Option<EdgeResult<P>> {
        let result = self.edge_search_fuzzy(edge);
        match result {
            Ok(pos) => {
                let tiles_copy = self.tiles[pos].clone();
                Some(EdgeResult {
                    edge: *edge,
                    data: tiles_copy.into_iter().filter(|x| x.entity != exclude).collect()
                })
            },
            Err(_) => None 
        }
    }

    fn get_tiles_for_all_edges<T: Tile<P>>(&self, tile: &TileWithTransform<T>) -> Vec<EdgeResult<P>> {
        self.get_tiles_for_all_edges_excluding(tile, Entity::new(0))
    }

    fn get_tiles_for_all_edges_excluding<T: Tile<P>>(&self, tile: &TileWithTransform<T>, exclude: Entity) -> Vec<EdgeResult<P>> {
        let edges = get_edges_for_tile(tile);
        let mut edge_data = Vec::new();
        for i in 0..edges.len() {
            match self.get_tiles_for_edge_excluding(&edges[i], exclude) {
                Some(e) => { 
                    println!("HIT for edge ({:?} {:?}) len {}", edges[i].start, edges[i].end, e.data.len());
                    if e.data.len() > 0 {
                        edge_data.push(e);
                    }
                },
                _ => {}
            }
        }

        edge_data
    }

    fn add_edge(&mut self, edge: &Edge, data: EdgeData<P>) {
        let result = self.edge_search_fuzzy(&edge);
        match result {
            Ok(pos) => {
                println!("Adding to EXISTING edge at {} ({:?}, {:?}) {:?} {} {} new len {}",
                    pos, edge.start, edge.end, data.entity, data.side, data.penrose_type, self.tiles[pos].len() + 1);
                self.tiles[pos].push(data)
            },
            Err(pos) =>  {
                println!("Adding to NEW edge at {} ({:?}, {:?}) {:?} {} {}", pos, edge.start, edge.end, data.entity, data.side, data.penrose_type);
                self.edges.insert(pos, *edge);

                let mut v = Vec::new();
                v.push(data);
                self.tiles.insert(pos, v);
            }
        }
    }

    fn add_edges<T: Tile<P>>(&mut self, tile: &TileWithTransform<T>, entity: &Entity) {
        let edges = get_edges_for_tile(tile);
        for i in 0..edges.len() {
            println!("--> Adding edge {:?} ({:?}, {:?}) {}", entity, edges[i].start, edges[i].end, i);
            self.add_edge(&edges[i], EdgeData {
                entity: *entity,
                side: i as u8,
                penrose_type: tile.tile.get_type()
            });
        }
    }

    fn remove_entity(&mut self, entity: Entity) {
        assert!(self.tiles.len() == self.edges.len());
        let mut i = 0;
        while i < self.tiles.len() {
            let edges: &mut Vec<EdgeData<P>> = &mut self.tiles[i];
            edges.retain(|data| {
                data.entity != entity
            });

            if edges.len() == 0 {
                self.tiles.remove(i);
                self.edges.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct TilePlacementInfo<P>
{
    on_handle: TilerHandle,
    on_type: P,
    on_side: u8,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct AddedTile<P> {
    on: Option<TilePlacementInfo<P>>,
    penrose_type: P
}

#[derive(Default)]
struct PenroseTiler<P: PenroseEnum> {
    history: Vec<AddedTile<P>>,
    tiles_added: Vec<Entity>,
    needs_check: bool,
    is_replay: bool
}

struct EdgeTile;

impl<P: PenroseEnum> PenroseTiler<P> {
    fn is_separating_axis(normal: &Vec2, points_a: &Vec<Vec2>, points_b: &Vec<Vec2>) -> bool {
        let epsilon = 0.004;
        let mut min_a = f32::INFINITY;
        let mut max_a = f32::NEG_INFINITY;
        let mut min_b = f32::INFINITY;
        let mut max_b = f32::NEG_INFINITY;

        for v in points_a {
            let projection = v.dot(*normal);
            min_a = min_a.min(projection);
            max_a = max_a.max(projection);
        }

        for v in points_b {
            let projection = v.dot(*normal);
            min_b = min_b.min(projection);
            max_b = max_b.max(projection);
        }
        
        let diff_1 = max_a - min_b;
        let diff_2 = max_b - min_a;


        // !(max_a >= min_b && max_b >= min_a)
        let overlap = diff_1 > epsilon && diff_2 > epsilon;

        /*println!("overlap: {}, diff_1: {}, diff_2: {}, min_a: {}, max_a: {}, min_b: {}, max_b: {}",
            overlap, diff_1, diff_2, min_a, max_a, min_b, max_b);*/

        !overlap
    }

    fn tiles_collide<T: Tile<P>>(tile_a: &TileWithTransform<T>, tile_b: &TileWithTransform<T>) -> bool {
        let points_a = get_points_for_tile(tile_a);
        let points_b = get_points_for_tile(tile_b);

        //println!("points_a: {:?}", points_a);
        //println!("points_b: {:?}", points_b);
        
        let mut vectors = get_edge_vectors_for_tile(tile_a);
        vectors.extend(get_edge_vectors_for_tile(tile_b));

        for v in &vectors {
            let normal = v.perp();
            if PenroseTiler::<P>::is_separating_axis(&normal, &points_a, &points_b) {
                return false;
            }
        }

        return true;
    }
    
    fn is_replaying(&self) -> bool {
        //self.history.len() > self.tiles_added.len()
        self.is_replay
    }

    fn remove(&mut self, handle: TilerHandle) {
        self.tiles_added.remove(handle.0);

        if !self.is_replaying() {
            self.history.remove(handle.0);
        }
    }

    fn spawn_tile_at<T: Tile<P>>(
        &mut self, 
        tile: &T,
        transform: Transform,
        on_info: Option<TilePlacementInfo<P>>, 
        commands: &mut Commands,
        asset_server: &Res<AssetServer>
    ) -> Entity {
        let mut entity = commands.spawn();
        let id = entity.id();
        tile.insert_shape_component(transform, &mut entity);
        entity.insert(tile.clone());
        entity.insert(EdgeTile);
        entity.insert(TilerHandle(self.tiles_added.len()));

        tile.spawn_dots_entities(id, commands, asset_server);
        self.history.push(AddedTile {
            on: on_info,
            penrose_type: tile.get_type()
        });
        self.tiles_added.push(id);

        id
    }

    fn spawn_tile_at_origin<T: Tile<P>>(&mut self, tile: &T, commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        self.spawn_tile_at(
            tile,
            Transform::identity(),
            None, 
            commands,
            asset_server)
    }

    fn spawn_tile_on<T: Tile<P>>(
        &mut self, 
        on_tile_side: u8,
        tile: &T, 
        on_tile: &PlacedTile<T>, 
        commands: &mut Commands,
        asset_server: &Res<AssetServer>
    ) -> (Entity, Transform) {
        assert!(!on_tile.tile.get_side_used(on_tile_side));

        let tile_side = T::get_matching_side(on_tile.tile.get_type(), on_tile_side, tile.get_type());
        let mut tile = tile.clone();

        tile.set_side_used(tile_side);
        let transform = (*on_tile.transform) * on_tile.tile.get_connection_transform(on_tile_side, tile.get_type());

        let entity = self.spawn_tile_at(
            &tile,
            transform, 
            Some(TilePlacementInfo {
                on_handle: on_tile.handle,
                on_type: on_tile.tile.get_type(),
                on_side: on_tile_side,
            }),
            commands,
            asset_server);

        println!("Setting side used {} new entity {:?} on tile transform {:?}", tile_side, entity, *on_tile.transform);


        (entity, transform)
    }

    fn spawn_random_tile_at_origin<T: Tile<P>>(&mut self, 
        commands: &mut Commands, 
        asset_server: &Res<AssetServer>
    ) -> (T, Entity) {
        let tile = T::new_random();
        let entity = self.spawn_tile_at_origin(&tile, commands, asset_server);
        (tile, entity)
    }

    fn spawn_next_tile_from_history<T: Tile<P>>(&mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        query: &mut Query<(Entity, &mut Rhombus, &TilerHandle, &Transform)>
    ) -> Option<(T, Transform, Entity)>  {
        if self.is_replaying() && self.history.len() > self.tiles_added.len() {
            let next = self.history[self.tiles_added.len()];
            println!("      SPAWNING {} FROM HISTORY", self.tiles_added.len());

            match next.on {
                Some(t) => {
                    println!("      SPAWNING on_handle {}", t.on_handle.0);
                    let entity = self.tiles_added[t.on_handle.0];
                    let on_tile = query.get_component::<T>(entity).unwrap();
                    let on_tile_transform = query.get_component::<Transform>(entity).unwrap();
                    let on_tile_placed = PlacedTile {
                        tile: on_tile,
                        transform: &on_tile_transform,
                        handle: t.on_handle
                    };
                    let new_tile = T::new(next.penrose_type);
                    let (new_entity, new_transform) = self.spawn_tile_on(t.on_side, &new_tile, &on_tile_placed, commands, asset_server);
                    Some((new_tile, new_transform, new_entity))
                },
                None => {
                    println!("      SPAWNING AT ORIGIN");

                    //Some(self.spawn_random_tile_at_origin(commands, asset_server))
                    let new_tile = T::new(next.penrose_type);
                    Some((new_tile.clone(), Transform::default(), self.spawn_tile_at_origin(&new_tile, commands, asset_server)))
                }
            }          
        } else {
            None
        }
    }

    fn get_allowed_tiles_to_place<T: Tile<P>>(on_tile: &PlacedTile<T>, 
        edge_lookup: &EdgeLookup<P>, 
        edge_vec: &Vec<(Entity, Mut<T>, &TilerHandle, &Transform)>,
    ) -> Vec<(u8, P)> {
        let mut allowed_tiles = Vec::new();
        let free_sides = on_tile.tile.get_free_sides();
        let all_types: Vec<P> = P::get_all();
        for side in free_sides {
            for t in &all_types {
                //println!("Pushing {}, {}", side, *t);
                allowed_tiles.push((side, *t));
            }
        }

        allowed_tiles.retain(|(on_tile_side, new_tile_penrose_type)| {
            let possible = T::new(*new_tile_penrose_type);
            let points = possible.get_points();
            let transform = (*on_tile.transform) * on_tile.tile.get_connection_transform(*on_tile_side, *new_tile_penrose_type);
            let matching_side = T::get_matching_side(on_tile.tile.get_type(), *on_tile_side, *new_tile_penrose_type);

            let new_tile = TileWithTransform::new(&possible, &transform);
            for (_, existing_t, _, existing_trans) in edge_vec {
                let existing_tile = TileWithTransform::new(& **existing_t, existing_trans);
                if PenroseTiler::tiles_collide(&new_tile, &existing_tile) {
                    //println!("Tiles collide, returning false.");
                    return false;
                }
            }

            for new_side in 0..points.len() {
                let new_side = new_side as u8;
                if new_side == matching_side {
                    continue;
                }
                let point1_index = new_side as usize;
                let point2_index = ((new_side + 1) as usize) % points.len();
                let point1 = (transform * points[point1_index].extend(0.0)).truncate();
                let point2 = (transform * points[point2_index].extend(0.0)).truncate();
                let edge = Edge::new(point1, point2);

                match edge_lookup.get_tiles_for_edge(&edge) {
                    Some(result) => {
                        if result.data.len() == 0 {
                            println!("      ALLOWED No matching tiles for edge, for on_tile_side {} new_side {} new_type {}",
                                *on_tile_side, new_side, *new_tile_penrose_type);
                        }
                        else if result.data.len() > 1 {
                            println!("      NOT ALLOWED More than 1 matching tiles for edge, for on_tile_side {}, new_side {} type {}",
                                *on_tile_side, new_side, *new_tile_penrose_type);
                            return false;
                        } else {
                            //if result.data[0].side == T::get_matching_side(*new_tile_penrose_type, new_side, result.data[0].penrose_type) {
                            if new_side == T::get_matching_side(result.data[0].penrose_type, result.data[0].side, *new_tile_penrose_type) {
                                println!("      ALLOWED matching side for edge, for on_tile_side {}, new_side {} type {} existing_side {}", 
                                    *on_tile_side, new_side, *new_tile_penrose_type, result.data[0].side);
                            } else {
                                println!("      NOT ALLOWED matching side for edge, for on_tile_side {}, new_side {} type {} existing_side {}",
                                    *on_tile_side, new_side, *new_tile_penrose_type, result.data[0].side);
                                return false;
                            }
                        }
                    },
                    None => {
                        println!("      ALLOWED NONE matching tiles for edge, for on_tile_side {} new_side {} new_type {}",
                            *on_tile_side, new_side, *new_tile_penrose_type)
                    }
                }
            }
            true
        });

        allowed_tiles
    }

    fn spawn_random_tile_on<T: Tile<P>>(
        &mut self, 
        on_tile: &PlacedTile<T>,
        edge_lookup: &EdgeLookup<P>,
        edge_vec: &Vec<(Entity, Mut<T>, &TilerHandle, &Transform)>,
        commands: &mut Commands, 
        asset_server: &Res<AssetServer>

    ) -> Option<(T, Transform, Entity)> {
        let possible_tiles = PenroseTiler::get_allowed_tiles_to_place(on_tile, edge_lookup, edge_vec);
        if possible_tiles.len() == 0 {
            return None;
        }

        let index = rand::thread_rng().gen_range(0..possible_tiles.len());

        let (side, penrose_type) = possible_tiles[index];
        let tile = T::new(penrose_type);

        let (entity, transform) = self.spawn_tile_on(side, &tile, on_tile, commands, asset_server);
        println!("Spawned {:?} {:?} on side {}", entity, transform, side);

        Some((tile, transform, entity))
    }
}

#[derive(Display, Clone, Copy, Primitive, PartialEq, Serialize, Deserialize)]
enum PenroseRhombusType {
    Fat = 0,
    Skinny = 1,

    Count = 2
}

impl Default for PenroseRhombusType {
    fn default() -> Self {
        PenroseRhombusType::Fat
    }
}

impl PenroseEnum for PenroseRhombusType {
    fn get_all() -> Vec<Self> {
        let mut vec = Vec::new();
        for i in 0..(PenroseRhombusType::Count as usize) {
            vec.push(PenroseRhombusType::from_usize(i).unwrap());
        }
        vec
    }
}

#[derive(Clone)]
struct Rhombus {
    small_angle: f32,
    leg_len: f32,
    color: Color,
    used_side_flags: u8,
    penrose_type: PenroseRhombusType,
    invalid_placements: Vec<(u8, PenroseRhombusType)>
}

fn make_rotation_transform(angle: f32, translation: Vec2, distance_to_center_from_translation: f32) -> Transform {
    let centerpoint = Vec3::new(0.0, distance_to_center_from_translation, 0.0);
    let rotation = Quat::from_rotation_z(angle);
    let centerpoint_rotated = rotation * centerpoint;
    let vec3_translation = Vec3::from((translation, 0.0));
    let centerpoint_translated_rotated = centerpoint_rotated + vec3_translation;

    Transform::from_matrix(
        Mat4::from_rotation_translation(
            rotation,
            centerpoint_translated_rotated
        )
    )
}

impl Rhombus {
    const FAT_SMALL_ANGLE: f32 = 72.0;
    const SKINNY_SMALL_ANGLE: f32 = 36.0;
    const FAT_LARGE_ANGLE: f32 = 180.0 - Rhombus::FAT_SMALL_ANGLE;
    const SKINNY_LARGE_ANGLE: f32 = 180.0 - Rhombus::SKINNY_SMALL_ANGLE;

    const UPPER_LEFT_SIDE: usize = 0;
    const UPPER_RIGHT_SIDE: usize = 1;
    const LOWER_RIGHT_SIDE: usize = 2;
    const LOWER_LEFT_SIDE: usize = 3;

    const PENROSE_POINT_INDICES: [[usize; 4]; 2] = [
        [0, 1, 2, 3],
        [0, 1, 2, 3]
    ];

    fn get_left_index(&self) -> usize {
        Rhombus::PENROSE_POINT_INDICES[self.penrose_type as usize][0]
    }
    fn get_top_index(&self) -> usize {
        Rhombus::PENROSE_POINT_INDICES[self.penrose_type as usize][1]
    }
    fn get_right_index(&self) -> usize {
        Rhombus::PENROSE_POINT_INDICES[self.penrose_type as usize][2]
    }
    fn get_bottom_index(&self) -> usize {
        Rhombus::PENROSE_POINT_INDICES[self.penrose_type as usize][3]
    }

    const PENROSE_MATCHING_RULES: [[[u8; 4]; 2]; 2] = [
        [
            // Fat -> Fat rules
            [3, 2, 1, 0],

            // Fat -> Skinny rules
            [0, 2, 3, 1]
        ],
        [
            // Skinny -> Fat rules
            [0, 3, 1, 2],

            // Skinny -> Skinny rules
            [1, 0, 3, 2]
        ]
    ];

    const PENROSE_EDGE_DOT_COLORS: [[Color; 4]; 2] = [
        [
            // Fat colors
            Color::DARK_GREEN, Color::VIOLET, Color::PURPLE, Color::LIME_GREEN
        ],
        [
            // Skinny colors
            Color::LIME_GREEN, Color::DARK_GREEN, Color::PURPLE, Color::VIOLET
        ]
    ];

    const PENROSE_POINT_SCALES: [[f32; 4]; 2] = [
        [
            // Fat scales
            0.33, 0.66, 0.66, 0.33
        ],
        [
            // Skinny scales
            0.66, 0.66, 0.66, 0.66
        ]
    ];

    fn new(penrose_type: PenroseRhombusType) -> Self {
        match penrose_type {
            PenroseRhombusType::Fat => Rhombus::new_fat(),
            PenroseRhombusType::Skinny => Rhombus::new_skinny(),
            _ => panic!("Invalid type")
        }
    }

    fn new_fat() -> Self {
        Rhombus {
            small_angle: f32::to_radians(Rhombus::FAT_SMALL_ANGLE),
            leg_len: 100.0,
            color: Color::BLUE,
            used_side_flags: 0,
            penrose_type: PenroseRhombusType::Fat,
            invalid_placements: Vec::new()
        }
    }

    fn new_skinny() -> Self {
        Rhombus {
            small_angle: f32::to_radians(Rhombus::SKINNY_SMALL_ANGLE),
            leg_len: 100.0,
            color: Color::RED,
            used_side_flags: 0,
            penrose_type: PenroseRhombusType::Skinny,
            invalid_placements: Vec::new()
        }
    }
}

impl Default for Rhombus {
    fn default() -> Self {
        Rhombus::new_skinny()
    }
}

fn get_edge_point(scale: f32, radius: f32, angle: f32, point: Vec2, neg_y: bool) -> Vec2 {
    let x_coord = point.x * scale;
    let y_coord = (point.x - x_coord).abs() * angle.tan() - radius;
    let y_coord = if neg_y { -y_coord } else { y_coord };
    Vec2::new(x_coord, y_coord)
}

fn insert_edge_point(rhombus: &Rhombus, side: usize, entity_commands: &mut EntityCommands) {
    let radius = 5.0;
    let scale = Rhombus::PENROSE_POINT_SCALES[rhombus.penrose_type as usize][side];
    let angle = rhombus.small_angle / 2.0;

    let (point_index, neg_y) = match side {
        Rhombus::UPPER_LEFT_SIDE => (rhombus.get_left_index(), false),
        Rhombus::UPPER_RIGHT_SIDE => (rhombus.get_right_index(), false),
        Rhombus::LOWER_RIGHT_SIDE => (rhombus.get_right_index(), true),
        Rhombus::LOWER_LEFT_SIDE => (rhombus.get_left_index(), true),
        _ => panic!("invalid index!")
    };
    let point = rhombus.get_points()[point_index];
    let color = Rhombus::PENROSE_EDGE_DOT_COLORS[rhombus.penrose_type as usize][side];
    let angle = if point.y < 0.0 { -angle } else { angle };

    entity_commands.insert_bundle(
        GeometryBuilder::build_as(
            &shapes::Circle {
                radius: radius,
                center: get_edge_point(scale, radius, angle, point, neg_y)
            },
            ShapeColors::outlined(color, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(2.0),
            },
            Transform::from_xyz(0.0, 0.0, 1.0)
        )
    );
}

impl Tile<PenroseRhombusType> for Rhombus {
    fn new_random() -> Self {
        let tile_type = PenroseRhombusType::from_i32(rand::thread_rng().gen_range(0..(PenroseRhombusType::Count as i32))).unwrap();
        Rhombus::new(tile_type)
    }
    
    fn new(penrose_type: PenroseRhombusType) -> Self {
        Rhombus::new(penrose_type)
    }

    fn get_num_sides() -> usize {
        4
    }

    fn has_free_sides(&self) -> bool {
        return self.used_side_flags & 0xf != 0xf
    }

    fn get_free_sides(&self) -> Vec<u8> {
        let mut free_sides = Vec::<u8>::new();
        for side in 0..Rhombus::get_num_sides() {
            if !self.get_side_used(side as u8) {
                free_sides.push(side as u8);
            }
        }

        free_sides
    }

    fn get_side_used(&self, side: u8) -> bool {
        return (self.used_side_flags & (1 << side)) != 0;
    }

    /*fn set_invalid_placement(&mut self, side: u8, penrose_type: PenroseRhombusType) {
        if self.invalid_placements.iter().find(|(s, t)| *s == side && *t == penrose_type).is_none() {
            self.invalid_placements.push((side, penrose_type));
        }

        let all_invalid_for_side = self.invalid_placements.iter().filter(|(s, _)| *s == side).collect::<Vec<&(u8, PenroseRhombusType)>>();
        if all_invalid_for_side.len() == PenroseRhombusType::Count as usize {
            self.set_side_used(side);
        }
    }*/

    fn set_side_used(&mut self, side: u8) {
        self.used_side_flags |= 1 << side;
    }

    fn set_side_free(&mut self, side: u8) {
        self.used_side_flags &= !(1 << side);
    }

    fn get_points(&self) -> PointList {
        assert!(self.small_angle <= f32::to_radians(90.0));

        // Create a rhombus centered at the origin
        let long_diag_len = self.leg_len * (2.0 + 2.0 * self.small_angle.cos()).sqrt();
        let half_long_diag = long_diag_len / 2.0;
        let small_diag_len = self.leg_len * (2.0 - 2.0 * self.small_angle.cos()).sqrt();
        let half_small_diag = small_diag_len / 2.0;

        let left = Vec2::new(-half_long_diag, 0.0);
        let top = Vec2::new(0.0, half_small_diag);
        let right = Vec2::new(half_long_diag, 0.0);
        let bottom = Vec2::new(0.0, -half_small_diag);

        vec![
            left,
            top,
            right,
            bottom
        ]
    }

    fn insert_shape_component(&self, transform: Transform, entity_commands: &mut EntityCommands) {
        entity_commands.insert_bundle(
            GeometryBuilder::build_as(
                &shapes::Polygon {
                    points: self.get_points(),
                    closed: true
                },
                ShapeColors::outlined(self.color, Color::BLACK),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(2.0),
                },
                transform
            )
        );
    }

    fn spawn_dots_entities(&self, parent: Entity, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let mut text = commands.spawn();
        text.insert_bundle(Text2dBundle{
            text: Text::with_section(
                format!("{:?}", parent),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        });
        let id = text.id();
        commands.entity(parent).push_children(&[id]);

        for i in 0..4 {
            let mut entity = commands.spawn();
            let id = entity.id();
            insert_edge_point(self, i, &mut entity);
            commands.entity(parent).push_children(&[id]);
        }
    }

    fn get_type(&self) -> PenroseRhombusType {
        self.penrose_type
    }

    fn get_matching_side(onto_type: PenroseRhombusType, onto_side: u8, other_type: PenroseRhombusType) -> u8 {
        assert!(onto_side < 4);
        Rhombus::PENROSE_MATCHING_RULES[onto_type as usize][other_type as usize][onto_side as usize]
    }

    fn get_connection_transform(&self, side: u8, other_type: PenroseRhombusType) -> Transform {
        ROTATION_TRANSFORMS[self.penrose_type as usize][other_type as usize][side as usize]
    }
}

fn remove_rhombus(    
    handle: TilerHandle,
    query: &mut Query<(Entity, &mut Rhombus, &TilerHandle, &Transform)>,
    mut tiler: ResMut<PenroseTiler<PenroseRhombusType>>,
    mut edges: ResMut<EdgeLookup<PenroseRhombusType>>,
    mut commands: Commands
) {
    assert!(handle.is_valid());
    let entity = tiler.tiles_added[handle.0];
    println!("  Removing {:?}", entity);
    let removed_rhombus: &Rhombus = query.get_component::<Rhombus>(entity).unwrap();
    let removed_transform: &Transform = query.get_component::<Transform>(entity).unwrap();
    let edge_data = edges.get_tiles_for_all_edges_excluding(&TileWithTransform::new(removed_rhombus, removed_transform), entity);
    for v in edge_data {
        if v.data.len() == 1 {
            let data = &v.data[0];
            let mut rhombus = query.get_component_mut::<Rhombus>(data.entity).unwrap();
            rhombus.set_side_free(data.side);
            commands.entity(data.entity).insert(EdgeTile);
            println!("  SET FREE side {} of entity {:?}, edge ({:?} {:?})", data.side, data.entity, v.edge.start, v.edge.end);
        } else if v.data.len() > 0 {
            let data = &v.data[0];
            println!("  NOT SETTING side {} of entity {:?} free as there are references OTHER than {:?} len {}",
            data.side, data.entity, entity, v.data.len());
        } else {
            println!("  EMPTY EDGE DATA???");
        }
    }
    commands.entity(entity).despawn_recursive();
    edges.remove_entity(entity);
    tiler.remove(handle);
}

fn spawn_random_valid_tile(
    mut tiler: ResMut<PenroseTiler<PenroseRhombusType>>,
    mut edges: ResMut<EdgeLookup<PenroseRhombusType>>,
    asset_server: Res<AssetServer>,
    mut query: QuerySet<(
        Query<(Entity, &EdgeTile, &mut Rhombus, &TilerHandle, &Transform)>,
        Query<(Entity, &mut Rhombus, &TilerHandle, &Transform)>
    )>,
    mut commands: Commands
) {
    if tiler.tiles_added.len() == 0 {
        let (new_tile, new_entity) = tiler.spawn_random_tile_at_origin::<Rhombus>(&mut commands, &asset_server);
        edges.add_edges(&TileWithTransform::new(&new_tile, &Transform::default()), &new_entity);
        return;
    }

    let q0 = query.q0_mut();

    let mut edge_vec: Vec<(Entity, Mut<Rhombus>, &TilerHandle, &Transform)> = q0.iter_mut().map(
        |(entity, _, rhombus, handle, transform)| (entity, rhombus, handle, transform)
    ).collect();

    edge_vec.shuffle(&mut rand::thread_rng());

    assert!(edge_vec.len() > 0);
    for (existing_entity, rhombus, handle, transform) in edge_vec.iter() {
        println!("  Attempting to spawn tile on exisiting {:?} !", existing_entity);
        if !rhombus.has_free_sides() {
            // This can happen if this tile is marked to be removed but we're in the same tick that it happened
            continue;
        }

        let existing_tile = PlacedTile {
            tile: & **rhombus,
            transform: transform,
            handle: **handle
        };

        match tiler.spawn_random_tile_on(&existing_tile, &edges, &edge_vec, &mut commands, &asset_server) {
            Some((new_tile, new_transform, new_entity)) => {
                let new_tile = TileWithTransform::new(&new_tile, &new_transform);
                edges.add_edges(&new_tile, &new_entity);
                tiler.needs_check = true;
                println!("  Success!");
                break;
            },
            None => {
                println!("  Fail! Trying again");
            }
        }
    }
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system().label("setup"))
        .add_system(camera_control.system())
        .add_system(mark_sides_used.system().label("mark_sides_used"))
        .add_system(check_for_invalid_tile.system().label("check_for_invalid_tile").after("mark_sides_used"))
        .add_system(place_shapes.system().label("place_shapes").after("check_for_invalid_tile"))
        .add_system(history_buffer.system().after("place_shapes"))
        .run();
}

fn setup(mut commands: Commands) {
    /*let mut camera_bundle = ;
    camera_bundle.orthographic_projection = OrthographicProjection {
        left: -640.0, 
        right: 640.0, 
        bottom: -360.0, 
        top: 360.0, 
        near: 0.0, 
        far: 1000.0,
        scaling_mode: ScalingMode::None,
        scale: 1.0,
        ..OrthographicProjection::default()
    };*/

    let mut tiler = PenroseTiler::<PenroseRhombusType>::default();
    let edges = EdgeLookup::<PenroseRhombusType>::default();

    let matches = clap::App::new("penrose")
                    .about("penrose tiler")
                    .arg(clap::Arg::with_name("history-path")
                        .long("history-path")
                        .required(false)
                        .multiple(false)
                        .value_name("FILE")
                        .number_of_values(1))
                    .get_matches();

    
    if let Some(history_path) = matches.value_of("history-path") {
        let path = Path::new(history_path);
        let serialized_buffer = fs::read(&path).unwrap();
        tiler.history = bincode::deserialize(&serialized_buffer).unwrap();
        tiler.is_replay = true;
    }
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(tiler);
    commands.insert_resource(edges);
}

fn camera_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>
) {
    let mut transform = query.single_mut().unwrap();

    if keyboard_input.just_pressed(KeyCode::Minus) || 
        keyboard_input.just_pressed(KeyCode::NumpadSubtract
    ) {
        transform.scale += Vec3::new(0.1, 0.1, 0.1);
    } else if keyboard_input.just_pressed(KeyCode::Plus) || 
        keyboard_input.just_pressed(KeyCode::NumpadAdd
    ) {
        transform.scale -= Vec3::new(0.1, 0.1, 0.1);
    }
}

fn check_for_invalid_tile(
    mut tiler: ResMut<PenroseTiler<PenroseRhombusType>>,
    edges: ResMut<EdgeLookup<PenroseRhombusType>>,
    mut query: QuerySet<(
        Query<(Entity, &EdgeTile, &mut Rhombus, &TilerHandle, &Transform)>,
        Query<(Entity, &mut Rhombus, &TilerHandle, &Transform)>
    )>,
    commands: Commands
) {
    if !tiler.needs_check {
        return;
    }

    println!("START INVALID TILE REMOVAL *****************************************************");
    tiler.needs_check = false;

    let edge_vec: Vec<(Entity, Mut<Rhombus>, &TilerHandle, &Transform)> = query.q0_mut().iter_mut().map(
        |(entity, _, rhombus, handle, transform)| (entity, rhombus, handle, transform)
    ).collect();

    let mut found_handle = TilerHandle::default();
    for (entity, rhombus, handle, transform) in edge_vec.iter() {
        println!("Inspecting tile {:?}", entity);
        let tile = PlacedTile {
            tile: & **rhombus,
            transform: transform,
            handle: **handle
        };
        let allowed_placements = PenroseTiler::get_allowed_tiles_to_place(&tile, &edges, &edge_vec);
        println!("    allowed_placements len: {}", allowed_placements.len());
        let mut sides_not_found: u8 = 0xf;
        for (s, _) in allowed_placements {
            sides_not_found &= !(1 << s);
        }
        println!("    sides_not_found: {:b}", sides_not_found);

        let mut side = 0;
        while sides_not_found != 0 {
            if sides_not_found & 1 != 0 && 
                !rhombus.get_side_used(side) {
                // This side isn't used by another tile, but is invalid to place a tile on
                println!("    side {} of entity {:?} not used but has no allowed placements, found_handle {}", side, entity, handle.0);
                found_handle = **handle;
                break;
            }
            side += 1;
            sides_not_found >>= 1;
        }

        if found_handle.is_valid() {
            break;
        }
    }

    if found_handle.is_valid() {
        //let penrose_type = tiler.tiles_added.last().unwrap().penrose_type;
        println!("  Removing {:?}", tiler.tiles_added.last().unwrap());
        remove_rhombus(TilerHandle(tiler.tiles_added.len() - 1), query.q1_mut(), tiler, edges, commands);

        //tiler.tiles_added[found_handle.0].set_invalid_placement()
    }
    println!("END INVALID TILE REMOVAL*****************************************************");
}

fn mark_sides_used(
    edges: Res<EdgeLookup<PenroseRhombusType>>,
    mut query: QuerySet<(
        Query<(Entity, &mut Rhombus, &Transform), Added<Rhombus>>,
        Query<(Entity, &mut Rhombus, &Transform)>
    )>,
    mut commands: Commands
) {

    let mut has_new_rhombus = false;
    let mut edges_to_set: Vec<EdgeData<PenroseRhombusType>> = Vec::new();
    for (entity, mut rhombus, transform) in query.q0_mut().iter_mut() {
        if !has_new_rhombus {
            println!("START MARK SIDES PASS*****************************************************");
            has_new_rhombus = true;
        }

        let tile = TileWithTransform::new(&mut *rhombus, &transform);
        let edge_data = edges.get_tiles_for_all_edges(&tile);
        
        for edge in edge_data {
            let num_data = edge.data.len();
            for data in edge.data {
                println!("  considering: {:?} {}", data.entity, data.side);
                if data.entity == entity {
                    if num_data < 2 {
                        println!( "  (Skipping side {} on new entity {:?}, nothing connected to it)", data.side, data.entity);
                    } else {
                        edges_to_set.push(data);
                        println!( "  --> SETTING side {} used on new entity {:?}, because there's something else connected)", data.side, data.entity);
                    }
                } else {
                    println!( "  --> SETTING side {} used on entity {:?}", data.side, data.entity);
                    edges_to_set.push(data);
                }
            }
        }
    }
    
    for data in edges_to_set {
        let mut rhombus = query.q1_mut().get_component_mut::<Rhombus>(data.entity).unwrap();
        rhombus.set_side_used(data.side);

        if !rhombus.has_free_sides() {
            println!("  Removing edge tile {:?}!", data.entity);
            commands.entity(data.entity).remove::<EdgeTile>();
        }
    }

    if has_new_rhombus {
        println!("END MARK SIDES PASS*****************************************************");
        println!("");
    }
}


fn place_shapes(
    mut tiler: ResMut<PenroseTiler<PenroseRhombusType>>,
    mut edges: ResMut<EdgeLookup<PenroseRhombusType>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: QuerySet<(
        Query<(Entity, &EdgeTile, &mut Rhombus, &TilerHandle, &Transform)>,
        Query<(Entity, &mut Rhombus, &TilerHandle, &Transform)>
    )>,
    mut commands: Commands
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("START*****************************************************");
        if tiler.is_replaying() {
            println!("  SPAWNING FROM HISTORY*********************************");
            match tiler.spawn_next_tile_from_history::<Rhombus>(&mut commands, &asset_server, query.q1_mut()) {
                Some((new_tile, new_transform, new_entity)) => {
                    let tile = TileWithTransform::new(&new_tile, &new_transform);
                    edges.add_edges(&tile, &new_entity);
                    tiler.needs_check = true;
                },
                None => {}
            }
        } else {
            println!("  SPAWNING RANDOM**************************************");
            spawn_random_valid_tile(tiler, edges, asset_server, query, commands);
        }
        
        println!("END*****************************************************");
        println!("");
    } else if keyboard_input.just_pressed(KeyCode::U) && tiler.tiles_added.len() > 0 {
        println!("START*****************************************************");
        println!("  Removing {:?}", tiler.tiles_added.last().unwrap());
        remove_rhombus(TilerHandle(tiler.tiles_added.len() - 1), query.q1_mut(), tiler, edges, commands);
        println!("END*****************************************************");
        println!("");
    }
}

fn history_buffer(
    tiler: ResMut<PenroseTiler<PenroseRhombusType>>,
    keyboard_input: Res<Input<KeyCode>>
) {
    if keyboard_input.just_pressed(KeyCode::S) {
        let unix_epoch_millis = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        let path_string = format!("tiles_{}.til", unix_epoch_millis);
        let path = Path::new(&path_string);
        let mut tile_history = fs::File::create(&path).unwrap();
        tile_history.write_all(&bincode::serialize(&tiler.history).unwrap()).unwrap();
        println!("WROTE FILE {}", path_string);
    }
}