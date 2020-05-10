# mao

Markov Algorithm Oracle

Inspired by [MAO](https://mao.snuke.org/)

## What is Markov Algorithm?

- Go to [MAO](https://mao.snuke.org/)

## Syntax for Markov Algorithm?

- Please go to [MAO](https://mao.snuke.org/)

## How works

```bash
$ cat ./sample.mao
# count 'o'
9o:o0
8o:9
7o:8
6o:7
5o:6
4o:5
3o:4
2o:3
1o:2
0o:1
o:1

$ cargo run ./sample.mao <<< oooooooooooo
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
    Running `target/debug/mao ./sample.mao`
12
```

## Build & Install

You need cargo ([cargo installation is very easy!](https://doc.rust-lang.org/cargo/getting-started/installation.html)).

```bash
make build
make user-install  # Put binary at ~/.local/bin
```
