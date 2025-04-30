type Uppercase<S extends string> = intrinsic;

type Lowercase<S extends string> = intrinsic;

type Capitalize<S extends string> = intrinsic;

type Uncapitalize<S extends string> = intrinsic;

// TODO:
// type NoInfer<T> = intrinsic;

type T1 = Uppercase<'hello'>;
//   ^? T1

type T2 = Lowercase<'HELLO'>;
//   ^? T2

type T3 = Capitalize<'hello world'>;
//   ^? T3

type T4 = Uncapitalize<'Hello World'>;
//   ^? T4
