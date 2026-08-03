# GMC

GML compiler written in Rust. Thank you to [Butterscotch](https://github.com/ButterscotchRunner/Butterscotch) for the information on data.win parsing and the GML bytecode format.

It uses the WAD 17 format but it's still a little wonky. You can generate data.win files using a command like this:

```bash
gmc new project
cd project
gmc add room Room1
gmc add object Object1
gmc object add event Object1 Create
gmc room add instance Room1 Object1 0 0
gmc build
```

This will output a data.win file in the `build` directory. You can then run this data.win file using Butterscotch.