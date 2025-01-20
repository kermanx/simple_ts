function f1(a: number): string { 
}
const t1 = f1(1);
//    ^? T1

function f2(a: 2) {
  return a
}
const t2 = f2();
//    ^? T2
