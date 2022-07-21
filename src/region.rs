pub trait InfoExt {
    fn size(&self) -> usize;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn is_read(&self) -> bool;
    fn is_write(&self) -> bool;
    fn pathname(&self) -> &str;
}
