# Grateful

Grateful is a command line interface (CLI) that enables you to boost your happyness by prompting you to write three things that you are grateful for every day. 

### Quick start

Installing this CLI requires `cargo`, the rust package manager. If you don't have it you can get it on mac with `brew install cargo`. Once you have `cargo` install the CLI with

```
cargo install grateful-cli
```

If you're on mac and you don't want to install `cargo` you can also just download the binary `./target/release/grateful` and add it to your `$PATH` 

Enter `grateful` into your terminal and it will prompt you to enter three things you're grateful for. Just type 

```
What are you grateful for today? (3) > beans, I really like beans
What are you grateful for today? (2) > carrots, mmm so fresh
What are you grateful for today? (1) > potatoes, can't do without 'em
```

The **only** other commands are `grateful history` and `grateful last`. These commands display all of your entries and your last entry respectively. 

### Demo 

![Example use of the cli](./img/cli-use2.png)

### FAQ

*Who is this for?* Are you an unhappy ungrateful sod who never gives thanks for the things that you have? Or perhaps you already do this but you wanna do it from the command line! 

*Why do I need `grateful-cli`?* It enables you to give thanks more easily, and to look through all the things you've been thankful for later. Everyone on the internet says that [it](https://www.health.harvard.edu/healthbeat/giving-thanks-can-make-you-happier) [will](https://www.abbott.com/corpnewsroom/nutrition-health-and-wellness/why-giving-thanks-is-good.html) [make](https://www.inc.com/jeremy-goldman/giving-thanks-31-inspiring-quotes-about-thankfulness.html) [you](https://hbr.org/2021/11/giving-thanks-at-work-an-hbr-guide) [happier](https://www.forbes.com/sites/margiewarrell/2018/11/17/the-gift-of-giving-thanks/?sh=57a5da1d468f). 

*Does this really work?* Yes it does! Doing this excersise every day actually makes you happier. An ex-psych student once told my girlfriend who told me that. 


### Todo

- [x] Implement tests
- [ ] Better handling of the args
  - [ ] Allow basic options like --version, (but maybe this is extra, it's such a tiny tool)
  - [ ] *only* `grateful` with no additional args should start the data entry prompt, currently it just runs in this mode default whenever there's an error
- [ ] Refactor 
  - [x] improve nondoc comments
  - [ ] understand docs and rust doc comments
  - [x] better file structure
  - [ ] make it more readable in general
- [ ] Better logs (trace error etc)
- [x] bugfix, `grateful last` returns the first element of the json file instead of the last one.
- [x] Find and link supporting documentation that validates claims about how this makes you happy. 
- [x] Prettier displays
- [x] Eliminate need for dummy grateful statement

If you like this cli and want a new feature just send me a message! Also, contributions are welcome.

### Appendix / MISC

Thanks [Jake](https://github.com/jakewilson/) for inspiring little crate with [tempus-cli](https://github.com/jakewilson/tempus). 

Statement of Apology: I am a really stupid person, version 0.0.1 DOESN'T WORK, I'm really sorry about that. Versions ^0.0.2 work. Please forgive me for this very poor code, it's my first package. Also, the first package I pushed was miss-spelled "greatful-cli" instead of "grateful-cli", don't install "greatful-cli" it doesn't work! I will yank it and contact the rust people to ask them about removing it. 


