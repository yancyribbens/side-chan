// inspired by https://electricdusk.com/cmov-conversion.html

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

	let secret_idx = 99;
	let result = constant_time_lookup(secret_idx, table);
	assert_eq!(false, result);

	let secret_idx = 11;
	let result = constant_time_lookup(secret_idx, table);
	assert_eq!(true, result);
}
