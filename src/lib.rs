#![feature(test)]
extern crate test;
extern crate time;

#[cfg(test)]
mod g1_generic {
    use test::Bencher;
    use std::thread;
    use std::time::Duration;
    use time::get_time;

    #[bench]
    fn t01_one_plus_one(b: &mut Bencher) {
        b.iter(|| 1 + 1);
    }

    #[bench]
    fn t02_get_time(b: &mut Bencher) {
        b.iter(|| get_time());
    }

    #[bench]
    fn t03_with_one_ms_setup(b: &mut Bencher) {
        thread::sleep(Duration::from_millis(1));
        b.iter(|| 1 + 1);
    }
}

#[cfg(test)]
mod g2_sync_channel {
    use test::Bencher;
    use std::sync::mpsc::sync_channel;

    #[bench]
    fn t01_send_unit(b: &mut Bencher) {
        let (s, _) = sync_channel(1024000);
        b.iter(|| s.send(()));
    }

    #[bench]
    fn t02_send_integer(b: &mut Bencher) {
        let (s, _) = sync_channel(1024000);
        b.iter(|| s.send(42));
    }

    #[bench]
    fn t03_send_string(b: &mut Bencher) {
        let (s, _) = sync_channel(1024000);
        b.iter(|| s.send(String::from("The quick brown fox jumps over the lazy dog.")));
    }

    #[bench]
    fn t04_send_recv_unit(b: &mut Bencher) {
        let (s, r) = sync_channel(1024000);
        b.iter(|| {
            s.send(()).unwrap();
            r.recv()
        });
    }

    #[bench]
    fn t05_send_recv_integer(b: &mut Bencher) {
        let (s, r) = sync_channel(1024000);
        b.iter(|| {
            s.send(42).unwrap();
            r.recv()
        });
    }

    #[bench]
    fn t06_send_recv_string(b: &mut Bencher) {
        let (s, r) = sync_channel(1024000);
        b.iter(|| {
            s.send(String::from("The quick brown fox jumps over the lazy dog.")).unwrap();
            r.recv()
        });
    }
}

#[cfg(test)]
mod g3_formatting {
    use test::Bencher;
    use std::thread;
    use time::{at, get_time};

    #[bench]
    fn t01_format_simple_string(b: &mut Bencher) {
        b.iter(|| format!("hello, world"));
    }

    #[bench]
    fn t02_format_debug_get_time(b: &mut Bencher) {
        b.iter(|| format!("now: {:?}", get_time()));
    }

    #[bench]
    fn t03_format_log_line(b: &mut Bencher) {
        b.iter(|| {
            let tm = at(get_time());
            let tm_millisec = tm.tm_nsec / 1_000_000;
            let tm_microsec = tm.tm_nsec / 1_000 - tm_millisec * 1_000;
            format!("{:0>4}{:0>2}{:0>2}T{:0>2}{:0>2}{:0>2}.{:0>3}{:0>3}{:>+03} {}:{}/{}:{}:{} {}",
                    tm.tm_year + 1900,
                    tm.tm_mon + 1,
                    tm.tm_mday,
                    tm.tm_hour,
                    tm.tm_min,
                    tm.tm_sec,
                    tm_millisec,
                    tm_microsec,
                    tm.tm_utcoff / 3600,
                    thread::current().name().unwrap_or_default(),
                    module_path!(),
                    file!(),
                    line!(),
                    "INFO",
                    "hello world")
        });
    }
}
