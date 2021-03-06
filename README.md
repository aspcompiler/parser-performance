# parser-performance

This is my playground to evaluate the performance of various frameworks and programming languages. In the first
iteration, I will focus on the Json parser as this is an area people have done good optimization in their
perspective frameworks and languages so we can have a fair comparison.

The test data is a 5000 objects Json file generated from  json-generator.com and is around 5.2 MB in size. 

The first time is measured using macOS/Linux `time` command. The second time is measured using the timer function embedded in the code.
This excludes the time that python parses python script as it is a valid real world scenario that one loads the python program once
and parses several scripts.

Memory is measured using the [memusg script](https://gist.github.com/netj/526585) which is copied to the script folder.

## Results

| Parser |  Time(s) |  Peak memory(MB) |
|---|---|---|
| Python/lark | 5.270/5.096 | 59.344 |
| Rust/nom (1) | 0.213/0.204 | 27.828 |
| Rust/pest (2) | 0.126/0.116 | 51.240 |
| Rust/serde | 0.097/0.054 | 3.152 (3) |

1. Unoptimized Rust nom parser is 23+ times faster than the flagship Python/lark parser and uses < 47% of peak memory.

2. Although [Pest](https://pest.rs/) that their parser is slower than nom, our test shows that it is no slower. It is possible that our
nom parser is unoptimized.

3. The memory number is strange because we load the entire Json into memory and our Json is over 5 MB. It is possible that that `memuse` program
failed to catch the peak.

## Python

### python-lark-json

I just follow [this](https://github.com/lark-parser/lark/blob/master/docs/json_tutorial.md).

Change to `python-lark-json` directory. Create a Python virtual environment, activate and then run `python -m pip install -requirements.txt`.

Then run:

```
time python json_parser.py ../data/5000_objects.json
```

Results: 

```
5.03s user 0.06s system 99% cpu 5.127 total
```

```
../scripts/memusg python json_parser.py ../data/5000_objects.json
```

Results:

```
memusg: peak=59344
```

## Rust

### rust-nom-json

The parser is adapter from [nom json example](https://github.com/Geal/nom/blob/main/examples/json.rs)

To compiler, from rust-nom-json directory, run:

```
cargo build --release
```

Time test:

```
time ./target/release/rust-nom-json ../data/5000_objects.json
```

Results:

```
0.19s user 0.02s system 97% cpu 0.215 total
```

Memory test:

```
../scripts/memusg ./target/release/rust-nom-json ../data/5000_objects.json
```

Results:

```
 memusg: peak=27828
```

### rust-pest-json

The parser is adapter from [pest json example](https://pest.rs/book/examples/json.html)

To compiler, from rust-pest-json directory, run:

```
cargo build --release
```

Time test:

```
time ./target/release/rust-pest-json ../data/5000_objects.json
```

Results:

```
0.10s user 0.02s system 96% cpu 0.126 total
```

Memory test:

```
../scripts/memusg ./target/release/rust-nom-json ../data/5000_objects.json
```

Results:

```
 memusg: peak=51240
```

### rust-serde-json

The parser simply uses [serde_json](https://docs.serde.rs/serde_json/index.html).

To compiler, from rust-serde-json directory, run:

```
cargo build --release
```

Time test:

```
time ./target/release/rust-serde-json ../data/5000_objects.json
```

Results:

```
0.07s user 0.02s system 94% cpu 0.097 total
```

Memory test:

```
../scripts/memusg ./target/release/rust-nom-json ../data/5000_objects.json
```

Results:

```
 memusg: peak=3152
```