extern crate kiss3d;

use kiss3d::{builtin::ObjectMaterial, nalgebra as na};

use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{UnitQuaternion, Vector3};

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new(concat!("Sphere Peeper ", env!("CARGO_PKG_VERSION")));
    let mut front_half = window.add_obj(
        Path::new("./sphere_hull.obj"),
        Path::new("/dev/null"),
        Vector3::new(1.0, 1.0, 1.0),
    );

    let rot_front = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), std::f32::consts::PI / -2.0);

    front_half.prepend_to_local_rotation(&rot_front);


    front_half.set_color(1.0, 1.0, 1.0);
    front_half.set_material(Rc::new(RefCell::new(Box::new(ObjectMaterial::new()))));

    window.set_light(Light::Absolute(na::Point3::new(0.0, 0.0, 0.0)));

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    front_half.set_texture_from_file(
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/test_pic.jpg")),
        "test-pic",
    );

    while window.render() {
        front_half.prepend_to_local_rotation(&rot);
    }

    Ok(())
}
