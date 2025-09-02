# Miden Standards

This crate contains common Miden contracts.

## Usage

To use the `counter` contract, add the following to your `Cargo.toml`:

```toml
[dependencies]
miden-standards = { git = "https://github.com/mmagician/miden-standards" }
```

Then, you can use the `counter` contract in your code:

```rust
use miden_standards::counter::library as counter_library;

// To compile a transaction script by dynamically linking the counter library
let tx_script = ScriptBuilder::new(true)
    .with_dynamically_linked_library(&counter_library().expect("Failed to get counter library"))
    .unwrap()
    .compile_tx_script(script_code)
    .unwrap();

// To compile the imported library into `AccountComponent` with one storage slot
let counter_component = AccountComponent::new(
    counter_library().expect("Failed to get counter library"),
    vec![StorageSlot::Value(
        [Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(0)].into(),
    )],
)
.unwrap()
.with_supports_all_types();
```

And the tx script code looks like:

```masm
use.standards::counter

begin
    call.counter::increment_count
end
```