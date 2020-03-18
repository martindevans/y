import "intrinsics.y";

type range<number> positive -> positive > 0;
type range<number> positive_or_zero -> positive >= 0;
type range<number> negative -> negative < 0;
type range<number> negative_or_zero -> negative <= 0;

type range<number> integer -> integer / 1000 * 1000 == integer;
type range<integer> natural -> natural > 0;

type range<number> square -> sqrt(square) is integer;