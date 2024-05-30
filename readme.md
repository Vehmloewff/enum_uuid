# enum_uuid

Derive a static UUID at compile time for each member in an enum.

```rust
#[derive(Debug, EnumUuid)]
enum Greetings {
    Hello,
    Hi,
    Peace,
}

let id = Greetings::Peace.to_id();

println!("{:?}", Greetings::from_id(&id)); // Some(Peace)
```
