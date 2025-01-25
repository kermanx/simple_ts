# simple_ts

\[WIP\] The simple way to resolve/infer TypeScript types. Written in Rust.

### Goals

- Type inference implemented _within 10k LOC_
- Support [oxc's Type Aware Linting](https://github.com/oxc-project/oxc/issues/3105)
- Fast enough
- (Possibly) Fast DTS emitter without the need of `isolatedDeclarations`

### Non-goals

- Type checking
- Type hinting/LSP
- 100% TypeScript compatibility (In fact, many TypeScript behaviors are never documented)

### Status

Currently, the project is of 6k LOC.

The following TypeScript features are implemented/to be implemented:

- [x] Basic type inference
- [x] Union and intersection type
- [x] Generic type, Object type, and Tuple type
- [x] Conditional type and inference: `T extends infer U ? X : Y`
- [x] Control flow based type inference
- [x] (Partial) Generic function and inference
- [x] (Partial) Interface type
- [x] (Partial) Printing types
- [ ] Type narrowing/guards/assertions
- [ ] Enum type
- [ ] Class
- [ ] Builtin libs
- [ ] Multiple files support

For AST nodes, some are implemented and some are not.

As you can see, the hardest part (generic and it's inference and control flow analysis) is already implemented. The rest is relatively easy.

However, the rest still consumes a lot of time and effort. So, I'm not going to implement them. Please take over this project if you want to implement them. Thank you.
