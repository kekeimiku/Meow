static MAPS: &str = r#"55ec34929000-55ec3492a000 r--p 00000000 103:02 538486                    /home/keke/Templates/哈 哈/test
55ec3492a000-55ec3492b000 r-xp 00001000 103:02 538486                    /home/keke/Templates/哈 哈/test
55ec3492b000-55ec3492c000 r--p 00002000 103:02 538486                    /home/keke/Templates/哈 哈/test
55ec3492c000-55ec3492d000 r--p 00002000 103:02 538486                    /home/keke/Templates/哈 哈/test
55ec3492d000-55ec3492e000 rw-p 00003000 103:02 538486                    /home/keke/Templates/哈 哈/test
55ec354d9000-55ec354fa000 rw-p 00000000 00:00 0                          [heap]
7f5c416cb000-7f5c416ce000 rw-p 00000000 00:00 0 
7f5c416ce000-7f5c416fa000 r--p 00000000 103:02 10489193                  /usr/lib/libc.so.6
7f5c416fa000-7f5c41870000 r-xp 0002c000 103:02 10489193                  /usr/lib/libc.so.6
7f5c41870000-7f5c418c4000 r--p 001a2000 103:02 10489193                  /usr/lib/libc.so.6
7f5c418c4000-7f5c418c5000 ---p 001f6000 103:02 10489193                  /usr/lib/libc.so.6
7f5c418c5000-7f5c418c8000 r--p 001f6000 103:02 10489193                  /usr/lib/libc.so.6
7f5c418c8000-7f5c418cb000 rw-p 001f9000 103:02 10489193                  /usr/lib/libc.so.6
7f5c418cb000-7f5c418d8000 rw-p 00000000 00:00 0 
7f5c418fe000-7f5c41900000 rw-p 00000000 00:00 0 
7f5c41900000-7f5c41902000 r--p 00000000 103:02 10489181                  /usr/lib/ld-linux-x86-64.so.2
7f5c41902000-7f5c41929000 r-xp 00002000 103:02 10489181                  /usr/lib/ld-linux-x86-64.so.2
7f5c41929000-7f5c41934000 r--p 00029000 103:02 10489181                  /usr/lib/ld-linux-x86-64.so.2
7f5c41935000-7f5c41937000 r--p 00034000 103:02 10489181                  /usr/lib/ld-linux-x86-64.so.2
7f5c41937000-7f5c41939000 rw-p 00036000 103:02 10489181                  /usr/lib/ld-linux-x86-64.so.2
7ffe40b6b000-7ffe40b8c000 rw-p 00000000 00:00 0                          [stack]
7ffe40bf3000-7ffe40bf7000 r--p 00000000 00:00 0                          [vvar]
7ffe40bf7000-7ffe40bf9000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  [vsyscall]
"#;

#[test]
fn test_parse_maps() {
    use lince::maps::parse_proc_maps;
    parse_proc_maps(MAPS);
}
