extern crate kiss3d;

use kiss3d::nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{UnitQuaternion, Vector3};

use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new(concat!("Sphere Peeper ", env!("CARGO_PKG_VERSION")));
    let mut s = window.add_sphere(1.0);

    s.set_color(1.0, 1.0, 1.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    s.set_texture_from_file(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/test_pic.jpg")), "test-pic");

    while window.render() {
        s.prepend_to_local_rotation(&rot);
    }

    Ok(())
}
