# iscoredump [![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/christian-blades-cb/iscoredump.svg)](http://isitmaintained.com/project/christian-blades-cb/iscoredump "Average time to resolve an issue") [![Percentage of issues still open](http://isitmaintained.com/badge/open/christian-blades-cb/iscoredump.svg)](http://isitmaintained.com/project/christian-blades-cb/iscoredump "Percentage of issues still open")

iscoredump answers a simple question: "is this file a core dump"?

# Usage

Iscoredump returns a non-zero exit code if the provided file is not a core dump, allowing you to integrate it into your scripts.

``` shell
➜  iscoredump ./core
ELF type: Core
➜  echo $?
0
➜  iscoredump /bin/true
ELF type: Executable
➜  echo $?
1
➜  mv core icouldbeanexecutable_youdontknowme
➜  iscoredump icouldbeanexecutable_youdontknowme
ELF type: Core
➜  echo $?
0
```
