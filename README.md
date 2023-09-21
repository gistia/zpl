# zpl-rs

Zebra Printer Language (ZPL) parser and encoder written in Rust.

## Example

You can run the example:

```bash
cargo run -q --example logo | http --form POST http://api.labelary.com/v1/printers/8dpmm/labels/2x1/0/ > output/label.png && open output/label.png
```
