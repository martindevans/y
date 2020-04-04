import "yunit.y";
import "numbers.y";

def macro parse_base10_char(input: string, output: number, counter: number) {
    var c:string = input - --input;
    var d:number = 3 * ((c > 1) + (c > 4) + (c > 7));
    output += (d + (c > d) - (c < d)) * 10 ^ counter++;
}

def macro parse_base16_char(input: string, output: number, counter: number) {
    const x:string = "FDB97531";
    const y:string = "FEBA7632";

    var c:string = input - --input;
    output += (4 * ((c > 3) + (c > 7) + (c > "B")) + (x > x - c) + 2 * (y > y - c)) * 16 ^ counter++;
}

main {
    line(init) {
        const i:string = "8237897";
        var o:number = 0;
        var c:number = 0;
    };
    line(parse) {
        parse_base10_char(i, o, c);
        goto parse;
    };
    line(check) {
        :assert = 8237897 == o;
    };
}