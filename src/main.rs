use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::math::Vec2;
use bevy::prelude::Commands;
use bevy_prototype_lyon::entity::{ShapeBundle, ShapeColors};
use rand::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_system(shapes.system())
        .run();
}

#[derive(Clone)]
struct Rhombus {
    small_angle: f32,
    short_leg_len: f32,
    color: Color
}

impl Default for Rhombus {
    fn default() -> Self {
        Rhombus {
            small_angle: f32::to_radians(36.0),
            short_leg_len: 100.0,
            color: Color::TEAL
        }
    }
}

impl Rhombus {
    fn new_skinny() -> Self {
        Rhombus {
            small_angle: f32::to_radians(36.0),
            color: Color::RED,
            ..Rhombus::default()
        }
    }

    fn new_fat() -> Self {
        Rhombus {
            small_angle: f32::to_radians(72.0),
            color: Color::BLUE,
            ..Rhombus::default()
        }
    }

    fn get_bundle_with_transform(&self, transform: Transform) -> ShapeBundle {
        assert!(self.small_angle <= f32::to_radians(90.0));

        // This is probably not the fastest way to do it... but it works
        let long_diag_len = self.short_leg_len * (2.0 + 2.0 * self.small_angle.cos()).sqrt();
        //let large_angle = f32::to_radians(180.0) - self.small_angle;
        //let small_diag_len = (2.0 + 2.0 * large_angle.cos()).sqrt();
        let half_small = self.small_angle / 2.0;
        //let half_large = large_angle / 2.0;

        let upper_right_corner = Vec2::new(long_diag_len * half_small.cos(), long_diag_len * half_small.sin());
        let y_axis_exterior_angle = f32::to_radians(90.0) - self.small_angle;
        let upper_left_corner = Vec2::new(upper_right_corner.y * y_axis_exterior_angle.tan(), upper_right_corner.y);

        GeometryBuilder::build_as(
            &shapes::Polygon {
                points: vec![
                    Vec2::new(0.0, 0.0), 
                    Vec2::new(self.short_leg_len, 0.0), 
                    upper_right_corner,
                    upper_left_corner
                ],
                closed: true
            },
            ShapeColors::outlined(self.color, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(10.0),
            },
            transform
        )
    }

    fn get_bundle(&self) -> ShapeBundle {
        self.get_bundle_with_transform(Transform::default())
    }

}

fn setup(mut commands: Commands) {

    let mut rng = rand::thread_rng();
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
    }

}

fn shapes(mut query: Query<(&Rhombus, &mut Transform)>, mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for (rhomb, mut transform) in query.iter_mut() {
        //if rhomb.color == Color::RED {
        //    continue;
        //}
        let (a, cur_angle) = transform.rotation.to_axis_angle();
        let mut new_angle = cur_angle + f32::to_radians(2.0 * rng.gen::<f32>());
        if new_angle.to_degrees() >= 360.0 {
            new_angle = 0.0;
        }
        transform.rotation = Quat::from_rotation_z(new_angle);
        //println!("translation {}, new_angle: {}, cur_angle {}, about {}", transform.rotation, new_angle.to_degrees(), cur_angle.to_degrees(), a)
    }
}