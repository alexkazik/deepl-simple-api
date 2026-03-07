# crate deepl-simple-api

<!-- cargo-rdme start -->

Simple `DeppL` API.

Currently supported:
- translation
- usage

Free and paid keys are supported.

Uses `reqwest` as a http client, optionally supports blocking.

```rust
let deepl = DeepL::new("API-KEY-HERE");
let options = Options::builder()
    .params(&[&TargetLanguage("DE"), &SourceLanguage("EN"), &Formality::PreferLess])
    .build();
let translated = deepl.translate(&options, &["Hello World"]).await?;
```

<!-- cargo-rdme end -->
