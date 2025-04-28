enum A {
  X,
  Y,
  Z,
}

type A2 = A;
//   ^? A2

type T1 = A.X | A.Y;
//   ^? T1

type T2 = A.X extends A.Y ? true : false;
//   ^? T2
