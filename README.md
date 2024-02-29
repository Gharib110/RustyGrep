# RustyGrep
The grep cmd in GNU Linux Operating Systems that is implemented in Rust

## Example
`` cargo run to ./file.txt `` <br/>
The first argument is the search query and the second argument is the file path which could be a relative or an absolute path.
The Content of the file:
```
Hello Where is it?
He is going to the shop !
```
The Output:
```
search result of to in ./file.txt
He is going to the shop !
```
