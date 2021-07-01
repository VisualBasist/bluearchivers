pub mod currency {
    struct AP {
        value: u32,
        // 1AP/6min
        last_recoverd_time: std::time::Instant,
    }
    struct Credit(u32);
    struct BluePyroxene(u32);
}
