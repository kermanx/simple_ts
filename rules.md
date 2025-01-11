
- ✅ = Implemented in Oxc
- ➕ = Can be implemented without ecma_analyzer, but not implemented yet
- ✨ = Will be implementable with ecma_analyzer
- ❌ = Can't be implemented with ecma_analyzer

---

## Already Implemented in Oxc

- adjacent-overload-signatures
- array-type
- ban-tslint-comment
- ban-ts-comment
- ban-types
- consistent-generic-constructors
- consistent-indexed-object-style
- consistent-type-definitions
- consistent-type-imports
- explicit-function-return-type
- no-confusing-non-null-assertion
- no-duplicate-enum-values
- no-dynamic-delete
- no-empty-interface
- no-empty-object-type
- no-explicit-any
- no-extraneous-class
- no-extra-non-null-assertion
- no-import-type-side-effects
- no-inferrable-types
- no-misused-new
- no-namespace
- no-non-null-asserted-nullish-coalescing
- no-non-null-asserted-optional-chain
- no-non-null-assertion
- no-require-imports
- no-this-alias
- no-unnecessary-type-constraint
- no-unsafe-declaration-merging
- no-unsafe-function-type
- no-useless-empty-export
- no-var-requires
- no-wrapper-object-types
- prefer-as-const
- prefer-enum-initializers
- prefer-for-of
- prefer-function-type
- prefer-literal-enum-member
- prefer-namespace-keyword
- prefer-ts-expect-error
- triple-slash-reference

---

## typescript-eslint's strict-type-checked-only rule set

> https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/eslint-plugin/src/configs/strict-type-checked-only.ts

- ✅ = Ok
- ❔ = Unknown

- ✅ https://tseslint.com/rules/await-thenable
- ✅ https://tseslint.com/rules/no-array-delete
- ✅ https://tseslint.com/rules/no-base-to-string
- ✅ https://tseslint.com/rules/no-confusing-void-expression
- ✅ https://tseslint.com/rules/no-deprecated
- ✅ https://tseslint.com/rules/no-duplicate-type-constituents
- ✅ https://tseslint.com/rules/no-floating-promises
- ✅ https://tseslint.com/rules/no-for-in-array
- ✅ https://tseslint.com/rules/no-implied-eval
- ✅ https://tseslint.com/rules/no-meaningless-void-operator
- ✅ https://tseslint.com/rules/no-misused-promises
- ✅ https://tseslint.com/rules/no-mixed-enums
- ✅ https://tseslint.com/rules/no-redundant-type-constituents
- ✅ https://tseslint.com/rules/no-unnecessary-boolean-literal-compare
- ✅ https://tseslint.com/rules/no-unnecessary-condition
- ✅ https://tseslint.com/rules/no-unnecessary-template-expression
- ✅ https://tseslint.com/rules/no-unnecessary-type-arguments
- ✅ https://tseslint.com/rules/no-unnecessary-type-assertion
- ✅ https://tseslint.com/rules/no-unnecessary-type-parameters
- ✅ https://tseslint.com/rules/no-unsafe-argument
- ✅ https://tseslint.com/rules/no-unsafe-assignment
- ✅ https://tseslint.com/rules/no-unsafe-call
- ✅ https://tseslint.com/rules/no-unsafe-enum-comparison
- ✅ https://tseslint.com/rules/no-unsafe-member-access
- ✅ https://tseslint.com/rules/no-unsafe-return
- ✅ https://tseslint.com/rules/no-unsafe-unary-minus
- ✅ https://tseslint.com/rules/only-throw-error
- ✅ https://tseslint.com/rules/prefer-promise-reject-errors
- ✅ https://tseslint.com/rules/prefer-reduce-type-parameter
- ✅ https://tseslint.com/rules/prefer-return-this-type
- ✅ https://tseslint.com/rules/related-getter-setter-pairs
- ✅ https://tseslint.com/rules/require-await
- ✅ https://tseslint.com/rules/restrict-plus-operands
- ✅ https://tseslint.com/rules/restrict-template-expressions
- ✅ https://tseslint.com/rules/return-await
- ✅ https://tseslint.com/rules/unbound-method
- ✅ https://tseslint.com/rules/use-unknown-in-catch-callback-variable

---

https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/eslint-plugin/src/configs/stylistic-type-checked-only.ts

- ✅ https://tseslint.com/rules/dot-notation
- ✅ https://tseslint.com/rules/non-nullable-type-assertion-style
- ✅ https://tseslint.com/rules/prefer-find
- ✅ https://tseslint.com/rules/prefer-includes
- ✅ https://tseslint.com/rules/prefer-nullish-coalescing
- ✅ https://tseslint.com/rules/prefer-optional-chain
- ✅ https://tseslint.com/rules/prefer-regexp-exec
- ✅ https://tseslint.com/rules/prefer-string-starts-ends-with

---



### https://tseslint.com/rules/adjacent-overload-signatures
Require that function overload signatures be consecutive
✅


### https://tseslint.com/rules/array-type
Require consistently using either T[] or Array<T> for arrays
✅

### https://tseslint.com/rules/await-thenable
Disallow awaiting a value that is not a Thenable
✨

### https://tseslint.com/rules/ban-ts-comment
Disallow @ts-<directive> comments or require descriptions after directives
✅

### https://tseslint.com/rules/ban-tslint-comment
Disallow // tslint:<rule-flag> comments
✅

### https://tseslint.com/rules/class-literal-property-style
Enforce that literals on classes are exposed in a consistent style


### https://tseslint.com/rules/class-methods-use-this
Enforce that class methods utilize this


### https://tseslint.com/rules/consistent-generic-constructors
Enforce specifying generic type arguments on type annotation or constructor name of a constructor call
✅

### https://tseslint.com/rules/consistent-indexed-object-style
Require or disallow the Record type
✅

### https://tseslint.com/rules/consistent-return
Require return statements to either always or never specify values


### https://tseslint.com/rules/consistent-type-assertions
Enforce consistent usage of type assertions


### https://tseslint.com/rules/consistent-type-definitions
Enforce type definitions to consistently use either interface or type


### https://tseslint.com/rules/consistent-type-exports
Enforce consistent usage of type exports


### https://tseslint.com/rules/consistent-type-imports
Enforce consistent usage of type imports


### https://tseslint.com/rules/default-param-last
Enforce default parameters to be last


### https://tseslint.com/rules/dot-notation
Enforce dot notation whenever possible


### https://tseslint.com/rules/explicit-function-return-type
Require explicit return types on functions and class methods


### https://tseslint.com/rules/explicit-member-accessibility
Require explicit accessibility modifiers on class properties and methods


### https://tseslint.com/rules/explicit-module-boundary-types
Require explicit return and argument types on exported functions' and classes' public class methods


### https://tseslint.com/rules/init-declarations
Require or disallow initialization in variable declarations


### https://tseslint.com/rules/max-params
Enforce a maximum number of parameters in function definitions


### https://tseslint.com/rules/member-ordering
Require a consistent member declaration order


### https://tseslint.com/rules/method-signature-style
Enforce using a particular method signature syntax


### https://tseslint.com/rules/naming-convention
Enforce naming conventions for everything across a codebase


### https://tseslint.com/rules/no-array-constructor
Disallow generic Array constructors
➕

### https://tseslint.com/rules/no-array-delete
Disallow using the delete operator on array values
✨

### https://tseslint.com/rules/no-base-to-string
Require .toString() and .toLocaleString() to only be called on objects which provide useful information when stringified


### https://tseslint.com/rules/no-confusing-non-null-assertion
Disallow non-null assertion in locations that may be confusing


### https://tseslint.com/rules/no-confusing-void-expression
Require expressions of type void to appear in statement position


### https://tseslint.com/rules/no-deprecated
Disallow using code marked as @deprecated


### https://tseslint.com/rules/no-dupe-class-members
Disallow duplicate class members


### https://tseslint.com/rules/no-duplicate-enum-values
Disallow duplicate enum member values


### https://tseslint.com/rules/no-duplicate-type-constituents
Disallow duplicate constituents of union or intersection types


### https://tseslint.com/rules/no-dynamic-delete
Disallow using the delete operator on computed key expressions
✨

### https://tseslint.com/rules/no-empty-function
Disallow empty functions


### https://tseslint.com/rules/no-empty-interface
Disallow the declaration of empty interfaces


### https://tseslint.com/rules/no-empty-object-type
Disallow accidentally using the "empty object" type


### https://tseslint.com/rules/no-explicit-any
Disallow the any type


### https://tseslint.com/rules/no-extra-non-null-assertion
Disallow extra non-null assertions


### https://tseslint.com/rules/no-extraneous-class
Disallow classes used as namespaces


### https://tseslint.com/rules/no-floating-promises
Require Promise-like statements to be handled appropriately


### https://tseslint.com/rules/no-for-in-array
Disallow iterating over an array with a for-in loop


### https://tseslint.com/rules/no-implied-eval
Disallow the use of eval()-like methods


### https://tseslint.com/rules/no-import-type-side-effects
Enforce the use of top-level import type qualifier when an import only has specifiers with inline type qualifiers


### https://tseslint.com/rules/no-inferrable-types
Disallow explicit type declarations for variables or parameters initialized to a number, string, or boolean


### https://tseslint.com/rules/no-invalid-this
Disallow this keywords outside of classes or class-like objects


### https://tseslint.com/rules/no-invalid-void-type
Disallow void type outside of generic or return types


### https://tseslint.com/rules/no-loop-func
Disallow function declarations that contain unsafe references inside loop statements


### https://tseslint.com/rules/no-loss-of-precision
Disallow literal numbers that lose precision


### https://tseslint.com/rules/no-magic-numbers
Disallow magic numbers


### https://tseslint.com/rules/no-meaningless-void-operator
Disallow the void operator except when used to discard a value


### https://tseslint.com/rules/no-misused-new
Enforce valid definition of new and constructor


### https://tseslint.com/rules/no-misused-promises
Disallow Promises in places not designed to handle them


### https://tseslint.com/rules/no-mixed-enums
Disallow enums from having both number and string members


### https://tseslint.com/rules/no-namespace
Disallow TypeScript namespaces


### https://tseslint.com/rules/no-non-null-asserted-nullish-coalescing
Disallow non-null assertions in the left operand of a nullish coalescing operator


### https://tseslint.com/rules/no-non-null-asserted-optional-chain
Disallow non-null assertions after an optional chain expression


### https://tseslint.com/rules/no-non-null-assertion
Disallow non-null assertions using the ! postfix operator


### https://tseslint.com/rules/no-redeclare
Disallow variable redeclaration


### https://tseslint.com/rules/no-redundant-type-constituents
Disallow members of unions and intersections that do nothing or override type information


### https://tseslint.com/rules/no-require-imports
Disallow invocation of require()


### https://tseslint.com/rules/no-restricted-imports
Disallow specified modules when loaded by import


### https://tseslint.com/rules/no-restricted-types
Disallow certain types


### https://tseslint.com/rules/no-shadow
Disallow variable declarations from shadowing variables declared in the outer scope


### https://tseslint.com/rules/no-this-alias
Disallow aliasing this


### https://tseslint.com/rules/no-type-alias
Disallow type aliases


### https://tseslint.com/rules/no-unnecessary-boolean-literal-compare
Disallow unnecessary equality comparisons against boolean literals


### https://tseslint.com/rules/no-unnecessary-condition
Disallow conditionals where the type is always truthy or always falsy


### https://tseslint.com/rules/no-unnecessary-parameter-property-assignment
Disallow unnecessary assignment of constructor property parameter


### https://tseslint.com/rules/no-unnecessary-qualifier
Disallow unnecessary namespace qualifiers


### https://tseslint.com/rules/no-unnecessary-template-expression
Disallow unnecessary template expressions


### https://tseslint.com/rules/no-unnecessary-type-arguments
Disallow type arguments that are equal to the default


### https://tseslint.com/rules/no-unnecessary-type-assertion
Disallow type assertions that do not change the type of an expression


### https://tseslint.com/rules/no-unnecessary-type-constraint
Disallow unnecessary constraints on generic types


### https://tseslint.com/rules/no-unnecessary-type-parameters
Disallow type parameters that aren't used multiple times


### https://tseslint.com/rules/no-unsafe-argument
Disallow calling a function with a value with type any


### https://tseslint.com/rules/no-unsafe-assignment
Disallow assigning a value with type any to variables and properties


### https://tseslint.com/rules/no-unsafe-call
Disallow calling a value with type any


### https://tseslint.com/rules/no-unsafe-declaration-merging
Disallow unsafe declaration merging


### https://tseslint.com/rules/no-unsafe-enum-comparison
Disallow comparing an enum value with a non-enum value


### https://tseslint.com/rules/no-unsafe-function-type
Disallow using the unsafe built-in Function type


### https://tseslint.com/rules/no-unsafe-member-access
Disallow member access on a value with type any


### https://tseslint.com/rules/no-unsafe-return
Disallow returning a value with type any from a function


### https://tseslint.com/rules/no-unsafe-type-assertion
Disallow type assertions that narrow a type


### https://tseslint.com/rules/no-unsafe-unary-minus
Require unary negation to take a number


### https://tseslint.com/rules/no-unused-expressions
Disallow unused expressions


### https://tseslint.com/rules/no-unused-vars
Disallow unused variables


### https://tseslint.com/rules/no-use-before-define
Disallow the use of variables before they are defined


### https://tseslint.com/rules/no-useless-constructor
Disallow unnecessary constructors


### https://tseslint.com/rules/no-useless-empty-export
Disallow empty exports that don't change anything in a module file


### https://tseslint.com/rules/no-var-requires
Disallow require statements except in import statements


### https://tseslint.com/rules/no-wrapper-object-types
Disallow using confusing built-in primitive class wrappers


### https://tseslint.com/rules/non-nullable-type-assertion-style
Enforce non-null assertions over explicit type assertions


### https://tseslint.com/rules/only-throw-error
Disallow throwing non-Error values as exceptions


### https://tseslint.com/rules/parameter-properties
Require or disallow parameter properties in class constructors


### https://tseslint.com/rules/prefer-as-const
Enforce the use of as const over literal type


### https://tseslint.com/rules/prefer-destructuring
Require destructuring from arrays and/or objects


### https://tseslint.com/rules/prefer-enum-initializers
Require each enum member value to be explicitly initialized


### https://tseslint.com/rules/prefer-find
Enforce the use of Array.prototype.find() over Array.prototype.filter() followed by [0] when looking for a single result


### https://tseslint.com/rules/prefer-for-of
Enforce the use of for-of loop over the standard for loop where possible


### https://tseslint.com/rules/prefer-function-type
Enforce using function types instead of interfaces with call signatures


### https://tseslint.com/rules/prefer-includes
Enforce includes method over indexOf method


### https://tseslint.com/rules/prefer-literal-enum-member
Require all enum members to be literal values


### https://tseslint.com/rules/prefer-namespace-keyword
Require using namespace keyword over module keyword to declare custom TypeScript modules


### https://tseslint.com/rules/prefer-nullish-coalescing
Enforce using the nullish coalescing operator instead of logical assignments or chaining


### https://tseslint.com/rules/prefer-optional-chain
Enforce using concise optional chain expressions instead of chained logical ands, negated logical ors, or empty objects


### https://tseslint.com/rules/prefer-promise-reject-errors
Require using Error objects as Promise rejection reasons


### https://tseslint.com/rules/prefer-readonly
Require private members to be marked as readonly if they're never modified outside of the constructor


### https://tseslint.com/rules/prefer-readonly-parameter-types
Require function parameters to be typed as readonly to prevent accidental mutation of inputs


### https://tseslint.com/rules/prefer-reduce-type-parameter
Enforce using type parameter when calling Array#reduce instead of using a type assertion


### https://tseslint.com/rules/prefer-regexp-exec
Enforce RegExp#exec over String#match if no global flag is provided


### https://tseslint.com/rules/prefer-return-this-type
Enforce that this is used when only this type is returned


### https://tseslint.com/rules/prefer-string-starts-ends-with
Enforce using String#startsWith and String#endsWith over other equivalent methods of checking substrings


### https://tseslint.com/rules/prefer-ts-expect-error
Enforce using @ts-expect-error over @ts-ignore


### https://tseslint.com/rules/promise-function-async
Require any function or method that returns a Promise to be marked async


### https://tseslint.com/rules/related-getter-setter-pairs
Enforce that get() types should be assignable to their equivalent set() type


### https://tseslint.com/rules/require-array-sort-compare
Require Array#sort and Array#toSorted calls to always provide a compareFunction


### https://tseslint.com/rules/require-await
Disallow async functions which do not return promises and have no await expression


### https://tseslint.com/rules/restrict-plus-operands
Require both operands of addition to be the same type and be bigint, number, or string


### https://tseslint.com/rules/restrict-template-expressions
Enforce template literal expressions to be of string type


### https://tseslint.com/rules/return-await
Enforce consistent awaiting of returned promises


### https://tseslint.com/rules/sort-type-constituents
Enforce constituents of a type union/intersection to be sorted alphabetically


### https://tseslint.com/rules/strict-boolean-expressions
Disallow certain types in boolean expressions


### https://tseslint.com/rules/switch-exhaustiveness-check
Require switch-case statements to be exhaustive


### https://tseslint.com/rules/triple-slash-reference
Disallow certain triple slash directives in favor of ES6-style import declarations


### https://tseslint.com/rules/typedef
Require type annotations in certain places


### https://tseslint.com/rules/unbound-method
Enforce unbound methods are called with their expected scope


### https://tseslint.com/rules/unified-signatures
Disallow two overloads that could be unified into one with a union or an optional/rest parameter


### https://tseslint.com/rules/use-unknown-in-catch-callback-variable
Enforce typing arguments in Promise rejection callbacks as unknown
