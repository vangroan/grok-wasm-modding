use wasmer::{
    imports, ExportError, Function, FunctionType, ImportObject, Instance, Module, Store, Type,
};

// Only instances need to be kept around to execute modules.
struct Game {
    instances: Vec<Instance>,
}

fn main() {
    let mut instances = vec![];

    // The wasmer Store contains the actual engine instance in an Arc.
    let store = Store::default();

    // Module contains the parsed artifact.
    let mod1 = Module::new(&store, &include_str!("../../mods/mod1.wast")).expect("load mod1");

    let mod1_instance = Instance::new(&mod1, &imports! {}).expect("mod1 instance");

    // A module's exports can be used as the imports of another instance.
    let exports = mod1_instance.exports.clone();
    instances.push(mod1_instance);

    let log_ty = FunctionType::new(vec![Type::I32], vec![]);
    let log_fn = Function::new(&store, &log_ty, |args| {
        println!("{:?}", args);
        Ok(vec![])
    });

    // Import object can be created without the macro
    // if the imports need to be defined at runtime.
    let mut import_object = ImportObject::new();
    import_object.register("mod1", exports);

    let mut builtins = wasmer::Exports::new();
    builtins.insert("log", log_fn);
    import_object.register("env", builtins);

    instances.push(
        Instance::new(
            &Module::new(&store, &include_str!("../../mods/mod2.wast")).expect("load mod2"),
            &import_object,
            // Exports from other instances can be passed into macro.
            //
            // &imports! {
            //     "mod1" => exports,
            //     "env" => {
            //         "log" => log_fn,
            //     }
            // },
        )
        .expect("mod2 instance"),
    );

    let game = Game { instances };

    // The different parts have shared, mutable interiors.
    //
    // The interior Arcs are copied by the things that need them.
    drop(store);

    // Function instances of the entry points can be cloned out
    // of the borrows.
    let mut entry_points = vec![];

    // Entry points into modules can be detected.
    for instance in &game.instances {
        match instance.exports.get_function("entry") {
            Ok(func) => {
                println!("calling mod entry point");
                func.call(&[]).expect("entry call 1 error");

                entry_points.push(func.clone());
            }
            Err(ExportError::Missing(..)) => { /* Mod does not define an entry point */ }
            Err(err) => {
                eprintln!("entry point error: {}", err);
            }
        }
    }

    // Engine is shared with function instance, so even if the game drops
    // we can still call the entry points we're holding onto.
    drop(game);

    // Function instance keeps copy of module instance and engine.
    for entry_point in &entry_points {
        entry_point.call(&[]).expect("entry call 1 error");
    }
}
