use heapsize::HeapSizeOf;
use std::time::Instant;
use sysinfo::{self, get_current_pid, ProcessExt, System, SystemExt};

// These measurement functions use publicly available code!!

struct BenchmarkResult {
    name: String,
    iterations: u32,
    total_time_ms: f64,
    average_time_ms: f64,
    memory_usage_kb: f64,
    peak_memory_usage_kb: f64,
}

fn measure(
    name: &str,
    f: impl Fn() -> Box<dyn HeapSizeOf>,
    iterations: u32,
    warmup_iterations: u32,
) -> BenchmarkResult {
    for _ in 0..warmup_iterations {
        let _ = f();
    }

    let start = Instant::now();
    let mut total_memory_kb = 0.0;
    let mut peak_memory_kb: f64 = 0.0;

    let initial_memory_kb = get_current_memory_kb();

    for _ in 0..iterations {
        let result = f();
        let current_memory_kb = result.heap_size_of_children() as f64 / 1024.0; // Convert bytes to KB

        total_memory_kb += current_memory_kb;
        peak_memory_kb = peak_memory_kb.max(current_memory_kb); // Update peak memory
    }

    let final_memory_kb = get_current_memory_kb();

    let memory_delta_kb = final_memory_kb - initial_memory_kb;

    let duration = start.elapsed();
    let total_time_ms = duration.as_secs_f64() * 1000.0;

    BenchmarkResult {
        name: name.to_string(),
        iterations,
        total_time_ms,
        average_time_ms: total_time_ms / iterations as f64,
        memory_usage_kb: memory_delta_kb,
        peak_memory_usage_kb: peak_memory_kb,
    }
}

fn get_current_memory_kb() -> f64 {
    let system = System::new_all();
    let pid_result = get_current_pid();

    match pid_result {
        Ok(pid) => {
            if let Some(process) = system.process(pid) {
                process.memory() as f64 / 1024.0
            } else {
                0.0
            }
        }
        Err(_) => 0.0,
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    println!("Starting benchmarks...\n");
    let mut results = Vec::new();
    let warmup_iterations = 5; // Set warm-up iterations

    // 1. Fibonacci
    results.push(measure(
        "Fibonacci(40)",
        || Box::new(fibonacci(40)),
        10,
        warmup_iterations,
    ));

    // Masivu operacijas
    results.push(measure(
        "Array Operations (1M elements)",
        || {
            let vec: Vec<i64> = (0..1_000_000).collect();
            let doubled: Vec<i64> = vec.iter().map(|x| x * 2).collect();
            let filtered: Vec<i64> = doubled.iter().filter(|&&x| x % 2 == 0).copied().collect();
            Box::new(filtered.iter().sum::<i64>())
        },
        10,
        warmup_iterations,
    ));

    // Teksta virknu operacijas
    results.push(measure(
        "String Operations",
        || {
            let strings: Vec<String> = (0..10_000).map(|i| format!("string_{}", i)).collect();

            let concatenated = strings.join("");
            let split: Vec<&str> = concatenated.split('_').collect();
            let reversed: Vec<&str> = split.iter().rev().copied().collect();
            let rejoined = reversed.join("_");

            Box::new(rejoined)
        },
        20,
        warmup_iterations,
    ));

    println!("Benchmark Results:\n");
    println!(
        "{:<25} {:>10} {:>15} {:>15} {:>15} {:>15}",
        "Name",
        "Iterations",
        "Total Time(ms)",
        "Avg Time(ms)",
        "Memory Usage(KB)",
        "Peak Memory(KB)"
    );
    println!("{:-<100}", "");

    for result in results {
        println!(
            "{:<25} {:>10} {:>15.2} {:>15.2} {:>15.2} {:>15.2}",
            result.name,
            result.iterations,
            result.total_time_ms,
            result.average_time_ms,
            result.memory_usage_kb,
            result.peak_memory_usage_kb
        );
    }
}
