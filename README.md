# GMC

GML compiler written in Rust. Thank you to [Butterscotch](https://github.com/ButterscotchRunner/Butterscotch) for the information on data.win parsing and the GML bytecode format.

It uses the WAD 17 format but it's still a little wonky. You can generate data.win files using a command like this:

```bash
gmc new project
gmc project/project.yyp add room Room1
gmc project/project.yyp add object Object1
gmc project/project.yyp object add event Object1 Create
gmc project/project.yyp room add instance Room1 Object1 0 0
```

Currently I'm rewriting a few things so you'll have to go back a commit or two if you want to actually compile anything.