use rerun::demo_util::grid;
use rerun::external::glam;
use rerun::{
    components::{ColorRGBA, Point3D},
    MsgSender, Session,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut counter = 2;

    let mut session = Session::init("minimal_rs", true);

    Session::connect(
        &mut session,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9876),
    );
    let one_millis = std::time::Duration::from_millis(1000);
    loop {
        let points = grid(glam::Vec3::splat(-5.0), glam::Vec3::splat(5.0), counter)
            .map(Point3D::from)
            .collect::<Vec<_>>();

        let colors = grid(glam::Vec3::ZERO, glam::Vec3::splat(255.0), counter)
            .map(|v| ColorRGBA::from_rgb(v.x as u8, v.y as u8, v.z as u8))
            .collect::<Vec<_>>();

        MsgSender::new("my_points")
            .with_component(&points)?
            .with_component(&colors)?
            .send(&mut session)?;

        counter += 1;

        std::thread::sleep(one_millis);
    }
}
