# GMLC

GML compiler written in Rust. Thank you to [Butterscotch](https://github.com/ButterscotchRunner/Butterscotch) for the information on data.win parsing and the GML bytecode format.

It uses the WAD 17 format but it's still a little wonky. You can generate data.win files using a command like this:

```bash
gmlc --compile-script ./file.gml -o ./data.win
```

Or if you require a project with rooms and objects, you can use the following command:

```bash
gmlc --create Project \
    --add-resource room Room1 \
    --add-resource object Object1 \
    --add-object-to-room Room1 Object1 100 120 \
    --add-event Object1 Create 0 "show_debug_message(\"Hello, world!\");" \
    -o ./data.win
```

This will create a project with a room and an object, add the object to the room, and add a Create event to the object that shows a debug message. You can also use a .gml file for the event code instead of a string.