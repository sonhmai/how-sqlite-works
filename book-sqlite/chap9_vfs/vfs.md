# OS Interface

```rust
pub trait IO {
    fn open_file(&self, path: &str, flags: OpenFlags, direct: bool) -> Result<Rc<dyn File>>;

    fn run_once(&self) -> Result<()>;

    fn generate_random_number(&self) -> i64;

    fn get_current_time(&self) -> String;
}
```