#![feature(once_cell)]

fn main() -> Result<(), std::io::Error> {
    let mut i = 0;
    let hello = "hello".as_bytes();
    loop {
        let t = lince::mem::search_all_rw_mem(hello)?;
        if t.len() < 11 {
            t.iter().try_for_each(|f| -> Result<(), std::io::Error> {
                let a = lince::mem::read_bytes(*f, hello.len())?;
                println!("0x{:x} -> 0x{:?}", f, a);
                Ok(())
            })?;
        } else {
            t[0..10]
                .iter()
                .try_for_each(|f| -> Result<(), std::io::Error> {
                    let a = lince::mem::read_bytes(*f, hello.len())?;
                    println!("0x{:x} -> 0x{:?}", f, a);
                    Ok(())
                })?;
        }

        i += 1;
        println!("============={}===========================", i);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
