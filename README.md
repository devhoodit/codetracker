# Code Tracker
Tracking code lines and characters all subdirectories

## How to use
Download codetracker - [Link](https://github.com/devhoodit/codetracker/releases/tag/v0.1.0)
  
Move file to directory where you want  
Set environment variable, so that call codetracker anywhere you want  
In my case, i move ct.ext to C:/codetracker (make new directory here)
And add environment variable "C:/codetracker"  

### Options
```
-e [extensions] : tracking these extension files
-q : quiet mode : tracking does not print Directory entry or File entry 
```
example
tracking current directory with py, rs, js extensions (quiet option)
```powershell
ct ./ -q -e py rs js
```