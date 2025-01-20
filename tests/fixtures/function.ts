function f1(a: number): string { 
}
const t1 = f1(1);
//    ^? T1

function f2(a: 1) {
  return a
}
const t2 = f2();
//    ^?
