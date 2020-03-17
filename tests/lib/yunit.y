import "constants.y";

[cfg("test")] def macro assert(a:bool, msg:string) {
    if (!a) {
        :assert_fail_msg = msg;
    };
}

[cfg("test")] def macro assert_eq(a:any, b:any, msg:string) {
    assert(a == b, msg);
}

[cfg("test")] def macro assert_neq(a:any, b:any, msg:string) {
    assert(a != b, msg);
}

main {
    assert(true, "true");
    assert_eq(1, 1, "1 == 1");
    assert_neq(1, 2, "1 != 2");
}