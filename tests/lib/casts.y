def macro num2bool(num: number) -> bool {
    return num != 0;
}

def macro num2str(num: number) -> string {
    return num + "";
}

def macro any2num(item: any) -> number {
    var r:number = 0;
    emit { "r = item" };
    return r;
}

def macro any2str(item: any) -> string {
    var r:string = "";
    emit { "r = item" };
    return r;
}

def macro any2bool(item: any) -> bool {
    var r:bool = "";
    emit { "r = item" };
    return r;
}

main {
    var a:number = 7;
    var b:number = 10;
    :out = a + b;
}