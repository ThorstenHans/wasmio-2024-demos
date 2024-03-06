use anyhow::Result;
use dialoguer::Input;
use std::path::PathBuf;
use wasmtime::{
    component::{Component, Linker, Val},
    Config, Engine, Store,
};
wasmtime::component::bindgen!("extensibility");

fn main() -> Result<()> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Hello Wasm I/O!");
    println!("---------------\n\n");
    loop {
        let message: String = Input::new()
            .with_prompt("Please provide a message")
            .with_post_completion_text("Thank you! We'll now transform")
            .validate_with(|v: &String| {
                if v.trim().len() > 0 {
                    return Ok(());
                }
                return Err("Funny! Please provide something");
            })
            .interact_text()
            .unwrap_or("Hello Wasm I/O!".to_string());
        match process_transformers(message.clone()) {
            Ok(result) => println!("Transformed \"{}\" to \"{}\"\n\n", message, result),
            Err(_) => println!("Error while transforming message"),
        }
    }
}

fn process_transformers(mut message: String) -> anyhow::Result<String> {
    let mut config = Config::new();
    config.wasm_multi_memory(true);
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut plugins = get_plugins_from_path("../active")?;
    plugins.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    if plugins.len() == 0 {
        println!("No transformers found. We'll keep it as it is");
        return Ok(message.clone());
    }
    let mut linker = Linker::new(&engine);
    Extensibility::add_to_linker(&mut linker, |state: &mut HostState| state)?;

    for plugin in plugins.iter() {
        let mut store = Store::new(&engine, HostState::default());
        let component = Component::from_file(&engine, plugin).map_err(|e| {
            println!("Error while loading component {:?}", e);
            e
        })?;
        let (_, instance) =
            Extensibility::instantiate(&mut store, &component, &linker).map_err(|e| {
                println!("Error while instantiating component {:?}", e);
                e
            })?;
        let params = vec![Val::String(message.clone().into())];
        let mut results = vec![Val::String("".into())];
        let Some(f) = instance.get_func(&mut store, "transform") else {
            break;
        };
        _ = match f.call(store, &params, &mut results) {
            Ok(_) => (),
            Err(_) => break,
        };
        message = match &results[0] {
            Val::String(s) => String::from(s.as_ref()),
            _ => unreachable!(),
        };
    }

    Ok(String::from(message.as_str()))
}

#[derive(Debug)]
struct HostState;

impl ExtensibilityImports for HostState {
    fn transform(&mut self, input: String) -> wasmtime::Result<String> {
        println!("Received {}", input);
        Ok(input)
    }
}

impl Default for HostState {
    fn default() -> Self {
        Self {}
    }
}

fn get_plugins_from_path(path: &str) -> anyhow::Result<Vec<PathBuf>> {
    let plugins = std::fs::read_dir(path)?
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
    Ok(plugins)
}
