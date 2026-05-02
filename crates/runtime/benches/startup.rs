use std::{hint::black_box, path::PathBuf, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use layer36_runtime::{Config, Runtime};
use wasmtime::{component::Component, Config as WasmtimeConfig, Engine};

const PRINT_LOOP_CALLS: u64 = 1_000;

fn phase1_runtime_benches(c: &mut Criterion) {
    let hello_wasm = wasm_path(
        "LAYER36_HELLO_WASM",
        "test/integration/hello-world/target/wasm32-wasip1/release/hello_world.wasm",
    );
    let print_loop_wasm = wasm_path(
        "LAYER36_PRINT_LOOP_WASM",
        "test/integration/print-loop/target/wasm32-wasip1/release/print_loop.wasm",
    );

    let hello = read_wasm(&hello_wasm);
    let print_loop = read_wasm(&print_loop_wasm);
    let runtime_config = Config::default();

    let mut group = c.benchmark_group("phase1_runtime");
    group
        .warm_up_time(Duration::from_secs(3))
        .measurement_time(Duration::from_secs(10));

    group.bench_function("engine_construction", |b| {
        b.iter(|| Runtime::new(black_box(&runtime_config)).expect("runtime should initialize"));
    });

    group.bench_function("component_from_binary_hello", |b| {
        let engine = component_engine();
        b.iter(|| {
            Component::from_binary(black_box(&engine), black_box(&hello))
                .expect("hello component should compile")
        });
    });

    group.bench_function("cold_start_to_main_hello", |b| {
        b.iter(|| {
            let runtime =
                Runtime::new(black_box(&runtime_config)).expect("runtime should initialize");
            runtime
                .run_bytes_silent(black_box(&hello), black_box(&runtime_config))
                .expect("hello component should run")
        });
    });

    group.bench_function("first_print_latency_hello", |b| {
        let runtime = Runtime::new(&runtime_config).expect("runtime should initialize");
        let component = runtime
            .load_component(&hello)
            .expect("hello component should compile");
        b.iter(|| {
            runtime
                .run_loaded_silent(black_box(&component), black_box(&runtime_config))
                .expect("hello component should run")
        });
    });

    group.throughput(Throughput::Elements(PRINT_LOOP_CALLS));
    group.bench_function("per_call_print_dispatch_1000", |b| {
        let runtime = Runtime::new(&runtime_config).expect("runtime should initialize");
        let component = runtime
            .load_component(&print_loop)
            .expect("print-loop component should compile");
        b.iter(|| {
            runtime
                .run_loaded_silent(black_box(&component), black_box(&runtime_config))
                .expect("print-loop component should run")
        });
    });

    group.finish();
}

fn component_engine() -> Engine {
    let mut config = WasmtimeConfig::new();
    config.wasm_component_model(true);
    Engine::new(&config).expect("engine should initialize")
}

fn wasm_path(env_var: &str, default_path: &str) -> PathBuf {
    std::env::var_os(env_var)
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace_root().join(default_path))
}

fn read_wasm(path: &PathBuf) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|err| {
        panic!(
            "failed to read {}: {err}. Run scripts/build-phase1-components.sh first, \
             or set LAYER36_HELLO_WASM and LAYER36_PRINT_LOOP_WASM.",
            path.display()
        )
    })
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("runtime crate should live under crates/runtime")
        .to_path_buf()
}

criterion_group!(benches, phase1_runtime_benches);
criterion_main!(benches);
