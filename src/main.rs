extern crate kiss3d;

use kiss3d::nalgebra as na;

use na::{Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    let mut window = Window::new(concat!("Sphere Peeper ", env!("CARGO_PKG_VERSION")));
    let mut c      = window.add_cube(1.0, 1.0, 1.0);

    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        c.prepend_to_local_rotation(&rot);
    }
}
