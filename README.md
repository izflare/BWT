# BWT

### Description

This is a convertor: input text to Burrows-Wheeler transformed text of it, and back.
This implementation uses SA-IS as suffix array construction algorithm (see https://github.com/izflare/sais)

### Download

To clone the repository, call

```
git clone https://github.com/izflare/bwt.git
```

### Compile

This code has been tested under linux compiling with rust (cargo) ver 1.32.0
After download the repository, 

```
cd bwt
cargo build --release
```

### Run

After compiling,

```
cd target/release
./bwt [-r] --input <input>
```

then the tool run.  
`<input>` is your input text data file.
Elapsed time for running will be displayed, and converted text file will be outputted as `<input>.bwt`.
If you want to get the original source text from `.bwt` file, set '-r' flag
(`<input>.rev` file will be outputted).

