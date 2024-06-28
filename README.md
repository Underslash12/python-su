## python-su

This is a tool I wrote to quickly create a test python workspace for trying stuff out.
```
Usage: python-su [options] name 

Options:
  -h, --help                Prints this help message
  -d, --dir <DIRECTORY>     Create project in the specified directory, 
                                by default this is the working directory
  -f, --folder <FOLDER>     Create project files in the specified folder,
                                by default this is the project name
  -F                        Use the directory as the project folder
                                this should not be used with the -f flag

Arguments:
  name                      Name of the project"; 
```
In the future I may add venv and/or git support.