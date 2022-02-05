# Grateful

Grateful is a command line interface (CLI) that enables you to boost your happyness by prompting you to write three things that you are grateful for every day. 

### Quick start

**Installation**

Installing this CLI requires `cargo`, the rust package manager. If you don't have it you can get it on mac with `brew install cargo`. Once you have `cargo` install the CLI with

`cargo install grateful-cli`

**Use**

Enter 

`grateful`

into your terminal and it will prompt you to enter three things you're grateful for. Just type 

```
What are you grateful for today? (3) > beans, I really like beans
What are you grateful for today? (2) > carrots, mmm so fresh
What are you grateful for today? (1) > potatoes, can't do without 'em
```

The **only** other commands are `grateful history` and `grateful last`. These commands display all of your entries and your last entry respectively. 

### Demo video / gif

*put gif here*

### Appendix

Does this really work? Yes it does! Doing this excersise every day actually makes you happier. *link to an article*


### Todo

- [ ] Implement tests
- [ ] Better handling of the args

### MISC

Code to reset the grateful json.

```rust
use json::object;

// over-writes grateful.json to {"grateful":[]}
fn reset_json() -> io::Result<()> {
    let data = object! {
        "grateful": []
    };
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./grateful.json")?;
    f.write_all(json::stringify_pretty(data, 4u16).as_bytes())?;
    f.flush()?;

    Ok(())
}
```
