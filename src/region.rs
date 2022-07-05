pub trait RegionExt {
    fn size(&self) -> usize;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn pathname(&self) -> &str;
    fn is_exec(&self) -> bool;
    fn is_write(&self) -> bool;
    fn is_read(&self) -> bool;
}
