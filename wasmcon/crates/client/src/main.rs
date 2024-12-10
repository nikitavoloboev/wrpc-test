mod bindings {
    wit_bindgen::generate!({
        world: "client",
        path: "../../wit",
        with: {
            "wasmcon:blue/green": generate,
        }
    });
}

fn main() {
    let greeting = bindings::wasmcon::blue::green::greet("demo");
    println!("{greeting}");
}
