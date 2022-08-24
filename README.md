# Hex Utilities
> A collection of utilities for working with hexadecimals

## Install
> Add package to Cargo.toml file
```rust
[dependencies]
hex-utilities = "0.1.1"
```

## Usage:
```rust
use hex_utilities::{
    get_text_for_hex,
}

// Convert hext to text
// Should work similiar to this http://www.unit-conversion.info/texttools/hexadecimal
let hex = "30784e6f6e63652077617320666f756e646564".to_string();
let expected_text = "0xNonce was founded".to_string();
let maybe_text = get_text_for_hex(&hex);
let text = match maybe_text {
    Ok(text) => text.to_string(),
    Err(_) => "wrong".to_string(),
};
assert_eq!(text, expected_text);
```

## License
MIT © [Joe Gesualdo]()
 

