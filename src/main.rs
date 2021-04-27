use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::math::Vec2;
use bevy::prelude::Commands;
use bevy_prototype_lyon::entity::{ShapeBundle, ShapeColors};
use rand::prelude::*;

type PointList = Vec<Vec2>;
type Radians = f32;

trait Tile {
    type SideFlags;
    fn get_bundle_with_transform(&self, transform: Transform) -> ShapeBundle;
    fn get_bundle(&self) -> ShapeBundle {
        self.get_bundle_with_transform(Transform::default())
    }
    fn has_free_sides(&self) -> bool;
    fn get_free_sides(&self) -> Self::SideFlags;
    fn set_side_used(&mut self, side: i32);
    fn get_side_coordinates(&self, side: i32) -> (Vec2, Vec2); // These are in tile-space
    //fn get_side_rotation(&self, side: i32) -> Radians;
}

#[derive(Clone)]
struct Rhombus {
    small_angle: f32,
    leg_len: f32,
    color: Color,
    used_side_flags: u8,
    points: PointList
}

impl Rhombus {
    const LEFT_INDEX: usize = 0;
    const TOP_INDEX: usize = 1;
    const RIGHT_INDEX: usize = 2;
    const BOTTOM_INDEX: usize = 3;
    fn new_skinny() -> Self {
        let mut r = Rhombus {
            small_angle: f32::to_radians(36.0),
            leg_len: 100.0,
            color: Color::RED,
            used_side_flags: 0,
            points: PointList::new()
        };

        r.points = Rhombus::get_points(r.leg_len, r.small_angle);
        r
    }

    fn new_fat() -> Self {
        let mut r = Rhombus {
            small_angle: f32::to_radians(72.0),
            leg_len: 100.0,
            color: Color::BLUE,
            used_side_flags: 0,
            points: PointList::new()
        };

        r.points = Rhombus::get_points(r.leg_len, r.small_angle);
        r
    }

    fn get_points(leg_len: f32, small_angle: f32) -> PointList {
        assert!(small_angle <= f32::to_radians(90.0));

        // Create a rhombus centered at the origin

        // This is probably not the fastest way to do it... but it works
        let long_diag_len = leg_len * (2.0 + 2.0 * small_angle.cos()).sqrt();
        let half_long_diag = long_diag_len / 2.0;
        //let large_angle = f32::to_radians(180.0) - small_angle;
        let small_diag_len = leg_len * (2.0 - 2.0 * small_angle.cos()).sqrt();
        let half_small_diag = small_diag_len / 2.0;
        //let half_small = small_angle / 2.0;
        //let y_axis_exterior_angle = f32::to_radians(90.0) - small_angle;

        // These coordinates are with the lower left corner at origin
        /*let lower_left_corner = Vec2::new(0.0, 0.0);
        let lower_right_corner = Vec2::new(leg_len, 0.0);
        let upper_right_corner = Vec2::new(long_diag_len * half_small.cos(), long_diag_len * half_small.sin());
        let upper_left_corner = Vec2::new(upper_right_corner.y * y_axis_exterior_angle.tan(), upper_right_corner.y);*/

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
}

impl Default for Rhombus {
    fn default() -> Self {
        Rhombus::new_skinny()
    }
}

impl Tile for Rhombus {
    type SideFlags = u8;
    fn has_free_sides(&self) -> bool {
        return self.used_side_flags & 0xf != 0xf
    }
    fn get_free_sides(&self) -> Self::SideFlags {
        return !(self.used_side_flags & 0xf);
    }
    fn set_side_used(&mut self, side: i32) {
        self.used_side_flags |= 1 << side;
    }

    fn get_side_coordinates(&self, side:i32) -> (Vec2, Vec2) {
        let uside = side as usize;
        assert!(side >= 0 && uside < self.points.len());

        let first_point = uside;
        let second_point = (uside + 1) % self.points.len();
        (self.points[first_point], self.points[second_point])
    }

    fn get_bundle_with_transform(&self, transform: Transform) -> ShapeBundle {
        GeometryBuilder::build_as(
            &shapes::Polygon {
                points: self.points.clone(),
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
    commands.spawn_bundle(
        fat.get_bundle()
    ).insert(fat);
    let fat_test = Rhombus::new_fat();
    let rhomb2 = Rhombus::new_fat();
    //let r = (rhomb2.leg_len * rhomb2.small_angle.sin()) / 2.0;
    /*let half_long_diag_len = (rhomb2.leg_len * (2.0 + 2.0 * rhomb2.small_angle.cos()).sqrt()) / 2.0;
    let large_angle = f32::to_radians(180.0) - rhomb2.small_angle;
    let half_short_diag = (rhomb2.leg_len * (2.0 - 2.0 * rhomb2.small_angle.cos()).sqrt()) / 2.0;
    //let mut transform = Transform::from_xyz(0.0/*(fat_test.leg_len - (half_long_diag_len * (fat_test.small_angle / 2.0).cos())) + half_short_diag*//*fat_test.leg_len * fat_test.small_angle.sin()*/, -fat_test.leg_len / 2.0 /* fat_test.small_angle.cos()*/, 0.0);
    let mut transform = Transform::from_xyz(fat_test.points[1].x + half_short_diag, fat_test.points[2].y - half_long_diag_len, 0.0);
    let mut transform = Transform::from_xyz(fat_test.points[1].x + half_short_diag * (large_angle / 2.0).cos(), fat_test.points[1].y - half_short_diag * (large_angle / 2.0).sin(), 0.0);
    transform.rotate(Quat::from_rotation_z(fat_test.small_angle));*/
    //let large_angle = f32::to_radians(180.0) - fat_test.small_angle;
    let rotation_angle: Radians =  -((fat_test.small_angle - rhomb2.small_angle) / 2.0);//f32::to_radians(90.0) + (large_angle / 2.0) + rhomb2.small_angle / 2.0;
    let long_diag_len = rhomb2.leg_len * (2.0 + 2.0 * rhomb2.small_angle.cos()).sqrt();
    let half_long_diag = long_diag_len / 2.0;

    //let mut dir = (fat_test.points[Rhombus::RIGHT_INDEX] - fat_test.points[Rhombus::TOP_INDEX]).extend(0.0);
    let dir = Vec3::new(1.0, 0.0, 0.0);
    let t = Transform::from_rotation(Quat::from_rotation_z(-((fat_test.small_angle - rhomb2.small_angle) / 2.0)));
    let rotated_extended = half_long_diag * (t * dir);
    println!("Rotated: {}", rotated_extended);
    let translated = rotated_extended + fat_test.points[Rhombus::TOP_INDEX].extend(0.0);
    println!("Top: {}", fat_test.points[Rhombus::TOP_INDEX].extend(0.0));
    println!("Translated: {}", translated);
    //let mut transform = Transform::from_xyz(fat_test.points[Rhombus::RIGHT_INDEX].x, fat_test.points[Rhombus::TOP_INDEX].y, 0.0);
    let mut transform = Transform::from_translation(translated);
    transform.rotate(
        Quat::from_rotation_z(rotation_angle)
    );
    println!("transform: {:?}, rotation_angle: {}", transform, f32::to_degrees(rotation_angle));
    //transform.tra
    commands.spawn_bundle(
        rhomb2.get_bundle_with_transform(transform.clone())
    ).insert(rhomb2.clone());

    println!("fat top: {}, skinny left: {}", fat_test.points[Rhombus::TOP_INDEX], transform * rhomb2.points[Rhombus::RIGHT_INDEX].extend(0.0))
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