use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::math::Vec2;
use bevy::prelude::Commands;
use bevy_prototype_lyon::entity::{ShapeBundle, ShapeColors};
use rand::prelude::*;

#[macro_use]
extern crate lazy_static;

type PointList = Vec<Vec2>;
type Radians = f32;
type SideFlags = u8;

trait Tile {
    fn get_bundle_with_transform(&self, transform: Transform) -> ShapeBundle;
    fn get_bundle(&self) -> ShapeBundle {
        self.get_bundle_with_transform(Transform::default())
    }
    fn has_free_sides(&self) -> bool;
    fn get_free_sides(&self) -> SideFlags;
    fn set_side_used(&mut self, side: usize);
    fn get_side_used(&self, size: usize) -> bool;
    fn get_points(&self) -> PointList;
    //fn get_connection_transform(&self, other_angle: f32, side: usize) -> Option<Transform>;
}

#[derive(Clone, Copy)]
enum PenroseRhombusType {
    Fat = 0,
    Skinny
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

fn make_rotation_transform(angle: f32, translation: Vec2) -> Transform {
    Transform::from_matrix(
        Mat4::from_rotation_translation(
            Quat::from_rotation_z(angle),
            translation.extend(0.0)
        )
    )
}

lazy_static! {
    static ref ROTATION_TRANSFORMS: Vec<Vec<Vec<Transform>>> = {
        // Create a rhombus centered at the origin
        let r_fat = Rhombus::new_fat();
        let r_skinny = Rhombus::new_skinny();
        let fat_points = r_fat.get_points();
        let skinny_points = r_skinny.get_points();

        let fat_long_diag_len = r_fat.leg_len * (2.0 + 2.0 * r_fat.small_angle.cos()).sqrt();
        let fat_half_long_diag = fat_long_diag_len / 2.0;
        let fat_small_diag_len = r_fat.leg_len * (2.0 - 2.0 * r_fat.small_angle.cos()).sqrt();
        let fat_half_small_diag = fat_small_diag_len / 2.0;

        let skinny_long_diag_len = r_skinny.leg_len * (2.0 + 2.0 * r_skinny.small_angle.cos()).sqrt();
        let skinny_half_long_diag = skinny_long_diag_len / 2.0;
        let skinny_small_diag_len = r_skinny.leg_len * (2.0 - 2.0 * r_skinny.small_angle.cos()).sqrt();
        let skinny_half_small_diag = skinny_small_diag_len / 2.0;

        let mut v = Vec::<Vec::<Vec::<Transform>>>::new();
        v.push(Vec::<Vec::<Transform>>::new());
        {
            let fat = &mut v[0];
            {
                fat.push(Vec::<Transform>::new());
                {
                    let fat_fat_angle = f32::to_radians(180.0) - Rhombus::FAT_SMALL_ANGLE / 2.0 - Rhombus::FAT_LARGE_ANGLE / 2.0;
                    let half_small_y_translate = Vec2::new(0.0, fat_half_small_diag);
                    let fat_fat_sides = &mut fat[0];
                    {
                        // Fat -> Fat on side 0
                        fat_fat_sides.push(
                            make_rotation_transform(fat_fat_angle, fat_points[Rhombus::TOP_POINT] + half_small_y_translate)
                        );
                    }
                    {
                        // Fat -> Fat on side 1
                        fat_fat_sides.push(
                            make_rotation_transform(-fat_fat_angle, fat_points[Rhombus::TOP_POINT] + half_small_y_translate)
                        );
                    }
                    {
                        // Fat -> Fat on side 2
                        fat_fat_sides.push(
                            make_rotation_transform(fat_fat_angle, fat_points[Rhombus::BOTTOM_POINT] - half_small_y_translate)
                        );
                    }
                    {
                        // Fat -> Fat on side 3
                        fat_fat_sides.push(
                            make_rotation_transform(-fat_fat_angle, fat_points[Rhombus::BOTTOM_POINT] - half_small_y_translate)
                        );
                    }
                }
                fat.push(Vec::<Transform>::new());
                {
                    let fat_skinny_angle = f32::to_radians(180.0) + Rhombus::FAT_SMALL_ANGLE;
                    let half_small_y_translate = Vec2::new(0.0, skinny_half_small_diag);
                    let fat_skinny_sides = &mut fat[1];
                    {
                        // Fat -> Skinny on side 0
                        fat_skinny_sides.push(
                            make_rotation_transform(fat_skinny_angle, fat_points[Rhombus::LEFT_POINT] + half_small_y_translate)
                        );
                    }
                    {
                        // Fat -> Skinny on side 1
                        fat_skinny_sides.push(
                            make_rotation_transform(-fat_skinny_angle, fat_points[Rhombus::TOP_POINT] + half_small_y_translate)
                        );
                    }
                    {
                        // Fat -> Skinny on side 2
                        fat_skinny_sides.push(
                            make_rotation_transform(fat_skinny_angle, fat_points[Rhombus::BOTTOM_POINT] - half_small_y_translate)
                        );
                    }
                    {
                        // Fat -> Skinny on side 3
                        fat_skinny_sides.push(
                            make_rotation_transform(-fat_skinny_angle, fat_points[Rhombus::LEFT_POINT] - half_small_y_translate)
                        );
                    }
                }
            }
        }
        v
/*
                // Fat
                [
                    // Fat
                    [
                        // Fat -> Fat UPPER_LEFT_SIDE
                        Transform::from_matrix(Mat4::from_rotation_translation( Quat, translation: Vec3) -> Mat4
                        Transform::identity(),
                        Transform::identity(),
                        Transform::identity()
                    ],
        
                    // Skinny
                    [
                        Transform::identity(),
                        Transform::identity(),
                        Transform::identity(),
                        Transform::identity()
                    ]
                ],
        
                // Skinny
                [
                    // Fat
                    [
                        // Skinny -> Fat UPPER_LEFT_SIDE
                        Transform::identity(),
                        Transform::identity(),
                        Transform::identity(),
                        Transform::identity()
                    ],
        
                    // Skinny
                    [
                        Transform::identity(),
                        Transform::identity(),
                        Transform::identity(),
                        Transform::identity()
                    ]
                ]
            ];*/
    };
}


impl Rhombus {
    const FAT_SMALL_ANGLE: f32 = 72.0;
    const SKINNY_SMALL_ANGLE: f32 = 36.0;
    const FAT_LARGE_ANGLE: f32 = 180.0 - Rhombus::FAT_SMALL_ANGLE;
    const SKINNY_LARGE_ANGLE: f32 = 180.0 - Rhombus::SKINNY_SMALL_ANGLE;
    const LEFT_POINT: usize = 0;
    const TOP_POINT: usize = 1;
    const RIGHT_POINT: usize = 2;
    const BOTTOM_POINT: usize = 3;

    const UPPER_LEFT_SIDE: usize = 0;
    const UPPER_RIGHT_SIDE: usize = 1;
    const LOWER_RIGHT_SIDE: usize = 2;
    const LOWER_LEFT_SIDE: usize = 3;

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

    /*const CONNECT_INFO: [[[Transform; 4]; 2]; 2] = [
        // Fat
        [
            // Fat
            [
                // Fat -> Fat UPPER_LEFT_SIDE
                Transform::from_matrix(Mat4::from_rotation_translation( Quat, translation: Vec3) -> Mat4
                Transform::identity(),
                Transform::identity(),
                Transform::identity()
            ],

            // Skinny
            [
                Transform::identity(),
                Transform::identity(),
                Transform::identity(),
                Transform::identity()
            ]
        ],

        // Skinny
        [
            // Fat
            [
                // Skinny -> Fat UPPER_LEFT_SIDE
                Transform::identity(),
                Transform::identity(),
                Transform::identity(),
                Transform::identity()
            ],

            // Skinny
            [
                Transform::identity(),
                Transform::identity(),
                Transform::identity(),
                Transform::identity()
            ]
        ]
    ];*/

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

    fn get_matching_side(&self, side: usize, other_type: PenroseRhombusType) -> usize {
        assert!(side < 4);
        Rhombus::PENROSE_MATCHING_RULES[self.penrose_type as usize][other_type as usize][side]
    }
    fn get_connection_transform(&self, other: &Rhombus, side: usize) -> Transform {
        let self_large_angle = f32::to_radians(180.0) - self.small_angle;

        /*if self.get_side_used(side) {
            return None;
        }

        let other_side = self.get_matching_side(side, other.penrose_type);

        if other.get_side_used(side) {
            return None;
        }*/

        let points = self.get_points();
        let point = match side {
            Rhombus::UPPER_LEFT_SIDE | Rhombus::UPPER_RIGHT_SIDE => points[Rhombus::TOP_POINT],
            Rhombus::LOWER_LEFT_SIDE | Rhombus::LOWER_LEFT_SIDE => points[Rhombus::BOTTOM_POINT],
            _ => panic!("invalid side")
        };

        //let rotation = 

        /*else if side == Rhombus::UPPER_RIGHT_SIDE {
            let rotation_angle;
            if self.color == Color::BLUE {
                if other.get_side_used(Rhombus::LOWER_RIGHT_SIDE) {
                    return None;
                }
                //rotation_angle = f32::to_radians(-36.0);
                //rotation_angle = -(self.small_angle - other.small_angle) / 2.0;
                //let other_large_angle = f32::to_radians(180.0) - other.small_angle;
                rotation_angle = -(self_large_angle) / 2.0;
                println!("rotation angle: {}", rotation_angle.to_degrees());
            } else {
                assert!(self.color == Color::RED);
                if other.color == Color::BLUE {
                    if other.get_side_used(Rhombus::LOWER_LEFT_SIDE) {
                        return None
                    }
                    rotation_angle = -(self_large_angle) / 2.0;
                    println!("rotation angle: {}", rotation_angle.to_degrees());
                } else  {
                    return None;
                } 
            }
            let rotation = Quat::from_rotation_z(rotation_angle);
            let short_diag_len = other.leg_len * (2.0 - 2.0 * other.small_angle.cos()).sqrt();
            let half_short_diag = short_diag_len / 2.0;
        
            let dir = Vec3::Y;
            let rotated_extended = half_short_diag * (rotation * dir);
            println!("rotated_extended: {}, top: {}", rotated_extended, self.get_points()[Rhombus::TOP_INDEX].extend(0.0));
            let translated = rotated_extended + self.get_points()[Rhombus::TOP_INDEX].extend(0.0);
            let mut transform = Transform::from_translation(translated);
            transform.rotate(rotation);
            Some(transform)
        } else {
            None
        }*/
        Transform::identity()
    }
}

impl Default for Rhombus {
    fn default() -> Self {
        Rhombus::new_skinny()
    }
}

impl Tile for Rhombus {
    fn has_free_sides(&self) -> bool {
        return self.used_side_flags & 0xf != 0xf
    }
    fn get_free_sides(&self) -> SideFlags {
        return !(self.used_side_flags & 0xf);
    }
    fn get_side_used(&self, size: usize) -> bool {
        return (self.used_side_flags & (1 << size)) != 0;
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

    /*fn get_side_coordinates(&self, side:i32) -> (Vec2, Vec2) {
        let uside = side as usize;
        assert!(side >= 0 && uside < self.points.len());

        let first_point = uside;
        let second_point = (uside + 1) % self.points.len();
        (self.points[first_point], self.points[second_point])
    }*/

    fn get_bundle_with_transform(&self, transform: Transform) -> ShapeBundle {
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
    }

    fn get_bundle(&self) -> ShapeBundle {
        self.get_bundle_with_transform(Transform::default())
    }

}


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

    /*let mut rng = rand::thread_rng();
    /*let mut fat_transform = Transform::from_xyz(-150.0, 0.0, 0.0);
    fat_transform.rotate(Quat::from_rotation_z(f32::to_radians(90.0)));*/

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    for _ in 1..100 {
        let skinny = Rhombus::new_skinny();
        let fat = Rhombus::new_fat();
        let rand_x1: f32 = rng.gen::<f32>() * 1000.0 - 500.0;
        let rand_y1: f32 = rng.gen::<f32>() * 500.0 - 250.0;
        let rand_x2: f32 = rng.gen::<f32>() * 1000.0 - 500.0;
        let rand_y2: f32 = rng.gen::<f32>() * 500.0 - 250.0;
        commands.spawn_bundle(
            skinny.get_bundle_with_transform(Transform::from_xyz(rand_x1, rand_y1, 0.0))
        ).insert(skinny);
        commands.spawn_bundle(
            fat.get_bundle_with_transform(Transform::from_xyz(rand_x2, rand_y2, 0.0))
        ).insert(fat);
    }*/

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let fat = Rhombus::new_fat();
    let skinny = Rhombus::new_skinny();

    commands.spawn_bundle(
        fat.get_bundle()
    ).insert(fat.clone());

    let skinny1_transform = fat.get_connection_transform(&skinny, Rhombus::UPPER_RIGHT_SIDE);
    commands.spawn_bundle(
        //fat.get_bundle_with_transform(fat.get_connection_transform(&fat, Rhombus::UPPER_RIGHT_SIDE).unwrap())
        skinny.get_bundle_with_transform(skinny1_transform)
    ).insert(skinny.clone());

    /*let mut fat_transform = skinny.get_connection_transform(&fat, Rhombus::UPPER_RIGHT_SIDE).unwrap();
    fat_transform = fat_transform * skinny1_transform;
    commands.spawn_bundle(
        fat.get_bundle_with_transform(fat_transform.clone())
    ).insert(fat.clone());*/

    /*let fat = Rhombus::new_fat();
    commands.spawn_bundle(
        fat.get_bundle()
    ).insert(fat);
    let fat_test = Rhombus::new_fat();
    let rhomb2 = Rhombus::new_fat();
    let rotation_angle = -(fat_test.small_angle - rhomb2.small_angle) / 2.0;
    let rotation = Quat::from_rotation_z(rotation_angle);
    let long_diag_len = rhomb2.leg_len * (2.0 + 2.0 * rhomb2.small_angle.cos()).sqrt();
    let half_long_diag = long_diag_len / 2.0;

    let dir = Vec3::X;
    let rotated_extended = half_long_diag * (rotation * dir);
    let translated = rotated_extended + fat_test.get_points()[Rhombus::TOP_INDEX].extend(0.0);
     let mut transform = Transform::from_translation(translated);
    transform.rotate(
        rotation
    );
    println!("transform: {:?}, rotation_angle: {}", transform, f32::to_degrees(rotation_angle));
    //transform.tra
    commands.spawn_bundle(
        rhomb2.get_bundle_with_transform(transform.clone())
    ).insert(rhomb2.clone());*/

    //println!("fat top: {}, skinny left: {}", fat_test.get_points()[Rhombus::TOP_INDEX], transform * rhomb2.get_points()[Rhombus::LEFT_INDEX].extend(0.0))
}

fn shapes(mut query: Query<(&Rhombus, &mut Transform)>, mut commands: Commands) {
    //let mut rng = rand::thread_rng();
    /*for (rhomb, mut transform) in query.iter_mut() {
        //if rhomb.color == Color::RED {
        //    continue;
        //}
        let (a, cur_angle) = transform.rotation.to_axis_angle();
        let mut new_angle = cur_angle + f32::to_radians(1.0);
        if new_angle.to_degrees() >= 360.0 {
            new_angle = 0.0;
        }
        transform.rotation = Quat::from_rotation_z(new_angle);
        //println!("translation {}, new_angle: {}, cur_angle {}, about {}", transform.rotation, new_angle.to_degrees(), cur_angle.to_degrees(), a)
    }*/
}