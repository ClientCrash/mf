# mf 
    
![Crates.io](https://img.shields.io/crates/d/mf?style=flat-square) ![GitHub repo size](https://img.shields.io/github/repo-size/clientcrash/mf?style=flat-square)
### I modify files.

### Installation

 - using cargo (rust package manager)  
        
        cargo install mf

#### Examples

This create the files foo.txt and bar.txt:

	mf create foo.txt bar.txt

This removes the files foo.txt and bar.txt:

	mf remove foo.txt bar.txt

This merges the files foo.txt and bar.txt into foobar.txt:

	mf merge foobar.txt foo.txt bar.txt

### Usage

	mf <COMMAND> [FILES ...]

	Commands:
		h, help             Print help information
		c  create           Create file/s
		m, merge <TARGET>   Merge files into target
		r, remove           Remove files

