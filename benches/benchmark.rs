use criterion::{black_box, criterion_group, criterion_main, Criterion};
use meow::{
    platform::{Region, RegionIter},
    region::InfoExt,
};

const CONTENTS: &str = r#"563ea224a000-563ea2259000 r--p 00000000 103:05 5920780 /usr/bin/fish
563ea23ea000-563ea2569000 rw-p 00000000 00:00 0 [heap]
7f9e08000000-7f9e08031000 rw-p 00000000 00:00 0"#;

pub fn get_region_range(contents: &str) -> Vec<Region> {
    let mut vec: Vec<Region> = Vec::new();
    for line in contents.split('\n') {
        let mut split = line.split_whitespace();
        let range = split.next();
        if range.is_none() {
            break;
        }
        let mut range_split = range.unwrap().split('-');
        let range_start = range_split.next().unwrap();
        let range_end = range_split.next().unwrap();
        let flags = split.next().unwrap();

        vec.push(Region {
            range_start: usize::from_str_radix(range_start, 16).unwrap(),
            range_end: usize::from_str_radix(range_end, 16).unwrap(),
            flags: flags.to_string(),
            pathname: split.by_ref().skip(3).collect::<Vec<&str>>().join(" "),
        });
    }
    vec
}

#[inline]
fn test_maps1(s: &str) {
    let maps = get_region_range(s);
    assert_eq!(maps[0].start(), 0x563ea224a000);
    assert_eq!(maps[0].end(), 0x563ea2259000);
    assert_eq!(maps[0].pathname(), "/usr/bin/fish");
    assert_eq!(maps[1].pathname(), "[heap]");
}

#[inline]
fn test_maps2(s: &str) {
    let maps = RegionIter::new(s).collect::<Vec<_>>();
    assert_eq!(maps[0].start(), 0x563ea224a000);
    assert_eq!(maps[0].end(), 0x563ea2259000);
    assert_eq!(maps[0].pathname(), "/usr/bin/fish");
    assert_eq!(maps[1].pathname(), "[heap]");
}

fn benchmarks(c: &mut Criterion) {
    c.bench_function("testmap2", |b| b.iter(|| test_maps2(black_box(CONTENTS))));
    c.bench_function("testmap1", |b| b.iter(|| test_maps1(black_box(CONTENTS))));
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
