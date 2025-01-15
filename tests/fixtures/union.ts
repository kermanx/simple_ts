let a: boolean | 3;
  a
//^? A

let b: string | "a";
  b
//^? B

let c: string | (bigint | symbol);
  c
//^? C
