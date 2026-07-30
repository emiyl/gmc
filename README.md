# GMLC

GML compiler written in Rust. Thank you to [Butterscotch](https://github.com/https://github.com/ButterscotchRunner/Butterscotch) for the information on data.win parsing and the GML bytecode format.

It uses the WAD 17 format but it's still a little wonky. You can do basic things like calling functions, variable assignments, if statements, and binary operations. It doesn't support loops, strings, arrays or structs yet. It also doesn't support the full range of GML syntax, so some things may not work as expected.

`gmlc <file.gml> -o data.win` will compile a GML file into a data.win file. 