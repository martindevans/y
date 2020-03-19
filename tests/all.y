import "file.y" in namespace;
// import that thing
import "x/y/z.y" in ns;
//import "no/such/thing";
import "lib/number_parser.y";

// Define an enum extending the number type
type enum<number> struct_name {
    a(todo_enum_value),
    b(todo_enum_value),
    c(todo_enum_value)
}

type struct name {
    foo : bar,
    bash: baz
}

// type range<number> positive => positive > 0;
type range<number> negative => todo_range_expression;

def proc foo(foo: bar, bash:baz) { todo_body }
def macro bar(copy bash:baz)
{
    todo_body
}
def macro bar(bar:   foo, copy x:y) {
    todo_body
}

/* comment 
comment */
main { // another comment
    todo_body

    panic("oh no");
}