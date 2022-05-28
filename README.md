# Plater Flow support app

This is my personal version of airox' awesome work on PlaterFlow. 
This is tailored to *my* needs and *my* needs only.

Support for OSX has been pulled due to dependencies not working on OSX.
## Limitations

~* SuperSlicer does not output thumbnails when used in CLI mode, sadly.~ Found a workaround with stl-thumb


## Install
Download from releases or build yourself:

![image](https://user-images.githubusercontent.com/227830/158068869-dd6cb941-8bd0-451b-abf4-5213a5f3be55.png)

## Usage
To better see what platerflow is doing it's best you open up a command terminal if you're on Windows.
Upon first use a config.toml will be created. Edit it to your needs, then run it again. 

**Keep in mind if you're on Windows to escape your \\. The first-run generated config.toml will show double \\, this is needed because Windows is retarded and you should do it too.**

Add all the STLs you want sorted in the input folder. PlaterFlow will search recursively so directories are fine too.


![image](https://user-images.githubusercontent.com/227830/158069084-b97994f7-11f7-482e-baba-c36c8a8f8023.png)
