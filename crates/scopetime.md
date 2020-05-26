# Crates

## scopetime

```rust
fn foo(){
    scopetime!("foo");
}
```

log: 
```
19:45:00 [TRACE] (7) scopetime: [scopetime/src/lib.rs:34] 
    scopetime: 2 ms [my_crate::foo] @my_crate/src/bar.rs:5
```