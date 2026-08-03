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

If you have the butterscotch binary in your path, you can also use the "run" command to build and run the project in one step. Alternatively you can specify the path to the butterscotch binary using the `--runner` option:

```bash
gmc run
gmc run --runner /path/to/butterscotch
```