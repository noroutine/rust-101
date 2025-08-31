use std::io::{self, Read};

const GB: usize = 1024 * 1024 * 1024;

fn main() {
    println!("Attempting to create an 8GB string...");

    let target_size = 16 * GB;

    println!(
        "Target size: {} bytes ({:.0} GB)",
        target_size,
        target_size / GB
    );

    // Method 1: Try to create with repeat
    match try_create_giant_string_repeat(target_size) {
        Ok(s) => println!("Success with repeat! Length: {}", s.len()),
        Err(e) => println!("Failed with repeat: {}", e),
    }

    // Method 2: Try to create by pushing in chunks
    match try_create_giant_string_chunks(target_size) {
        Ok(s) => println!("Success with chunks! Length: {}", s.len()),
        Err(e) => println!("Failed with chunks: {}", e),
    }

    // Method 3: Try with Vec<u8> first, then convert
    match try_create_giant_vec(target_size) {
        Ok(v) => {
            println!("Success with Vec! Length: {}", v.len());
            // Try to convert to String (this might fail if not valid UTF-8)
            match String::from_utf8(v) {
                Ok(s) => println!("Converted to String successfully! Length: {}", s.len()),
                Err(_) => println!("Vec created but couldn't convert to String"),
            }
        }
        Err(e) => println!("Failed with Vec: {}", e),
    }

    println!("{:.0}G Experiment begins...", target_size / GB);
    let mut double_cheese = try_create_giant_string_repeat(target_size).expect("Experiment failed...");

    let len = double_cheese.len();
    println!("..Reserving");
    double_cheese.reserve(len);
    println!("..Cloning");
    let double_cheese_clone = double_cheese.clone();
    println!("..Copying");
    double_cheese.push_str(&double_cheese_clone);

    println!("Press any key to continue...");
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

fn try_create_giant_string_repeat(size: usize) -> Result<String, String> {
    println!("Trying String::repeat...");

    match std::panic::catch_unwind(|| "A".repeat(size)) {
        Ok(s) => Ok(s),
        Err(_) => Err("Panic during repeat".to_string()),
    }
}

fn try_create_giant_string_chunks(size: usize) -> Result<String, String> {
    println!("Trying chunk-by-chunk creation...");

    match std::panic::catch_unwind(|| {
        let mut s = String::with_capacity(size);
        let chunk = "A".repeat(1024 * 1024); // 1MB chunks

        for _ in 0..(size / (1024 * 1024)) {
            s.push_str(&chunk);
        }

        // Add remaining bytes
        let remaining = size % (1024 * 1024);
        if remaining > 0 {
            s.push_str(&"A".repeat(remaining));
        }

        s
    }) {
        Ok(s) => Ok(s),
        Err(_) => Err("Panic during chunk creation".to_string()),
    }
}

fn try_create_giant_vec(size: usize) -> Result<Vec<u8>, String> {
    println!("Trying Vec<u8> creation...");

    match std::panic::catch_unwind(|| {
        vec![b'A'; size] // Create vector of bytes
    }) {
        Ok(v) => Ok(v),
        Err(_) => Err("Panic during Vec creation".to_string()),
    }
}
