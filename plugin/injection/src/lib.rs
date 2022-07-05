use sdk::Plugin;

#[derive(Debug, Default)]
struct Injection {}

impl Plugin for Injection {
    fn name(&self) -> &'static str {
        "injection"
    }

    fn call(&self, args: &str) {
        match args {
            "version" => println!("v0.1.0"),
            "info" => println!("injection plugin linux x64"),
            _ => {}
        }
    }
}

#[no_mangle]
pub extern "C" fn plugin_start() -> *mut dyn Plugin {
    let object = Injection::default();
    let boxed: Box<dyn Plugin> = Box::new(object);
    Box::into_raw(boxed)
}
