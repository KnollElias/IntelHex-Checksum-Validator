mod validator;

use std::env;
use std::io::{self, Read};
use validator::{ValidationResult, validate_intel_hex_record};

fn main() {
    let input = read_input();

    let Some(record) = input else {
        eprintln!("Usage: checksum-validator '<Intel HEX record>'");
        eprintln!("Or pipe a record in through stdin.");
        std::process::exit(2);
    };

    match validate_intel_hex_record(&record) {
        Ok(ValidationResult {
            checksum,
            computed_checksum,
            byte_count,
            record_type,
        }) => {
            println!("Valid Intel HEX record");
            println!("byte_count=0x{byte_count:02X}");
            println!("record_type=0x{record_type:02X}");
            println!("checksum=0x{checksum:02X}");
            println!("computed_checksum=0x{computed_checksum:02X}");
        }
        Err(err) => {
            eprintln!("Invalid Intel HEX record: {err}");
            std::process::exit(1);
        }
    }
}

fn read_input() -> Option<String> {
    if let Some(arg) = env::args().nth(1) {
        let trimmed = arg.trim();
        return (!trimmed.is_empty()).then(|| trimmed.to_string());
    }

    let mut buffer = String::new();
    if io::stdin().read_to_string(&mut buffer).ok()? == 0 {
        return None;
    }

    let trimmed = buffer.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}
