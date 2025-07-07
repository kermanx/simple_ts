declare namespace N1 {
  const A: "a";
  function f(a: string): number;

  namespace N2 {
    enum A {
      A = 1
    }

    type B = 2
  }
}

const t1 = N1.A
//    ^? T1

const t2 = N1.f(a)
//    ^? T2

const t3 = N1.N2.A.A
//    ^? T3

type T4 = N1.N2.B
//    ^? T4
