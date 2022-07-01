fn main() {
    // let file = File::open("/proc/16017/mem").unwrap();

    // let value = 9998_i32.to_ne_bytes();
    // // 564cc08e8000-564cc0a52000
    // // 7ffd22537000-7ffd22559000
    // let v = Mem::new(&file).find_region_addr(0x7ffd22537000, 0x7ffd22559000, &value);

    // info!("{}", 0x564cc09dd980 - 0x564cc08e8000_usize);

    // info!(
    //     "{:?}",
    //     v.unwrap()
    //         .iter()
    //         .map(|n| format!("0x{:x}", n + 0x7ffd22537000))
    //         .collect::<Vec<_>>()
    // );
}
