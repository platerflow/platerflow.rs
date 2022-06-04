# Plater Flow support app

This is my personal version of airox' awesome work on PlaterFlow. 

This is tailored to *my* needs and *my* needs only.

## Limitations

* Superslicer is quite chatty 


## Install
Download from releases or build yourself. 

## Usage
1) Download from releases or build yourself.
2) Move the downloaded platerflow executable to a folder of it's own.
3) Open a terminal and navigate to where platerflow is located
4) Run platerflow (Windows: platerflow.exe, Linux: ./platerflow, MacOS: ./platerflow (might need to chmod to executable in Linux/Mac case))
5) You will see newly generated files and folders.
6) Edit config.toml to your needs. Keep in mind: Windows is stupid, escape your \ with another \; the example config generated shows this too.
7) Put STLs you want sorted into the input folder. Platerflow will recognize \[a\] files as accent files and will read the number of times it needs to print this too. Directories are fine too and will be searched recursively.
8) Run platerflow again (see 4)
9) Profit???