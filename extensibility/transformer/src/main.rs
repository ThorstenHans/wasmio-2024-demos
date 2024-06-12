use anyhow::Result;
use dialoguer::Input;
use std::{io::Error, path::PathBuf};
use wasmtime::{
    component::{Component, Linker, Val},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
wasmtime::component::bindgen!("extensibility");

fn process_transformers(mut message: String) -> anyhow::Result<String> {
    let mut config = Config::new();
    config.wasm_multi_memory(true);
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let plugins = get_plugins_from_path("../active")?;

    if plugins.is_empty() {
        println!("No transformers found. We'll keep it as it is");
        return Ok(message.clone());
    }

    let mut linker = Linker::<MyState>::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    for plugin in plugins.iter() {
        let mut store = Store::new(
            &engine,
            MyState {
                ctx: WasiCtxBuilder::new().build(),
                table: ResourceTable::new(),
            },
        );

        // Load Component from the .wasm file
        let component = Component::from_file(&engine, plugin).map_err(|e| {
            println!("Error while loading component {:?}", e);
            e
        })?;

        // Instantiate the Component
        let instance = linker.instantiate(&mut store, &component)?;
        let params = vec![Val::String(message.clone())];
        let mut results = vec![Val::String("".into())];

        // Look for the transform function
        let Some(f) = instance.get_func(&mut store, "transform") else {
            println!("Component {:?} does not export transform function! Not a valid plugin for transformer!", plugin);
            break;
        };

        // invoke the transform function
        match f.call(store, &params, &mut results) {
            Ok(_) => (),
            Err(_) => {
                println!("Plugin {:?} failed upon invocation", plugin);
                break;
            }
        };
        message = match &results[0] {
            Val::String(s) => String::from(s),
            _ => unreachable!(),
        };
    }

    Ok(String::from(message.as_str()))
}

// push that down to here to prevent distracting the session attendee
fn main() -> Result<()> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Hello Wasm I/O!");
    println!("---------------\n\n");
    loop {
        let message: String = Input::new()
            .with_prompt("Please provide a message")
            .with_post_completion_text("Thank you! We'll now transform")
            .validate_with(validate_user_input)
            .interact_text()
            .unwrap_or("Hello Wasm I/O!".to_string());

        match process_transformers(message.clone()) {
            Ok(result) => println!("Transformed \"{}\" to \"{}\"\n\n", message, result),
            Err(_) => println!("Error while transforming message"),
        }
    }
}

#[derive(Debug)]
struct HostState;

impl ExtensibilityImports for HostState {
    fn transform(&mut self, input: String) -> String {
        println!("Received {}", input);
        input
    }
}

fn get_plugins_from_path(path: &str) -> anyhow::Result<Vec<PathBuf>> {
    let mut plugins = std::fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .filter_map(|path| {
            if path.extension().map_or(false, |ext| ext == "wasm") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    plugins.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    Ok(plugins)
}

fn validate_user_input(input: &String) -> Result<(), Error> {
    if input.trim().len() > 0 {
        return Ok(());
    }
    Err(Error::new(
        std::io::ErrorKind::Unsupported,
        "Funny! Please provide something",
    ))
}

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}
