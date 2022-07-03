use sdk::{plugin_start, Plugin};

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

plugin_start!(Injection, Injection::default);
