# GMC

GML compiler written in Rust. Thank you to [Butterscotch](https://github.com/ButterscotchRunner/Butterscotch) for the information on data.win parsing and the GML bytecode format.

It uses the WAD 17 format but it's still a little wonky. You can generate data.win files using a command like this:

```bash
gmc new <project_name>
cd <project_name>
gmc add room <room_name>
gmc add object <object_name>
gmc object add event <object_name> Create
gmc room add instance <room_name> <object_name> 0 0
gmc build
```

If you have the butterscotch binary in your path, you can also use the "run" command to build and run the project in one step. Alternatively you can specify the path to the butterscotch binary using the `--runner` option:

```bash
gmc run
gmc run --runner /path/to/butterscotch
```