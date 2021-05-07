use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::math::Vec2;
use bevy::prelude::Commands;
use bevy::ecs::system::EntityCommands;
use bevy::ecs::component::Component;
use rand::prelude::*;
use std::ops::DerefMut;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use num_traits::FromPrimitive;


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

type PointList = Vec<Vec2>;

trait Tile : Component {
    type TileType;
    fn new_random() -> Self;
    fn get_num_sides() -> usize;
    fn insert_shape_component(&self, transform: Transform, entity_commands: &mut EntityCommands);
    fn spawn_dots_entities(&self, transform: Transform, commands: &mut Commands);
    fn has_free_sides(&self) -> bool;
    fn get_free_sides(&self) -> Vec<usize>;
    fn set_side_used(&mut self, side: usize);
    fn get_side_used(&self, side: usize) -> bool;
    fn get_points(&self) -> PointList;
    fn get_type(&self) -> Self::TileType;
    fn get_matching_side(&self, side: usize, other_type: Self::TileType) -> usize;
    fn get_connection_transform(&self, side: usize, other_type: Self::TileType) -> Transform;
}

struct TileWithTransform<'a, T: Tile>{
    tile: &'a mut T,
    transform: &'a Transform
}

impl<'a, T: Tile> TileWithTransform<'a, T> {
    fn new(tile: &'a mut T, transform: &'a Transform) -> Self {
        TileWithTransform {
            tile: tile,
            transform: transform
        }
    }
}

#[derive(Default)]
struct PenroseTiler {
    tiles_added: Vec<Entity>
}
struct EdgeTile;

impl PenroseTiler {
    fn spawn_tile_at<T: Tile>(&mut self, tile: T, transform: Transform, commands: &mut Commands) {
        tile.spawn_dots_entities(transform, commands);
        let mut entity = commands.spawn();
        tile.insert_shape_component(transform, &mut entity);
        entity.insert(tile);
        entity.insert(EdgeTile);

        self.tiles_added.push(entity.id());
    }
    fn spawn_tile_at_origin<T: Tile>(&mut self, tile: T, commands: &mut Commands) {
        self.spawn_tile_at(tile, Transform::identity(), commands);
    }

    fn spawn_tile_on<T: Tile>(&mut self, side: usize, mut tile: T, on_tile: TileWithTransform<T>, commands: &mut Commands) {
        assert!(!on_tile.tile.get_side_used(side));

        let transform = (*on_tile.transform) * on_tile.tile.get_connection_transform(side, tile.get_type());
        tile.set_side_used(on_tile.tile.get_matching_side(side, tile.get_type()));
        self.spawn_tile_at(tile, transform, commands);
        on_tile.tile.set_side_used(side);
    }
    fn spawn_random_tile_at_origin<T: Tile>(&mut self, commands: &mut Commands) {
        self.spawn_tile_at_origin(T::new_random(), commands);
    }
    fn spawn_random_tile_on<T: Tile>(&mut self, on_tile: TileWithTransform<T>, commands: &mut Commands) {
        let free_sides = on_tile.tile.get_free_sides();
        assert!(!free_sides.is_empty());
        let index = rand::thread_rng().gen_range(0..free_sides.len());
        let side = free_sides[index];
        let tile = T::new_random();
        self.spawn_tile_on(side, tile, on_tile, commands);
    }
}

#[derive(Clone, Copy, Primitive)]
enum PenroseRhombusType {
    Fat = 0,
    Skinny = 1,

    Count = 2
}

#[derive(Clone)]
struct Rhombus {
    small_angle: f32,
    leg_len: f32,
    color: Color,
    used_side_flags: u8,
    penrose_type: PenroseRhombusType
    //points: PointList
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
    //const LEFT_POINT: usize = 0;
    //const TOP_POINT: usize = 1;
    //const RIGHT_POINT: usize = 2;
    //const BOTTOM_POINT: usize = 3;

    const UPPER_LEFT_SIDE: usize = 0;
    const UPPER_RIGHT_SIDE: usize = 1;
    const LOWER_RIGHT_SIDE: usize = 2;
    const LOWER_LEFT_SIDE: usize = 3;

    const PENROSE_POINT_INDICES: [[usize; 4]; 2] = [
        //[0, 1+3, 2+7, 3+11],
        //[0, 1+3, 2+6, 3+10]

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

    const PENROSE_MATCHING_RULES: [[[usize; 4]; 2]; 2] = [
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
            penrose_type: PenroseRhombusType::Fat
        }
    }

    fn new_skinny() -> Self {
        Rhombus {
            small_angle: f32::to_radians(Rhombus::SKINNY_SMALL_ANGLE),
            leg_len: 100.0,
            color: Color::RED,
            used_side_flags: 0,
            penrose_type: PenroseRhombusType::Skinny
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

fn insert_edge_point(rhombus: &Rhombus, side: usize, transform: Transform, entity_commands: &mut EntityCommands) {
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

    //println!("scale: {}, angle: {}, point: {}, color: {:?}", scale, angle, point, color);

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
            transform
        )
    );
}

impl Tile for Rhombus {
    type TileType = PenroseRhombusType;

    fn new_random() -> Self {
        let tile_type = PenroseRhombusType::from_i32(rand::thread_rng().gen_range(0..(PenroseRhombusType::Count as i32))).unwrap();
        Rhombus::new(tile_type)
    }
    
    fn get_num_sides() -> usize {
        4
    }

    fn has_free_sides(&self) -> bool {
        return self.used_side_flags & 0xf != 0xf
    }

    fn get_free_sides(&self) -> Vec<usize> {
        let mut free_sides= Vec::<usize>::new();
        for side in 0..Rhombus::get_num_sides() {
            if !self.get_side_used(side) {
                free_sides.push(side);
            }
        }

        free_sides
    }

    fn get_side_used(&self, side: usize) -> bool {
        return (self.used_side_flags & (1 << side)) != 0;
    }

    fn set_side_used(&mut self, side: usize) {
        self.used_side_flags |= 1 << side;
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

    fn spawn_dots_entities(&self, mut transform: Transform, commands: &mut Commands) {
        transform.translation.z = 1.0;

        insert_edge_point(self, 0, transform, &mut commands.spawn());
        insert_edge_point(self, 1, transform, &mut commands.spawn());
        insert_edge_point(self, 2, transform, &mut commands.spawn());
        insert_edge_point(self, 3, transform, &mut commands.spawn());
    }

    fn get_type(&self) -> Self::TileType {
        self.penrose_type
    }

    fn get_matching_side(&self, side: usize, other_type: Self::TileType) -> usize {
        assert!(side < 4);
        Rhombus::PENROSE_MATCHING_RULES[self.get_type() as usize][other_type as usize][side]
    }

    fn get_connection_transform(&self, side: usize, other_type: Self::TileType) -> Transform {
        //println!("{} {} {}", self.penrose_type as usize, other_type as usize, side);
        ROTATION_TRANSFORMS[self.penrose_type as usize][other_type as usize][side]
    }
}

/*fn connect_tiles<T: Tile>(tile_existing: &mut T, tile_existing_side: usize, tile_to_connect: &mut T) {
    assert!(!tile_existing.get_side_used(tile_existing_side));
    
    let side_to_match = tile_existing.get_matching_side(tile_existing_side, tile_to_connect.get_type());
    assert!(!tile_to_connect.get_side_used(side_to_match));

    tile_existing.set_side_used(tile_existing_side);
    tile_to_connect.set_side_used(side_to_match);
}*/

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_system(shapes.system())
        .run();
}

fn setup(mut commands: Commands) {

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut tiler = PenroseTiler::default();
    tiler.spawn_tile_at_origin(Rhombus::new_fat(), &mut commands);

    commands.insert_resource(tiler);

    /*let r1 = Rhombus::new_skinny();
    let r2 = Rhombus::new_fat();
    let r3 = Rhombus::new_fat();
    let r4 = Rhombus::new_fat();
    let r5 = Rhombus::new_fat();

    let mut entity = commands.spawn();
    r1.insert_shape_component(Transform::identity(), &mut entity);
    entity.insert(r1.clone());
    r1.spawn_dots_entities(Transform::identity(), &mut commands);

    let mut entity = commands.spawn();
    let transform = r1.get_connection_transform(Rhombus::UPPER_LEFT_SIDE, r2.get_type());
    r2.insert_shape_component(transform, &mut entity);
    entity.insert(r2.clone());
    r2.spawn_dots_entities(transform, &mut commands);

    let mut entity = commands.spawn();
    let transform = r1.get_connection_transform(Rhombus::UPPER_RIGHT_SIDE, r3.get_type());
    r3.insert_shape_component(transform, &mut entity);
    entity.insert(r3.clone());
    r3.spawn_dots_entities(transform, &mut commands);

    {
        let onto_upper_right = Rhombus::new_skinny();
        let mut entity = commands.spawn();
        let onto_upper_right_transform = transform * r3.get_connection_transform(Rhombus::UPPER_RIGHT_SIDE, onto_upper_right.get_type());
        onto_upper_right.insert_shape_component(onto_upper_right_transform, &mut entity);
        entity.insert(onto_upper_right.clone());
        onto_upper_right.spawn_dots_entities(onto_upper_right_transform, &mut commands);
    }

    {
        let onto_upper_left = Rhombus::new_fat();
        let mut entity = commands.spawn();
        let onto_upper_left_transform = transform * r3.get_connection_transform(Rhombus::UPPER_LEFT_SIDE, onto_upper_left.get_type());
        onto_upper_left.insert_shape_component(onto_upper_left_transform, &mut entity);
        entity.insert(onto_upper_left.clone());
        onto_upper_left.spawn_dots_entities(onto_upper_left_transform, &mut commands);
    }

    let mut entity = commands.spawn();
    let transform = r1.get_connection_transform(Rhombus::LOWER_RIGHT_SIDE, r4.get_type());
    r4.insert_shape_component(transform, &mut entity);
    entity.insert(r4.clone());
    r4.spawn_dots_entities(transform, &mut commands);

    let mut entity = commands.spawn();
    let transform = r1.get_connection_transform(Rhombus::LOWER_LEFT_SIDE, r5.get_type());
    r5.insert_shape_component(transform, &mut entity);
    entity.insert(r5.clone());
    r5.spawn_dots_entities(transform, &mut commands);
    */
}

fn shapes(mut tiler: ResMut<PenroseTiler>,
    mut query: Query<(Entity, &EdgeTile, &mut Rhombus, &Transform)>, 
    mut commands: Commands) {

    if tiler.tiles_added.len() >= 4 {
        return;
    }

    for (entity, _, mut rhombus, transform) in query.iter_mut() {
        if rhombus.has_free_sides() {
            println!("Spawning tile!");
            let t = TileWithTransform::new(&mut *rhombus, &transform);
            tiler.spawn_random_tile_on(t, &mut commands);
        }
        
        if !rhombus.has_free_sides() {
            println!("Removing edge tile!");
            commands.entity(entity).remove::<EdgeTile>();
        }
    }
}