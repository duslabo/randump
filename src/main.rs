use argh::FromArgs;
pub use socketcan::{CANFilter, CANFrame, CANSocket, CANSocketOpenError};
use termion::color;

#[derive(FromArgs)]
/// Reach new heights.
struct GoUp {
    /// provide can interface name like can0 or vcan0
    #[argh(option, short = 'i')]
    interface: String,

    /// whether or not can id for matching like -m 123,12d,211
    #[argh(option, short = 'm')]
    match_id: Option<String>,
}

fn main() {
    println!("randump, start!");
    let up: GoUp = argh::from_env();

    let socket = CANSocket::open(&up.interface).unwrap();
    let mut vec_ids = Vec::new();

    if up.match_id != None {
        let c_ids = up
            .match_id
            .unwrap()
            .split(',')
            .map(|num_str| u32::from_str_radix(num_str, 16))
            .collect::<Vec<_>>();

        for idss in c_ids.iter() {
            let idd = idss.clone();
            let filter1 = CANFilter::new(idd.unwrap(), 0xfff).unwrap();
            vec_ids.push(filter1);
        }
        socket.set_filter(vec_ids.as_slice()).unwrap();
    }

    loop {
        let frame2 = socket.read_frame().unwrap();
        println!(
            "{} {:x} {} {:02x?}",
            color::Fg(color::Yellow),
            frame2.id(),
            color::Fg(color::Blue),
            frame2.data()
        );
    }
}
