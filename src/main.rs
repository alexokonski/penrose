use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::math::Vec2;
use bevy::prelude::Commands;
use bevy::ecs::system::EntityCommands;
use bevy_prototype_lyon::entity::{ShapeBundle, ShapeColors};

#[macro_use]
extern crate lazy_static;

type PointList = Vec<Vec2>;
type SideFlags = u8;

trait Tile {
    type TileType;
    /*fn insert_shapes_with_transform(&self, transform: Transform, commands: &mut Commands);
        self.insert_shapes_with_transform(Transform::default(), commands)
    }*/
    fn insert_shape_component(&self, transform: Transform, entity_commands: &mut EntityCommands);
    fn spawn_dots_entities(&self, transform: Transform, commands: &mut Commands);
    fn has_free_sides(&self) -> bool;
    fn get_free_sides(&self) -> SideFlags;
    fn set_side_used(&mut self, side: usize);
    fn get_side_used(&self, side: usize) -> bool;
    fn get_points(&self) -> PointList;
    fn get_type(&self) -> Self::TileType;
    fn get_matching_side(&self, side: usize, other_type: Self::TileType) -> usize;
    fn get_connection_transform(&self, side: usize, other_type: Self::TileType) -> Transform;
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

fn make_rotation_transform_good(angle: f32, translation: Vec2, distance_to_center_from_translation: f32) -> Transform {
    /*
    // Create a vector that will point to the center of the figure we are translating
    let dir = Vec3::X;
    let rotation = Quat::from_rotation_z(angle);
    let vec_to_center_point = rotation * dir * distance_to_center_from_translation;
    
    // Now translate that point to its final position
    let center_point = vec_to_center_point + Vec3::from((translation, 0.0));

    Transform::from_matrix(
        Mat4::from_rotation_translation(
            rotation,
            center_point
        )
    )
    */

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
        //let skinny_half_long_diag = skinny_long_diag_len / 2.0;
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
                            make_rotation_transform_good(fat_vert_angle, fat_points[r_fat.get_top_index()], fat_half_small_diag)
                        );
                    }
                    {
                        // Fat on Fat on side 1
                        fat_fat_sides.push(
                            make_rotation_transform_good(-fat_vert_angle, fat_points[r_fat.get_top_index()], fat_half_small_diag)
                        );
                    }
                    {
                        // Fat on Fat on side 2
                        fat_fat_sides.push(
                            make_rotation_transform_good(fat_vert_angle, fat_points[r_fat.get_bottom_index()], -fat_half_small_diag)
                        );
                    }
                    {
                        // Fat on Fat on side 3
                        fat_fat_sides.push(
                            make_rotation_transform_good(-fat_vert_angle, fat_points[r_fat.get_bottom_index()], -fat_half_small_diag)
                        );
                    }
                }
                fat.push(Vec::<Transform>::new());
                {
                    let skinny_fat_horizontal_angle = f32::to_radians(180.0 + Rhombus::FAT_SMALL_ANGLE / 2.0 - Rhombus::SKINNY_SMALL_ANGLE / 2.0);
                    let skinny_fat_vert_angle = f32::to_radians(180.0 - Rhombus::SKINNY_LARGE_ANGLE / 2.0 - Rhombus::FAT_LARGE_ANGLE / 2.0);
                    println!("skinny_fat_horizontal_angle {}", f32::to_degrees(skinny_fat_horizontal_angle));
                    //let half_small_large_x_translate = Vec2::new(fat_half_long_diag, 0.0);
                    //let half_small_y_translate = Vec2::new(0.0, skinny_half_small_diag);
                    let skinny_fat_sides = &mut fat[1];
                    {
                        // Skinny onto fat on side 0
                        skinny_fat_sides.push(
                            make_rotation_transform_good(skinny_fat_horizontal_angle, fat_points[r_fat.get_left_index()], -skinny_half_small_diag)
                        );
                    }
                    {
                        // Skinny onto fat on side 1
                        skinny_fat_sides.push(
                            make_rotation_transform_good(-skinny_fat_vert_angle, fat_points[r_fat.get_top_index()], skinny_half_small_diag)
                        );
                    }
                    {
                        // Skinny onto fat on side 2
                        skinny_fat_sides.push(
                            make_rotation_transform_good(skinny_fat_vert_angle + f32::to_radians(180.0), fat_points[r_fat.get_bottom_index()], skinny_half_small_diag)
                        );
                    }
                    {
                        // Skinny onto fat on side 3
                        skinny_fat_sides.push(
                            make_rotation_transform_good(-f32::to_radians(Rhombus::SKINNY_SMALL_ANGLE / 2.0), fat_points[r_fat.get_left_index()], -skinny_half_small_diag)
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
                    let fat_skinny_angle = f32::to_radians(180.0 + Rhombus::FAT_LARGE_ANGLE / 2.0 - Rhombus::SKINNY_LARGE_ANGLE / 2.0);
                    let half_small_y_translate = Vec2::new(0.0, fat_half_small_diag);
                    let skinny_fat_sides = &mut skinny[0];
                    {
                        // Fat on Skinny on side 0
                        skinny_fat_sides.push(
                            make_rotation_transform_good(fat_skinny_angle, skinny_points[r_skinny.get_left_index()], -fat_half_small_diag)
                        );
                    }
                    {
                        // Fat on Skinny on side 1
                        skinny_fat_sides.push(
                            make_rotation_transform_good(-fat_skinny_angle, skinny_points[r_skinny.get_top_index()], fat_half_small_diag)
                        );
                    }
                    {
                        // Fat on Skinny on side 2
                        skinny_fat_sides.push(
                            make_rotation_transform_good(fat_skinny_angle, skinny_points[r_skinny.get_bottom_index()], fat_half_small_diag)
                        );
                    }
                    {
                        // Fat on Skinny on side 3
                        skinny_fat_sides.push(
                            make_rotation_transform_good(-fat_skinny_angle, skinny_points[r_skinny.get_bottom_index()], fat_half_small_diag)
                        );
                    }
                }
                skinny.push(Vec::<Transform>::new());
                {
                    let skinny_skinny_angle = f32::to_radians(180.0 + 180.0 - Rhombus::SKINNY_LARGE_ANGLE / 2.0 - Rhombus::SKINNY_LARGE_ANGLE / 2.0);
                    let half_small_y_translate = Vec2::new(0.0, skinny_half_small_diag);
                    let skinny_skinny_sides = &mut skinny[1];
                    {
                        // Skinny -> Skinny on side 0
                        skinny_skinny_sides.push(
                            make_rotation_transform_good(skinny_skinny_angle, skinny_points[r_skinny.get_top_index()], -skinny_half_small_diag)
                        );
                    }
                    {
                        // Skinny -> Skinny on side 1
                        skinny_skinny_sides.push(
                            make_rotation_transform_good(-skinny_skinny_angle, skinny_points[r_skinny.get_top_index()], -skinny_half_small_diag)
                        );
                    }
                    {
                        // Skinny -> Skinny on side 2
                        skinny_skinny_sides.push(
                            make_rotation_transform_good(skinny_skinny_angle, skinny_points[r_skinny.get_bottom_index()], skinny_half_small_diag)
                        );
                    }
                    {
                        // Skinny -> Skinny on side 3
                        skinny_skinny_sides.push(
                            make_rotation_transform_good(-skinny_skinny_angle, skinny_points[r_skinny.get_bottom_index()], skinny_half_small_diag)
                        );
                    }
                }
            }
        }
        v
    };
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

fn calc_point_on_angle(scale: f32, angle: f32, leg_len: f32, origin: Vec2) -> Vec2 {
    let hyp = leg_len * scale;
    Vec2::new(origin.x + hyp * angle.cos(), origin.y + hyp * angle.sin())
}

fn get_tooth_points(scale: f32, angle: f32, leg_len: f32, origin: Vec2, offset: f32, inverted: bool) -> PointList {
    let start_tooth = calc_point_on_angle(scale, angle / 2.0, leg_len, origin);
    let end_tooth =  calc_point_on_angle(scale + 0.10, angle / 2.0, leg_len, origin);
    let mut tooth_point = start_tooth + ((end_tooth - start_tooth) / 2.0);
    tooth_point.y += if inverted { -offset } else { offset };

    vec! [
        start_tooth,
        tooth_point,
        end_tooth
    ]
}

fn get_peg_points(scale: f32, angle: f32, leg_len: f32, origin: Vec2, offset: f32, inverted: bool) -> PointList {
    let start_peg = calc_point_on_angle(scale, angle / 2.0, leg_len, origin);
    let end_peg =  calc_point_on_angle(scale + 0.10, angle / 2.0, leg_len, origin);
    let mut peg_point_1 = start_peg.clone();
    peg_point_1.y += if inverted { -offset } else { offset };
    let mut peg_point_2 = end_peg.clone();
    peg_point_2.y += if inverted { -offset } else { offset };

    vec! [
        start_peg,
        peg_point_1,
        peg_point_2,
        end_peg
    ]
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

    println!("scale: {}, angle: {}, point: {}, color: {:?}", scale, angle, point, color);

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
    fn has_free_sides(&self) -> bool {
        return self.used_side_flags & 0xf != 0xf
    }

    fn get_free_sides(&self) -> SideFlags {
        return !(self.used_side_flags & 0xf);
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

        //let hyp = self.leg_len * 0.66;
        //let seg = Vec2::new(left.x + hyp * (self.small_angle / 2.0).cos(), left.y + hyp * (self.small_angle / 2.0).sin());

        match self.penrose_type {
            PenroseRhombusType::Fat => {
                let mut points = PointList::new();
                points.push(left);
                //points.append(&mut get_tooth_points(0.44, self.small_angle, self.leg_len, left, 12.0, true));
                points.push(top);
                //points.append(&mut get_peg_points(-0.44, -self.small_angle, self.leg_len, right, 12.0, true));
                points.push(right);
                //points.append(&mut get_peg_points(-0.44, self.small_angle, self.leg_len, right, -10.0, false));
                points.push(bottom);
                //points.append(&mut get_tooth_points(0.44, -self.small_angle, self.leg_len, left, -10.0, false));

                points
                /*vec![
                    left,
                    top,
                    right,
                    bottom
                ]*/
            },
            PenroseRhombusType::Skinny => {                
                let mut points = PointList::new();
                points.push(left);
                //points.append(&mut get_tooth_points(0.44, self.small_angle, self.leg_len, left, 10.0, false));
                points.push(top);
                //points.append(&mut get_tooth_points(-0.44, -self.small_angle, self.leg_len, right, 12.0, true));
                points.push(right);
                //points.append(&mut get_peg_points(-0.44, self.small_angle, self.leg_len, right, -10.0, false));
                points.push(bottom);
                //points.append(&mut get_peg_points(0.44, -self.small_angle, self.leg_len, left, -12.0, true));
                
                points
                /*vec![
                    left,
                    start_tooth,
                    tooth_point,
                    end_tooth,
                    top,
                    right,
                    bottom
                ]*/

            }
        }
    }

    /*fn get_side_coordinates(&self, side:i32) -> (Vec2, Vec2) {
        let uside = side as usize;
        assert!(side >= 0 && uside < self.points.len());

        let first_point = uside;
        let second_point = (uside + 1) % self.points.len();
        (self.points[first_point], self.points[second_point])
    }*/

    /*fn insert_shapes_with_transform(&self, transform: Transform, commands: &mut Commands) {
        commands.insert(GeometryBuilder::build_as(
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
        ))
    }*/

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

        /*insert_edge_point(
            0.33, 
            dot_radius, 
            self.small_angle / 2.0,
            points[self.get_right_index()],
            Rhombus::PENROSE_EDGE_DOT_COLORS[self.penrose_type as usize][Rhombus::UPPER_RIGHT_SIDE], 
            transform, 
            entity_commands);

        insert_edge_point(
            0.33, 
            dot_radius, 
            -self.small_angle / 2.0,
            points[self.get_left_index()],
            Rhombus::PENROSE_EDGE_DOT_COLORS[self.penrose_type as usize][Rhombus::UPPER_LEFT_SIDE], 
            transform, 
            entity_commands);

        insert_edge_point(
            0.33, 
            dot_radius, 
            -self.small_angle / 2.0,
            points[self.get_left_index()],
            Rhombus::PENROSE_EDGE_DOT_COLORS[self.penrose_type as usize][Rhombus::UPPER_LEFT_SIDE], 
            transform, 
            entity_commands);*/

        /*entity_commands.insert_bundle(
            GeometryBuilder::build_as(
                &shapes::Circle {
                    radius: dot_radius,
                    center: get_edge_point(0.66, dot_radius, self.small_angle / 2.0, points[self.get_right_index()])
                },
                ShapeColors::outlined(Color::GREEN, Color::BLACK),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(2.0),
                },
                transform
            )
        );*/
    }


    fn get_type(&self) -> Self::TileType {
        self.penrose_type
    }

    fn get_matching_side(&self, side: usize, other_type: Self::TileType) -> usize {
        assert!(side < 4);
        Rhombus::PENROSE_MATCHING_RULES[self.get_type() as usize][other_type as usize][side]
    }

    fn get_connection_transform(&self, side: usize, other_type: Self::TileType) -> Transform {
        println!("{} {} {}", self.penrose_type as usize, other_type as usize, side);
        ROTATION_TRANSFORMS[self.penrose_type as usize][other_type as usize][side]
        /*if self.get_side_used(side) {
            return None;
        }

        let other_side = self.get_matching_side(side, other.penrose_type);

        if other.get_side_used(side) {
            return None;
        }*/

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
    }
}

fn connect_tiles<T: Tile>(tile_existing: &mut T, tile_existing_side: usize, tile_to_connect: &mut T) {
    assert!(!tile_existing.get_side_used(tile_existing_side));
    
    let side_to_match = tile_existing.get_matching_side(tile_existing_side, tile_to_connect.get_type());
    assert!(!tile_to_connect.get_side_used(side_to_match));

    tile_existing.set_side_used(tile_existing_side);
    tile_to_connect.set_side_used(side_to_match);
}

//fn connect_and_translate

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

    let r1 = Rhombus::new_skinny();
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

    /*let mut entity = commands.spawn();
    let transform = r1.get_connection_transform(Rhombus::UPPER_RIGHT_SIDE, r3.get_type());
    r3.insert_shape_component(transform, &mut entity);
    entity.insert(r3.clone());
    r3.spawn_dots_entities(transform, &mut commands);

    let mut entity = commands.spawn();
    let transform = r1.get_connection_transform(Rhombus::LOWER_RIGHT_SIDE, r4.get_type());
    r4.insert_shape_component(transform, &mut entity);
    entity.insert(r4.clone());
    r4.spawn_dots_entities(transform, &mut commands);

    let mut entity = commands.spawn();
    let transform = r1.get_connection_transform(Rhombus::LOWER_LEFT_SIDE, r5.get_type());
    r5.insert_shape_component(transform, &mut entity);
    entity.insert(r5.clone());
    r5.spawn_dots_entities(transform, &mut commands);*/

    
    /*let r2_transform = r1.get_connection_transform(Rhombus::UPPER_LEFT_SIDE, r2.get_type());
    commands.spawn_bundle(
        //fat.get_bundle_with_transform(fat.get_connection_transform(&fat, Rhombus::UPPER_RIGHT_SIDE).unwrap())
        r2.get_bundle_with_transform(r2_transform)
    ).insert(r2.clone());

    let r3_transform = r1.get_connection_transform(Rhombus::UPPER_RIGHT_SIDE, r3.get_type());
    commands.spawn_bundle(
        //fat.get_bundle_with_transform(fat.get_connection_transform(&fat, Rhombus::UPPER_RIGHT_SIDE).unwrap())
        r3.get_bundle_with_transform(r3_transform)
    ).insert(r3.clone());

    let r4_transform = r1.get_connection_transform(Rhombus::LOWER_RIGHT_SIDE, r4.get_type());
    commands.spawn_bundle(
        //fat.get_bundle_with_transform(fat.get_connection_transform(&fat, Rhombus::UPPER_RIGHT_SIDE).unwrap())
        r4.get_bundle_with_transform(r4_transform)
    ).insert(r4.clone());

    let r5_transform = r1.get_connection_transform(Rhombus::LOWER_LEFT_SIDE, r5.get_type());
    commands.spawn_bundle(
        //fat.get_bundle_with_transform(fat.get_connection_transform(&fat, Rhombus::UPPER_RIGHT_SIDE).unwrap())
        r5.get_bundle_with_transform(r5_transform)
    ).insert(r5.clone());*/

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