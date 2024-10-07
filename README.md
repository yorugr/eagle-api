# Eagel API

Rust implementataions of [Eagle API](https://api.eagle.cool/).

## Usage

```rust
use eagle_api::EagleApi;

async {
    let api = EagleApi::new("http://localhost:41595");
    let items = api.item_list(None).await.unwrap();
    println!("{:#?}", items);
};
```
