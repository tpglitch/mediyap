# MediYap

A Rust library for decoding medical terminology into plain English by breaking down prefixes, suffixes, and root words.

## Features

- Decode complex medical terms into readable explanations
- (fairly) Extensive dictionary of medical prefixes, suffixes, and roots
- Zero dependencies
- Well-tested
- Incredible cli

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mediyap = "0.1.1"
```

## Usage

```rust
use mediyap::MedicalDecoder;

fn main() {
    let decoder = MedicalDecoder::new();

    println!("{}", decoder.decode("hypoglycemia"));
    // Output: low glucose/sugar presence in blood

    println!("{}", decoder.decode("tachycardia"));
    // Output: fast heart

    println!("{}", decoder.decode("nephritis"));
    // Output: kidney inflammation
}
```

### As a CLI Tool

After installation, you can use the `mediyap` command:

```bash
# Decode a single term
mediyap hypoglycemia
# Output: hypoglycemia: low glucose/sugar presence in blood

# Decode multiple terms
mediyap tachycardia bradycardia nephritis
# Output:
# tachycardia: fast heart
# bradycardia: slow heart
# nephritis: kidney inflammation

# Interactive mode (no arguments)
mediyap
# MediYap - Interactive Mode
# Enter medical terms to decode (Ctrl+D or Ctrl+C to exit):
#
# > hypoglycemia
# → low glucose/sugar presence in blood
#
# > arthritis
# → joint inflammation
```

## Examples

| Medical Term     | Decoded Meaning                      |
| ---------------- | ------------------------------------ |
| hypoglycemia     | low glucose/sugar presence in blood  |
| hyperglycemia    | high glucose/sugar presence in blood |
| tachycardia      | fast heart                           |
| bradycardia      | slow heart                           |
| leukemia         | white presence in blood              |
| anemia           | without blood                        |
| thrombocytopenia | clot cell deficiency                 |
| arthritis        | joint inflammation                   |
| gastritis        | stomach inflammation                 |
| cardiomegaly     | heart enlargement                    |

## Supported Components

### Prefixes

- `hypo-` (low)
- `hyper-` (high)
- `brady-` (slow)
- `tachy-` (fast)
- `a-/an-` (without)
- `poly-` (many)
- `oligo-` (few)
- And many more...

### Suffixes

- `-emia` (presence in blood)
- `-itis` (inflammation)
- `-uria` (presence in urine)
- `-pathy` (disease)
- `-penia` (deficiency)
- `-algia` (pain)
- `-ectomy` (surgical removal)
- And many more...

### Root Words

- `glyc/gluc` (glucose/sugar)
- `card/cardi` (heart)
- `hem/hemat` (blood)
- `nephr/ren` (kidney)
- `gastr` (stomach)
- `hepat` (liver)
- And many more...

## Running Tests

```bash
cargo test
```

## Contributing

Contributions are welcome! Feel free to submit pull requests to add more medical terms or improve the decoding logic.

## License

This project is licensed under the MIT license. Read the [LICENSE](./LICENSE) file for more info.
