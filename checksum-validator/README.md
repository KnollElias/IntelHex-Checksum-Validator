# checksum-validator

A small Rust CLI that validates the checksum of a single Intel HEX record.

## What It Validates

- Hex parsing (every byte must be valid hex)
- Record length (matches `LL`)
- Checksum (`CC`) against computed two's-complement checksum

The input can include a leading `:` or omit it.

## Requirements

- Rust toolchain with Cargo

## Run

Pass a record as the first CLI argument:

```bash
cargo run -- ":10010000214601360121470136007EFE09D2190140"
```

You can also omit the leading colon:

```bash
cargo run -- "10010000214601360121470136007EFE09D2190140"
```

Or pipe input through stdin:

```bash
printf ':10010000214601360121470136007EFE09D2190140\n' | cargo run --
```

## Example Output

Valid record:

```text
Valid Intel HEX record
byte_count=0x10
record_type=0x00
checksum=0x40
computed_checksum=0x40
```

Invalid record:

```text
Invalid Intel HEX record: checksum mismatch: expected 0x40, got 0x41
```

## Exit Codes

- `0`: Record is valid
- `1`: Record is invalid
- `2`: No input provided

## Intel HEX Record Shape

This tool validates one record in the standard form:

```text
:LLAAAATT[DD...]CC
```

- `LL`: data byte count
- `AAAA`: address
- `TT`: record type
- `DD`: data bytes (optional, count defined by `LL`)
- `CC`: checksum

## Test

```bash
cargo test
```
