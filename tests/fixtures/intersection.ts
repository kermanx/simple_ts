const t1: string & number;
   t1
// ^? T1

const t2: number & 2;
   t2
// ^? T2

const t3: string & { a: number };
   t3
// ^? T3

const t4: string & (1 | 2)
   t4
// ^? T4

const t5: string & ('a' | number)
   t5
// ^? T5

const t6: (1 | 2 | 'a' | 1n) & ('a' | 1 | bigint)
   t6
// ^? T6
