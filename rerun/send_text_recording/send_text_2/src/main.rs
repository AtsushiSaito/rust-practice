use rerun;
use rerun::{MsgSender, Session};
use uuid;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut counter = 0;

    let mut session = Session::init("record_text2", true);
    session.set_recording_id(rerun::RecordingId::from_uuid(uuid::uuid!(
        "8a658439-8b9b-0ca8-e76f-96c5b4f8e5cf"
    )));

    Session::connect(
        &mut session,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9876),
    );
    let one_millis = std::time::Duration::from_millis(1000);
    loop {
        let text = rerun::components::TextEntry::from_body(format!("One Counter is {}", counter));
        MsgSender::new("one_sec")
            .with_splat(text)?
            .send(&mut session)?;

        std::thread::sleep(one_millis);
        counter += 1;
    }
}
