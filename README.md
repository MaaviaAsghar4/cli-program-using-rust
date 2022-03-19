### MiniGrep Using Rust

This is the rust cli program that has the same function as linux grep but with limited functionality

#### Run the program
To run the program;
- Make sure you have the repo cloned and rust installed on your system
- Go to the repo directory and open the terminal
- Run the following command 
```
cargo run <String to search> <path to file>
```
e.g cargo run demostring demo.txt

###### For case insensitive search
Set the env variable CASE_INSENSITIVE=1
or run
```
CASE_INSENSITIVE=1 cargo run <String to search> <path to file>
```

###### To get the output in a file
```
cargo run <String to search> <path to file> > <output file name>
```