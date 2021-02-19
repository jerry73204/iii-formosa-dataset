# Formosa Dataset Toolkit in Rust

[Serde](https://docs.rs/serde)-compatible data types and parsing library for **Formosa dataset**, created by [Institute for Information Industry](https://web.iii.org.tw/) in Taiwan.

## Documentation

The [API documentation](https://docs.rs/iii-formosa-dataset/) is hosted on docs.rs.

## Usage

Add this crate to your `Cargo.toml`.

```toml
iii-formosa-dataset = "0.1"
```

Loading all samples in the dataset directory can be done by:

```rust
let samples = iii_formosa_dataset::load(&dataset_dir)?;

```

## License

The implementation of parser and dataset format, exclusive of the dataset images and label files, are distributed under MIT license.

By using this library, please cite one of the following texts.

- English

```
Formosa Dataset, created by Institute for Information Industry
```

- Traditional Chinese

```
Formosa自駕深度學習資料庫, 財團法人資訊工業策進會
```
