type T1 = any extends any ? 1 : 2;
//   ^? T1

type T2 = any extends unknown ? 1 : 2;
//   ^? T2

type T3 = any extends 'a' ? 1 : 2;
//   ^? T3

type T4 = 1 extends number ? 1 : 2;
//   ^? T4
