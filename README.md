# Grateful

Grateful is a command line interface (CLI) that enables you to boost your happyness by prompting you to write three things that you are grateful for every day. 

### Quick start

Install with this command *(macOS?, windows?, linux? does it work cross platform?)*

`command to install`

Enter 

`grateful`

into your terminal and it will prompt you to enter three things you're grateful for. 

```
What are you grateful for today? (3) > beans
What are you grateful for today? (2) > carrots
What are you grateful for today? (1) > potatoes
```

The **only** other commands are `grateful history` and `grateful last`. These commands display all of your entries and your last entry respectively. 

### Demo video / gif

*put gif here*

### Appendix

Does this really work? Yes it does! Doing this excersise every day actually makes you happier. *link to an article*


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
