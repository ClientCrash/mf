# mf 
    
![Crates.io](https://img.shields.io/crates/d/mf?style=flat-square) ![GitHub repo size](https://img.shields.io/github/repo-size/clientcrash/mf?style=flat-square)
### I work with files.

### Installation

 - With cargo (rust package manager)  
        
        cargo install mf

#### Examples
This merges file 1.txt and 2.txt into one file called target.txt    
       ` mf -m target.txt 1.txt 2.txt `

### Usage



        mf <mode> [file ... file ... file]

        modes:

        -h: Help | -c Create file/s | -m Merge file/s | -r Delete file/s

        
        !! If the mode is merge, first file is target file name !!


