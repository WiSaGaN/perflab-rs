#![feature(test)]
extern crate test;
extern crate time;
pub use time::{get_time, Timespec};
pub use std::sync::mpsc::{sync_channel, Sender};

#[cfg(test)]
mod g1_generic {
    use std;
    use super::*;
    use test::Bencher;

    #[bench]
    fn t01_one_plus_one(b: &mut Bencher) {
        b.iter(|| 1 + 1 );
    }

    #[bench]
    fn t02_get_time(b: &mut Bencher) {
        b.iter(|| get_time());
    }

    #[bench]
    fn t03_with_one_ms_setup(b: &mut Bencher) {
        std::thread::sleep(std::time::Duration::from_millis(1));
        b.iter(|| 1 + 1 );
    }
}

#[cfg(test)]
mod g2_sync_channel {
    use super::*;
    use test::Bencher;

    #[bench]
    fn t01_send_unit(b: &mut Bencher) {
        let (s, _) = sync_channel(1024000);
        b.iter(|| { s.send(()) });
    }

    #[bench]
    fn t02_send_integer(b: &mut Bencher) {
        let (s, _) = sync_channel(1024000);
        b.iter(|| { s.send(42) });
    }

    #[bench]
    fn t03_send_string(b: &mut Bencher) {
        let (s, _) = sync_channel(1024000);
        b.iter(|| { s.send(String::from("The quick brown fox jumps over the lazy dog.")) });
    }

    #[bench]
    fn t04_send_recv_unit(b: &mut Bencher) {
        let (s, r) = sync_channel(1024000);
        b.iter(|| { s.send(()).unwrap(); r.recv() });
    }

    #[bench]
    fn t05_send_recv_integer(b: &mut Bencher) {
        let (s, r) = sync_channel(1024000);
        b.iter(|| { s.send(42).unwrap(); r.recv() });
    }

    #[bench]
    fn t06_send_recv_string(b: &mut Bencher) {
        let (s, r) = sync_channel(1024000);
        b.iter(|| { s.send(String::from("The quick brown fox jumps over the lazy dog.")).unwrap(); r.recv() });
    }
}
