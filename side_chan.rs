// inspired by https://electricdusk.com/cmov-conversion.html
use std::time;

// Select an element from an array in non-constant time
pub fn non_constant_time_lookup(secret_idx: u64,
                            table: [u64; 16]) -> bool {
    for val in table.iter() {
        if *val == secret_idx {
            return true
        }
    }

    return false
}

// Select an element from an array in constant time
pub fn constant_time_lookup(secret_idx: usize,
                            table: [u64; 16]) -> bool {
    let mut result: i64 = 0;

    for (i, _val) in table.iter().enumerate() {
        let cond = i == secret_idx;
        result |= cond as i64;
    }

    result != 0
}

fn main() {
	let table: [u64; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // constant time
    println!("constant time:");
    let secret_idx = 99;
    let now = time::Instant::now();
    let result = constant_time_lookup(secret_idx, table);
    let elapsed_time = now.elapsed();
    println!("ellapsed - not found: {:?}", elapsed_time);
    assert_eq!(false, result);

	let secret_idx = 0;
    let now = time::Instant::now();
	let result = constant_time_lookup(secret_idx, table);
    let elapsed_time = now.elapsed();
    println!("ellapsed - found: {:?}", elapsed_time);
	assert_eq!(true, result);

	let secret_idx = 8;
    let now = time::Instant::now();
	let result = constant_time_lookup(secret_idx, table);
    let elapsed_time = now.elapsed();
    println!("ellapsed - found: {:?}", elapsed_time);
	assert_eq!(true, result);

	let secret_idx = 15;
    let now = time::Instant::now();
	let result = constant_time_lookup(secret_idx, table);
    let elapsed_time = now.elapsed();
    println!("ellapsed - found: {:?}", elapsed_time);
	assert_eq!(true, result);

    // non-constant time
    println!("non-constant time:");
    let secret_idx = 99;
    let now = time::Instant::now();
    let result = non_constant_time_lookup(secret_idx, table);
    let elapsed_time = now.elapsed();
    println!("ellapsed - not found: {:?}", elapsed_time);
    assert_eq!(false, result);

	let secret_idx = 0;
    let now = time::Instant::now();
	let result = non_constant_time_lookup(secret_idx, table);
    let elapsed_time = now.elapsed();
    println!("ellapsed - found: {:?}", elapsed_time);
	assert_eq!(true, result);

	let secret_idx = 8;
    let now = time::Instant::now();
	let result = non_constant_time_lookup(secret_idx, table);
    let elapsed_time = now.elapsed();
    println!("ellapsed - found: {:?}", elapsed_time);
	assert_eq!(true, result);

	let secret_idx = 15;
    let now = time::Instant::now();
	let result = non_constant_time_lookup(secret_idx, table);
    let elapsed_time = now.elapsed();
    println!("ellapsed - found: {:?}", elapsed_time);
	assert_eq!(true, result);
}
