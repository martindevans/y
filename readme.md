## Y Language Compiler

Y is a C like language which compiles into [Yolol](https://wiki.starbasegame.com/index.php/YOLOL). Y is intended to be a completely zero cost abstraction over Yolol, suitable as a _total replacement_ of the underlying language.

### Example

#### Types

```
type struct coordinate {
    x: number,
    y: number,
    z: number
}

type enum<string> color {
    r("red"),
    g("green"),
    b("blue")
}

type range<number> negative => negative < 0;
```

Structs are a collection of fields which can be accessed together. An enum is a set of values taken from another type, in the example above the `color` enum is a subset of the `string` type and the string "green" can be expressed as `color.g` in code. Ranges express a set of values which are valid instances of this type as an expression, in the example above the `negative` range accepts all numbers which are less than zero.

#### Procedures And Macros

```
def proc add(a: number, b: number) {
    return a + b;
}

def macro add(a: number, b: number)
{
    return a + b
}
```

Procedures are compiled with a `goto` which jumps into the procedure and then jumps back to the call site, this is slow but keeps code size small for procedures which are used several times. Conversely macros are copied in line to where they are used, this means that a macro used multiple times can bloat code size but is faster to execute (no 400ms delay caused by the 2 `goto` statements). All arguments are passed **by reference**.

#### Importing Files

```
import "a.y";
import "b.y" in namespace;
```

Files of `Y` code can be imported into another file of `Y` code. This parses the imported file and makes all the items in it (types, procedures, macros etc) available for use. Files can additionally be imported in a namespace, which prepends the namespace name to the name of the imported item. For example if `Add` is defined in `b.y` it would be named `namespace:add` in this file.

#### Main Block

```
main {
    line(add_loop) {
        :output = add(:left, :right);
        goto add_loop;
    }
}

A `Y` file is compiled to a single Yolol chip script. The `main` block defines the entry point of the code. It is optional, if not included compilation will result in an empty output.