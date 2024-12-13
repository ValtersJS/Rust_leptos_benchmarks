use heapsize::HeapSizeOf;
use rayon::prelude::*;
use std::time::Instant;

// These measurement functions use publicly available code!!

// Define BenchmarkResult struct
struct BenchmarkResult {
    name: String,
    iterations: u32,
    total_time_ms: f64,
    average_time_ms: f64,
    memory_usage_kb: f64,
    peak_memory_usage_kb: f64, // Change made here
}

// Measure function to benchmark code
fn measure(name: &str, f: impl Fn() -> Box<dyn HeapSizeOf>, iterations: u32) -> BenchmarkResult {
    let start = Instant::now();
    let mut total_memory_kb = 0.0;
    let mut peak_memory_kb: f64 = 0.0; // Specify the type explicitly

    for _ in 0..iterations {
        let result = f();
        let current_memory_kb = result.heap_size_of_children() as f64 / 1024.0; // Convert bytes to KB

        total_memory_kb += current_memory_kb;
        peak_memory_kb = peak_memory_kb.max(current_memory_kb); // Update peak memory
    }

    let duration = start.elapsed();
    let total_time_ms = duration.as_secs_f64() * 1000.0;

    BenchmarkResult {
        name: name.to_string(),
        iterations,
        total_time_ms,
        average_time_ms: total_time_ms / iterations as f64,
        memory_usage_kb: total_memory_kb / iterations as f64, // Average memory usage
        peak_memory_usage_kb: peak_memory_kb,                 // Use the correct variable
    }
}

// Example computation functions
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

// Matrix multiplication function
fn matrix_multiply(size: usize) -> Box<Vec<Vec<i32>>> {
    let a: Vec<Vec<i32>> = (0..size).map(|_| (0..size).map(|_| 1).collect()).collect();
    let b: Vec<Vec<i32>> = (0..size).map(|_| (0..size).map(|_| 1).collect()).collect();

    let mut c = vec![vec![0; size]; size];

    c.par_iter_mut().enumerate().for_each(|(i, row)| {
        for j in 0..size {
            row[j] = (0..size).map(|k| a[i][k] * b[k][j]).sum();
        }
    });

    Box::new(c)
}

fn main() {
    println!("Starting benchmarks...\n");
    let mut results = Vec::new();

    // 1. Fibonacci
    results.push(measure("Fibonacci(40)", || Box::new(fibonacci(40)), 10));

    // 2. Array Operations
    results.push(measure(
        "Array Operations (1M elements)",
        || {
            let vec: Vec<i64> = (0..1_000_000).collect();
            let doubled: Vec<i64> = vec.iter().map(|x| x * 2).collect();
            let filtered: Vec<i64> = doubled.iter().filter(|&&x| x % 2 == 0).copied().collect();
            Box::new(filtered.iter().sum::<i64>())
        },
        10,
    ));

    // 3. Object Operations
    results.push(measure(
        "Object Operations (100k objects)",
        || {
            let objects: Vec<_> = (0..100_000)
                .map(|i| (i, format!("item {}", i), vec![i; 10]))
                .collect();

            Box::new(
                objects
                    .iter()
                    .map(|(id, value, data)| {
                        let id = *id; // Copy `id`
                        let value = value.clone(); // Clone `value` to own it
                        let data = data.clone(); // Clone `data` to own it
                        let sum: u32 = data.iter().sum();
                        (id, value, true, sum)
                    })
                    .collect::<Vec<_>>(),
            )
        },
        5,
    ));

    // 4. String Operations
    results.push(measure(
        "String Operations",
        || {
            let strings: Vec<String> = (0..10_000).map(|i| format!("string_{}", i)).collect();
            let concatenated = strings.join("");
            Box::new(
                concatenated
                    .split('_')
                    .map(String::from) // Convert &str to String for ownership
                    .rev()
                    .collect::<Vec<String>>(),
            )
        },
        20,
    ));

    // 5. Matrix Multiplication
    results.push(measure(
        "Matrix Multiplication (100x100)",
        || matrix_multiply(100),
        10,
    ));

    // Print results
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
