mod bindings {
    use crate::Handler;

    wit_bindgen::generate!({
        world: "server",
        path: "../../wit",
        with: {
            "wasmcon:blue/green": generate,
        }
    });
    export!(Handler);
}

struct Handler;

impl bindings::exports::wasmcon::blue::green::Guest for Handler {
    fn greet(name: String) -> String {
        format!("hello, {name}!")
    }
}
