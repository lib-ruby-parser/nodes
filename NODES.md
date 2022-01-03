## Alias

    Represents `alias to from` statement.

Fields:

1. **to** (`Node`)

    Target of the `alias`.
   
    `Sym("foo")` node for `alias :foo :bar`

2. **from** (`Node`)

    Source of the `alias`.
   
    `Sym("bar")` node for `alias :foo :bar`

3. **keyword_l** (`Loc`)

    Location of the `alias` keyword
   
    ```text
    alias foo bar
    ~~~~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    alias foo bar
    ~~~~~~~~~~~~~
    ```


## And

    Represents `foo && bar` (or `foo and bar`) statement.

Fields:

1. **lhs** (`Node`)

    Left hand statament of the `&&` operation.
   
    `Lvar("foo")` node for `foo && bar`

2. **rhs** (`Node`)

    Right hand statement of the `&&` operation.
   
    `Lvar("bar")` node for `foo && bar`

3. **operator_l** (`Loc`)

    Location of the `&&` (or `and`) operator
   
    ```text
    a && b
      ~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    a && b
    ~~~~~~
    ```


## AndAsgn

    Represents `a &&= 1` statement.

Fields:

1. **recv** (`Node`)

    Receiver of the `&&=` operation.
   
    `Lvasgn("a")` node for `a &&= 1`

2. **value** (`Node`)

    Right hand statement of assignment
   
    `Int("1")` node for `a &&= 1`

3. **operator_l** (`Loc`)

    Location of the `&&=` operator
   
    ```text
    a &&= 1
      ~~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    a &&= 1
    ~~~~~~~
    ```


## Arg

    Represents a positional required block/method argument.
   
    `a` in `def m(a); end` or `proc { |a| }`

Fields:

1. **name** (`Str { raw: false }`)

    Name of the argument

2. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    def m(argument); end
          ~~~~~~~~
    ```


## Args

    Represents an arguments list
   
    `Args(vec![Arg("a"), Optarg("b", Int("1"))])` in `def m(a, b = 1); end`

Fields:

1. **args** (`Nodes`)

    List of arguments

2. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    def m(a, b = 1, c:, &blk); end
         ~~~~~~~~~~~~~~~~~~~~
    ```

3. **begin_l** (`MaybeLoc`)

    Location of the open parenthesis
   
    ```text
    def m(a, b = 1, c:, &blk); end
         ~
    ```
   
    `None` for code like `def m; end` or `def m arg; end`

4. **end_l** (`MaybeLoc`)

    Location of the closing parenthesis
   
    ```text
    def m(a, b = 1, c:, &blk); end
                            ~
    ```
   
    `None` for code like `def m; end` or `def m arg; end`


## Array

    Represents an array literal

Fields:

1. **elements** (`Nodes`)

    A list of elements

2. **begin_l** (`MaybeLoc`)

    Location of the open bracket
   
    ```text
    [1, 2, 3]
    ~
    ```

3. **end_l** (`MaybeLoc`)

    Location of the closing bracket
   
    ```text
    [1, 2, 3]
            ~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    [1, 2, 3]
    ~~~~~~~~~
    ```


## ArrayPattern

    Represents an array pattern used in pattern matching

Fields:

1. **elements** (`Nodes`)

    A list of elements

2. **begin_l** (`MaybeLoc`)

    Location of the open bracket
   
    ```text
    [1, ^a, 3 => foo]
    ~
    ```
   
    `None` for pattern like `1, 2` without brackets

3. **end_l** (`MaybeLoc`)

    Location of the closing bracket
   
    ```text
    [1, ^a, 3 => foo]
                    ~
    ```
   
    `None` for pattern like `1, 2` without brackets

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    [1, ^a, 3 => foo]
    ~~~~~~~~~~~~~~~~~
    ```


## ArrayPatternWithTail

    Represents an array pattern *with trailing comma* used in pattern matching
   
    It's slightly different from `ArrayPattern`, trailing comma at the end works as `, *`

Fields:

1. **elements** (`Nodes`)

    A list of elements

2. **begin_l** (`MaybeLoc`)

    Location of the open bracket
   
    ```text
    [1, ^a, 3 => foo,]
    ~
    ```
   
    `None` for pattern like `1, 2,` without brackets

3. **end_l** (`MaybeLoc`)

    Location of the closing bracket
   
    ```text
    [1, ^a, 3 => foo,]
                     ~
    ```
   
    `None` for pattern like `1, 2,` without brackets

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    [1, ^a, 3 => foo,]
    ~~~~~~~~~~~~~~~~~~
    ```


## BackRef

    Represents special global variables:
    1. `` $` ``
    2. `$&`
    3. `$'`
    4. `$+`

Fields:

1. **name** (`Str { raw: false }`)

    Name of the variable (`"$+"` for `$+`)

2. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    $+
    ~~
    ```


## Begin

    Represents compound statement (i.e. a multi-statement)
   
    Basically all blocks of code are wrapped into `Begin` node (e.g. method/block body, rescue/ensure handler etc)

Fields:

1. **statements** (`Nodes`)

    A list of statements

2. **begin_l** (`MaybeLoc`)

    Begin of the block
   
    ```text
    (1; 2)
    ~
    ```
   
    `None` if the block of code is "implicit", like
   
    ```text
    if true; 1; 2; end
    ```

3. **end_l** (`MaybeLoc`)

    End of the block
   
    ```text
    (1; 2)
         ~
    ```
   
    `None` if the block of code is "implicit", like
   
    ```text
    if true; 1; 2; end
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    (1; 2)
    ~~~~~~
    ```


## Block

    Represents a Ruby block that is passed to a method (`proc { |foo| bar }`)

Fields:

1. **call** (`Node`)

    Method call that takes a block
   
    `Send("foo")` in `foo {}`

2. **args** (`MaybeNode { regexp_options: false }`)

    A list of argument that block takes
   
    `vec![ Arg("a"), Optarg("b", Int("1")) ]` for `proc { |a, b = 1| }`
   
    `None` if the block takes no arguments

3. **body** (`MaybeNode { regexp_options: false }`)

    Block body, `None` if block has no body.

4. **begin_l** (`Loc`)

    Location of the open brace
   
    ```text
    proc { }
         ~
    ```

5. **end_l** (`Loc`)

    Location of the closing brace
   
    ```text
    proc { }
           ~
    ```

6. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    proc { }
    ~~~~~~~~
    ```


## Blockarg

    Represents a `&blk` argument in the method definition (but not in the method call, see `BlockPass`)

Fields:

1. **name** (`MaybeStr { chars: false }`)

    Name of the argument, `String("foo")` for `def m(&foo)`

2. **operator_l** (`Loc`)

    Location of the `&` operator
   
    ```text
    def m(&foo); end
          ~
    ```

3. **name_l** (`MaybeLoc`)

    Location of the name
   
    ```text
    def m(&foo); end
           ~~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    def m(&foo); end
          ~~~~
    ```


## BlockPass

    Represents a `&blk` argument of the method call (but not of the method definition, see `BlockArg`)

Fields:

1. **value** (`Node`)

    Value that is converted to a block
   
    `Int("1")` in `foo(&1)` (yes, it's possible)

2. **operator_l** (`Loc`)

    Location of the `&` operator
   
    ```text
    foo(&blk)
        ~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo(&bar)
        ~~~~
    ```


## Break

    Represents a `break` keyword (with optional argument)

Fields:

1. **args** (`Nodes`)

    A list of arguments

2. **keyword_l** (`Loc`)

    Location of the `break` keyword
   
    ```text
    break :foo
    ~~~~~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    break(:foo)
    ~~~~~~~~~~~
    ```


## Case

    Represents a `case` statement (for pattern matching see `CaseMatch` node)

Fields:

1. **expr** (`MaybeNode { regexp_options: false }`)

    Expression given to `case`, `Int("1")` for `case 1; end`
    `None` for code like
   
    ```text
    case
    when pattern
    end
    ```

2. **when_bodies** (`Nodes`)

    A list of `When` nodes (each has `patterns` and `body`)

3. **else_body** (`MaybeNode { regexp_options: false }`)

    Body of the `else` branch, `None` if there's no `else` branch

4. **keyword_l** (`Loc`)

    Location of the `case` keyword
   
    ```text
    case 1; end
    ~~~~
    ```

5. **else_l** (`MaybeLoc`)

    Location of the `else` keyword
   
    ```text
    case 1; else; end
            ~~~~
    ```
   
    `None` if there's no `else` branch

6. **end_l** (`Loc`)

    Location of the `end` keyword
   
    ```text
    case 1; end
            ~~~
    ```

7. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    case 1; end
    ~~~~~~~~~~~
    ```


## CaseMatch

    Represents a `case` statement used for pattern matching (for regular `case` see `Case` node)

Fields:

1. **expr** (`Node`)

    Expression given to `case`, `Int("1")` for `case 1; in 1; end`
    `None` for code like
   
    ```text
    case
    in pattern
    end
    ```

2. **in_bodies** (`Nodes`)

    A list of `InPattern` nodes (each has `pattern`, `guard` and `body`)

3. **else_body** (`MaybeNode { regexp_options: false }`)

    Body of the `else` branch, `None` if there's no `else` branch

4. **keyword_l** (`Loc`)

    Location of the `case` keyword
   
    ```text
    case 1; in 2; end
    ~~~~
    ```

5. **else_l** (`MaybeLoc`)

    Location of the `else` keyword
   
    ```text
    case 1; in 2; else; end
                  ~~~~
    ```
   
    `None` if there's no `else` branch

6. **end_l** (`Loc`)

    Location of the `end` keyword
   
    ```text
    case 1; in 2; end
                  ~~~
    ```

7. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    case 1; in 2; end
    ~~~~~~~~~~~~~~~~~
    ```


## Casgn

    Represents a constant assignment (i.e. `A = 1`)

Fields:

1. **scope** (`MaybeNode { regexp_options: false }`)

    Scope where the constant is defined:
    1. `Some(Const("A"))` for `A::B = 1`
    2. `None` if it's defined in the current scope (i.e. `A = 1`)
    3. `Some(Cbase)` if it's defined in the global scope (i.e. `::A = 1`)

2. **name** (`Str { raw: false }`)

    Name of the constant, `String("A")` for `A = 1`

3. **value** (`MaybeNode { regexp_options: false }`)

    Value that is assigned to a constant, `Int("1")` for `A = 1`.
   
    **Note**: `None` if constant assignment is a part of the multi-assignment.
    In such case `value` belongs to `Masgn` node of the multi-assignment.

4. **double_colon_l** (`MaybeLoc`)

    Location of the `::` operator
   
    ```text
    A::B = 1
     ~~
   
    ::A = 1
    ~~
    ```
   
    `None` if the constant is defined in the current scope

5. **name_l** (`Loc`)

    Location of the constant name
   
    ```text
    A::CONST = 1
       ~~~~~
    ```

6. **operator_l** (`MaybeLoc`)

    Location of the `=` operator
   
    ```text
    A = 1
      ~
    ```
   
    `None` if constant assignment is a part of the multi-assignment.
    In such case `=` belongs to a `Masgn` node

7. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    A = 1
    ~~~~~
    ```


## Cbase

    Represents leading `::` part of the constant access/assignment that is used to get/set on a global namespace.

Fields:

1. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    ::A
    ~~
    ```


## Class

    Represents a class definition (using a `class` keyword, `Class.new` is just a method call)

Fields:

1. **name** (`Node`)

    Name of the class, `String("Foo")` for `class Foo; end`

2. **superclass** (`MaybeNode { regexp_options: false }`)

    Superclass. Can be an expression in cases like `class A < (obj.foo + 1); end`
   
    `None` if no explicit superclass given (i.e. `class Foo; end`)

3. **body** (`MaybeNode { regexp_options: false }`)

    Body of the method, `None` if there's no body.

4. **keyword_l** (`Loc`)

    Location of the `class` keyword.
   
    ```text
    class Foo; end
    ~~~~~
    ```

5. **operator_l** (`MaybeLoc`)

    Location of the `<` operator
   
    ```text
    class A < B; end
            ~
    ```
   
    `None` if there's no explicit superclass given.

6. **end_l** (`Loc`)

    Location of the `end` keyword.
   
    ```text
    class Foo; end
               ~~~
    ```

7. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    class Foo; end
    ~~~~~~~~~~~~~~
    ```


## Complex

    Represents a `Complex` literal (that returns an `Complex` number)

Fields:

1. **value** (`Str { raw: false }`)

    Value of the complex literal, returned as a `String`, `String("1i")` for `1i`

2. **operator_l** (`MaybeLoc`)

    Location of the `-` (but not `+`) operator. `+` is a part of the literal:
    1. `+1i` is `String("+1i")` with `operator = None`
    2. `-1i` is `String("1i")` with `operator = String("-")`
   
    ```text
    -1i
    ~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    -1i
    ~~~
    ```


## Const

    Represents constant access (i.e. `Foo::Bar`)

Fields:

1. **scope** (`MaybeNode { regexp_options: false }`)

    Scope where the constant is taken from:
    1. `Some(Const("A"))` for `A::B`
    2. `None` if it's taken from the current scope (i.e. `A`)
    3. `Some(Cbase)` if it's taken from the global scope (i.e. `::A`)

2. **name** (`Str { raw: false }`)

    Name of the constant, `String("Foo")` for `Foo`

3. **double_colon_l** (`MaybeLoc`)

    Location of the `::` operator. `None` if constant is taken from the current scope.
   
    ```text
    A::B
     ~~
    ```

4. **name_l** (`Loc`)

    Location of the constant name
   
    ```text
    Foo::Bar
         ~~~
    ```

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    Foo::Bar
    ~~~~~~~~
    ```


## ConstPattern

    Const pattern used in pattern matching (e.g. `in A(1, 2)`)

Fields:

1. **const** (`Node`)

    Constant that is used, `Const("Foo")` for `in For(42)`

2. **pattern** (`Node`)

    Inner part of the constant pattern
   
    `ArrayPattern(vec![ Int("1") ])` for `Foo(1)`

3. **begin_l** (`Loc`)

    Location of the open parenthesis
   
    ```text
    case 1; in Foo(42); end
                  ~
    ```

4. **end_l** (`Loc`)

    Location of the closing parenthesis
   
    ```text
    case 1; in Foo(42); end
                     ~
    ```

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    case 1; in Foo(42); end
               ~~~~~~~
    ```


## CSend

    Represents conditional method call using `&.` operator

Fields:

1. **recv** (`Node`)

    Receiver of the method call, `Int("1")` for `1&.foo`

2. **method_name** (`Str { raw: false }`)

    Name of the method, `String("foo")` for `1&.foo`

3. **args** (`Nodes`)

    List of arguments
   
    ```text
    foo&.bar(42)
    # and also setters like
    foo&.bar = 42
    ```

4. **dot_l** (`Loc`)

    Location of the `&.` operator
   
    ```text
    foo&.bar
       ~~
    ```

5. **selector_l** (`MaybeLoc`)

    Location of the method name
   
    ```text
    foo&.bar(42)
         ~~~
    ```
   
    `None` in a very special case when method call is implicit (i.e. `foo&.()`)

6. **begin_l** (`MaybeLoc`)

    Location of the open parenthesis
   
    ```text
    foo&.bar(42)
            ~
    ```
   
    `None` if there are no parentheses

7. **end_l** (`MaybeLoc`)

    Location of the closing parenthesis
   
    ```text
    foo&.bar(42)
               ~
    ```
   
    `None` if there are no parentheses

8. **operator_l** (`MaybeLoc`)

    Location of the operator if `CSend` is a part of assignment like
   
    ```text
    foo&.bar = 1
             ~
    ```
   
    `None` for a regular call.

9. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo&.bar(42)
    ~~~~~~~~~~~~
    ```


## Cvar

    Represents access to class variable (i.e. `@@var`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the class variable, `String("@@foo")` for `@@foo`

2. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    @@foo
    ~~~~~
    ```


## Cvasgn

    Represents class variable assignment (i.e. `@@var = 42`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the class variable, `String("@@foo")` for `@@foo = 1`

2. **value** (`MaybeNode { regexp_options: false }`)

    Value that is assigned to class variable, `Int("1")` for `@@foo = 1`

3. **name_l** (`Loc`)

    Location of the class variable name
   
    ```text
    @@foo = 1
    ~~~~~
    ```

4. **operator_l** (`MaybeLoc`)

    Location of the `=` operator
   
    ```text
    @@foo = 1
          ~
    ```

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    @@foo = 1
    ~~~~~~~~~
    ```


## Def

    Represents method definition using `def` keyword (not on a singleton, see `Defs` node).

Fields:

1. **name** (`Str { raw: false }`)

    Name of the method, `String("foo")` for `def foo; end`

2. **args** (`MaybeNode { regexp_options: false }`)

    Arguments of a method, `None` if there's no arguments.
   
    All information about parentheses around arguments is stored in this node.

3. **body** (`MaybeNode { regexp_options: false }`)

    Body of a method, `None` if there's no body.

4. **keyword_l** (`Loc`)

    Location of the `def` keyword.
   
    ```text
    def foo; end
    ~~~
    ```

5. **name_l** (`Loc`)

    Location of the method name.
   
    ```text
    def foo; end
        ~~~
    ```

6. **end_l** (`MaybeLoc`)

    Location of the `end` keyword.
   
    ```text
    def foo; end
             ~~~
    ```
   
    `None` for endless method definition

7. **assignment_l** (`MaybeLoc`)

    Location of the `=` operator for endless method definition
   
    ```text
    def m() = 1
            ~
    ```
   
    `None` for regular method definition

8. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    def m(a); foo; end
    ~~~~~~~~~~~~~~~~~~
    ```


## Defined

    Represents a `defined?(foo)` expression

Fields:

1. **value** (`Node`)

    Value given to `defined?`

2. **keyword_l** (`Loc`)

    Location of the `defined?` keyword
   
    ```text
    defined?(foo)
    ~~~~~~~~
    ```

3. **begin_l** (`MaybeLoc`)

    Location of the open parenthesis
   
    ```text
    defined?(foo)
            ~
    ```
   
    `None` if there are no parentheses

4. **end_l** (`MaybeLoc`)

    Location of the closing parenthesis
   
    ```text
    defined?(foo)
                ~
    ```
   
    `None` if there are no parentheses

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    defined?(foo)
    ~~~~~~~~~~~~~
    ```


## Defs

    Represents a singleton method definition (i.e. `def self.foo; end`)

Fields:

1. **definee** (`Node`)

    Definee of a method definition, `Lvar("x")` for `def x.foo; end`

2. **name** (`Str { raw: false }`)

    Name of the method, `String("foo")` for `def x.foo; end`

3. **args** (`MaybeNode { regexp_options: false }`)

    Arguments of a method, `None` if there's no arguments.
   
    All information about parentheses around arguments is stored in this node.

4. **body** (`MaybeNode { regexp_options: false }`)

    Body of the method, `None` if there's no body.

5. **keyword_l** (`Loc`)

    Location of the `def` keyword
   
    ```text
    def self.foo; end
    ~~~
    ```

6. **operator_l** (`Loc`)

    Location of the `.`
   
    ```text
    def self.foo; end
            ~
    ```

7. **name_l** (`Loc`)

    Location of the method name
   
    ```text
    def self.foo; end
             ~~~
    ```

8. **assignment_l** (`MaybeLoc`)

    Location of the `=` operator for endless method definition
   
    ```text
    def self.foo() = 42
                   ~
    ```
   
    `None` for regular method definition

9. **end_l** (`MaybeLoc`)

    Location of the `end` keyword
   
    ```text
    def self.foo; end
                  ~~~
    ```
   
    `None` for endless method definition

10. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    def self.foo; end
    ~~~~~~~~~~~~~~~~~
    ```


## Dstr

    Represents a string with interpolation (i.e. `"#{foo}"`)

Fields:

1. **parts** (`Nodes`)

    A list of string parts (static literals and interpolated expressions)

2. **begin_l** (`MaybeLoc`)

    Location of the string begin
   
    ```text
    "#{foo}"
    ~
   
    %Q{#{foo}}
    ~~~
    ```

3. **end_l** (`MaybeLoc`)

    Location of the string end
   
    ```text
    "#{foo}"
           ~
   
    %Q{#{foo}}
             ~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    "#{foo}"
    ~~~~~~~~
   
    %Q{#{foo}}
    ~~~~~~~~~~
    ```


## Dsym

    Represents a symbol with interpolation (i.e. `:"#{foo}"`)

Fields:

1. **parts** (`Nodes`)

    A list of symbol parts (static literals and interpolated expressions)

2. **begin_l** (`MaybeLoc`)

    Location of the symbol begin
   
    ```text
    :"#{foo}"
    ~~
    ```
   
    `None` if `Dsym` is a part of the interpolated symbol array:
   
    ```text
    %I[#{bar}]
    ```

3. **end_l** (`MaybeLoc`)

    Location of the symbol begin
   
    ```text
    :"#{foo}"
            ~
    ```
   
    `None` if `Dsym` is a part of the interpolated symbol array:
   
    ```text
    %I[#{bar}]
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    :"#{foo}"
    ~~~~~~~~~
    ```


## EFlipFlop

    Represents exclusive flip-flop (i.e. in `if foo...bar; end`)

Fields:

1. **left** (`MaybeNode { regexp_options: false }`)

    Left part of the flip-flop. `None` if based on a range without begin (`...bar`)

2. **right** (`MaybeNode { regexp_options: false }`)

    Right part of the flip-flop. `None` if based on a range without end (`foo...`)

3. **operator_l** (`Loc`)

    Location of the `...` operator
   
    ```text
    if foo...bar; end
          ~~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    if foo...bar; end
       ~~~~~~~~~
    ```


## EmptyElse

    Represents a special empty else that is a part of the pattern matching.
   
    Usually empty else (e.g. part of the `if` statement) doesn't mean anything,
    however in pattern matching it prevents raising a `NoPatternError`.
   
    Throwing away this `else` may affect your code.

Fields:

1. **expression_l** (`Loc`)

    Location of the `else` keyword
   
    ```text
    case foo; in 1; else; end
                    ~~~~
    ```


## Encoding

    Represents a special `__ENCODING__` keyword

Fields:

1. **expression_l** (`Loc`)

    Location of the `__ENCODING__` keyword
   
    ```text
    __ENCODING__
    ~~~~~~~~~~~~
    ```


## Ensure

    Represents a block of code with `ensure` (i.e. `begin; ensure; end`)

Fields:

1. **body** (`MaybeNode { regexp_options: false }`)

    Block of code that is wrapped into `ensure`
    **Note**: that's the body of the `ensure` block
   
    `Int("1")` for `begin; 1; ensure; 2; end`

2. **ensure** (`MaybeNode { regexp_options: false }`)

    Body of the `ensure` block
   
    `Int("2")` for `begin; 1; ensure; 2; end`

3. **keyword_l** (`Loc`)

    Location of the `ensure` keyword
   
    ```text
    begin; ensure; end
           ~~~~~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    begin; 1; rescue; 2; else; 3; ensure; 4; end
           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ```
   
    **Note**: begin/end belong to `KwBegin` node.


## Erange

    Represents range literal with excluded `end` (i.e. `1...3`)

Fields:

1. **left** (`MaybeNode { regexp_options: false }`)

    Begin of the range, `None` if range has no begin (i.e `...42`)

2. **right** (`MaybeNode { regexp_options: false }`)

    End of the range, `None` if range has no end (i.e `42...`)

3. **operator_l** (`Loc`)

    Location of the `...` operator
   
    ```text
    1...3
     ~~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    1...3
    ~~~~~
    ```


## False

    Represents a `false` literal

Fields:

1. **expression_l** (`Loc`)

    Location of the `false` literal
   
    ```text
    false
    ~~~~~
    ```


## File

    Represents a special `__FILE__` literal

Fields:

1. **expression_l** (`Loc`)

    Location of the `__FILE__` literal
   
    ```text
    __FILE__
    ~~~~~~~~
    ```


## FindPattern

    Represents a find pattern using in pattern matching (i.e. `in [*x, 1 => a, *y]`)
   
    It's different from `ArrayPattern`/`ConstPattern` because it supports multiple wildcard pattern

Fields:

1. **elements** (`Nodes`)

    Inner part of the find pattern

2. **begin_l** (`MaybeLoc`)

    Location of the begin
   
    ```text
    case foo; in [*x, 1 => a, *y]; end
                 ~
    ```
   
    `None` if there are no brackets/parentheses

3. **end_l** (`MaybeLoc`)

    Location of the end
   
    ```text
    case foo; in [*x, 1 => a, *y]; end
                                ~
    ```
   
    `None` if there are no brackets/parentheses

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    case foo; in [*x, 1 => a, *y]; end
                 ~~~~~~~~~~~~~~~~
    ```


## Float

    Represents a float literal (i.e. `42.5`)

Fields:

1. **value** (`Str { raw: false }`)

    String value of the literal, `String("42.5")` for `42.5`

2. **operator_l** (`MaybeLoc`)

    Location of unary `-` (but not `+`)
   
    ```text
    -42.5
    ~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    -42.5
    ~~~~~
    ```


## For

    Represents a `for` loop

Fields:

1. **iterator** (`Node`)

    Variable that is used in loop, `Lvasgn("a")` in `for a in b; end`

2. **iteratee** (`Node`)

    Collection that is for iteration. `Lvar("b")` in `for a in b; end`

3. **body** (`MaybeNode { regexp_options: false }`)

    Body of the loop. `None` if there's no body

4. **keyword_l** (`Loc`)

    Location of the `for` keyword
   
    ```text
    for a in b; end
    ~~~
    ```

5. **operator_l** (`Loc`)

    Location of the `in` keyword
   
    ```text
    for a in b; end
          ~~
    ```

6. **begin_l** (`Loc`)

    Location of the `do` keyword
   
    ```text
    for a in b do; end
               ~~
    ```
   
    **Note**: this `do` is optional, and so `begin_l` can be `None`.

7. **end_l** (`Loc`)

    Location of the `end` keyword
   
    ```text
    for a in b; end
                ~~~
    ```

8. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    for a in b; end
    ~~~~~~~~~~~~~~~
    ```


## ForwardArg

    Represents a special `...` argument that forwards positional/keyword/block arguments.

Fields:

1. **expression_l** (`Loc`)

    Location of the `...`
   
    ```text
    def m(...); end
          ~~~
    ```


## ForwardedArgs

    Represents a `...` operator that contains forwarded argument (see `ForwardArg`)

Fields:

1. **expression_l** (`Loc`)

    Location of the `...`
   
    ```text
    def m(...); foo(...); end
                    ~~~
    ```


## Gvar

    Represents access to global variable (i.e. `$foo`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the global variable, `String("$foo")` for `$foo`

2. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    $foo
    ~~~~
    ```


## Gvasgn

    Represents global variable assignment (i.e. `$foo = 42`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the global variable, `String("$foo")` for `$foo`

2. **value** (`MaybeNode { regexp_options: false }`)

    Value that is assigned to global variable, `Int("42")` for `$foo = 42`
   
    `None` if global variable assignment is a part of the multi-assignment.
    In such case `value` is a part of the `Masgn` node.

3. **name_l** (`Loc`)

    Location of the global variable name
   
    ```text
    $foo = 42
    ~~~~
    ```

4. **operator_l** (`MaybeLoc`)

    Location of the `=` operator
   
    ```text
    $foo = 42
         ~
    ```
   
    `None` if global variable assignment is a part of the multi-assignment.
    In such case `=` operator belongs to the `Masgn` node.

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    $foo = 42
    ~~~~~~~~~
    ```


## Hash

    Represents a hash literal (i.e. `{ foo: 42 }`)

Fields:

1. **pairs** (`Nodes`)

    A list of key-value pairs

2. **begin_l** (`MaybeLoc`)

    Location of the open parenthesis
   
    ```text
    { a: 1 }
    ~
    ```
   
    `None` if hash literal is implicit, e.g. `foo(key: "value")`

3. **end_l** (`MaybeLoc`)

    Location of the closing parenthesis
   
    ```text
    { a: 1 }
           ~
    ```
   
    `None` if hash literal is implicit, e.g. `foo(key: "value")`

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    { a: 1 }
    ~~~~~~~~
    ```


## HashPattern

    Represents a hash pattern used in pattern matching (i.e. `in { a: 1 }`)

Fields:

1. **elements** (`Nodes`)

    A list of inner patterns

2. **begin_l** (`MaybeLoc`)

    Location of the open parenthesis
   
    ```text
    case foo; in { a: 1 }; end
                 ~
    ```
   
    `None` if there are no parentheses

3. **end_l** (`MaybeLoc`)

    Location of the open parenthesis
   
    ```text
    case foo; in { a: 1 }; end
                        ~
    ```
   
    `None` if there are no parentheses

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    case foo; in { a: 1 }; end
                 ~~~~~~~~
    ```


## Heredoc

    Represents a here-document literal (both with and without interpolation)
   
    It's similar to `Dstr` in terms of abstract syntax tree, but has different source maps.

Fields:

1. **parts** (`Nodes`)

    A list of string parts (static literals and interpolated expressions)

2. **heredoc_body_l** (`Loc`)

    Location of the here-document body
   
    ```text
    <<-HERE\n  a\n   #{42}\nHERE
    ~~~~~~~~~~~~~~~
    ```

3. **heredoc_end_l** (`Loc`)

    Location of the here-document end
   
    ```text
    <<-HERE\n  a\n   #{42}\nHERE
                            ~~~~
    ```

4. **expression_l** (`Loc`)

    Location of the here-document identifier
   
    ```text
    <<-HERE\n  a\n   #{42}\nHERE
    ~~~~~~~
    ```
   
    **Note**: This is the only node (with `XHeredoc`) that has `expression_l` smaller that all other sub-locations merged.
    The reason for that is that it's possible to add more code after here-document ID:
   
    ```text
    <<-HERE + "rest"
    content
    HERE
    ```


## If

    Represents an `if` statement (i.e. `if foo; bar; else; baz; end`)

Fields:

1. **cond** (`Node`)

    Condition given to the `if` statement, `Lvar("a")` for `if a; b; else; c; end`

2. **if_true** (`MaybeNode { regexp_options: false }`)

    True-branch of the `if` statement, `Lvar("b")` for `if a; b; else; c; end`

3. **if_false** (`MaybeNode { regexp_options: false }`)

    False-branch of the `if` statement, `Lvar("c")` for `if a; b; else; c; end`

4. **keyword_l** (`Loc`)

    Location of the `if` keyword
   
    ```text
    if foo; end
    ~~
    ```

5. **begin_l** (`Loc`)

    Location of the `then` keyword
   
    ```text
    if foo then; end
           ~~~~
    ```
   
    `None` if `then` keyword is omitted

6. **else_l** (`MaybeLoc`)

    Location of the `else` keyword
   
    ```text
    if foo; else; end
            ~~~~
    ```
   
    `None` if there's no `else` branch

7. **end_l** (`MaybeLoc`)

    Location of the `end` keyword
   
    ```text
    if foo; end
            ~~~
    ```

8. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    if a then; b; else; c end
    ~~~~~~~~~~~~~~~~~~~~~~~~~
    ```


## IfGuard

    Represents an `if` guard used in pattern matching (i.e. `case foo; in pattern if guard; end`)

Fields:

1. **cond** (`Node`)

    Condition of the guard, `Lvar("foo")` in `in pattern if guard`

2. **keyword_l** (`Loc`)

    Location of the `if` keyword
   
    ```text
    case foo; in pattern if cond; end
                         ~~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    case foo; in pattern if cond; end
                         ~~~~~~~
    ```


## IFlipFlop

    Represents inclusive flip-flop (i.e. in `if foo..bar; end`)

Fields:

1. **left** (`MaybeNode { regexp_options: false }`)

    Left part of the flip-flop. `None` if based on a range without begin (`..bar`)

2. **right** (`MaybeNode { regexp_options: false }`)

    Right part of the flip-flop. `None` if based on a range without end (`foo..`)

3. **operator_l** (`Loc`)

    Location of the `..` operator
   
    ```text
    if foo..bar; end
          ~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    if foo..bar; end
       ~~~~~~~~
    ```


## IfMod

    Represents an `if`/`unless` modifier (i.e. `stmt if cond`)

Fields:

1. **cond** (`Node`)

    Condition of the modifier

2. **if_true** (`MaybeNode { regexp_options: false }`)

    True-branch of the modifier.
   
    Always set for `if` modifier.
    Always `None` for `unless` modifier.

3. **if_false** (`MaybeNode { regexp_options: false }`)

    False-branch of the modifier.
   
    Always set for `unless` modifier.
    Always `None` for `if` modifier.

4. **keyword_l** (`Loc`)

    Location of the `if`/`unless` keyword
   
    ```text
    stmt if cond
         ~~
   
    stmt unless cond
         ~~~~~~
    ```

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    stmt if cond
    ~~~~~~~~~~~~
   
    stmt unless cond
    ~~~~~~~~~~~~~~~~
    ```


## IfTernary

    Represents ternary `if` statement (i.e. `cond ? if_true : if_false`)

Fields:

1. **cond** (`Node`)

    Condition of the `if` statement

2. **if_true** (`Node`)

    True-branch

3. **if_false** (`Node`)

    True-branch

4. **question_l** (`Loc`)

    Location of the `?` operator
   
    ```text
    cond ? if_true : if_false
         ~
    ```

5. **colon_l** (`Loc`)

    Location of the `:` operator
   
    ```text
    cond ? if_true : if_false
                   ~
    ```

6. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    cond ? if_true : if_false
    ~~~~~~~~~~~~~~~~~~~~~~~~~
    ```


## Index

    Represents indexing operation (i.e. `foo[1,2,3]`)

Fields:

1. **recv** (`Node`)

    Receiver of indexing

2. **indexes** (`Nodes`)

    A list of indexes

3. **begin_l** (`Loc`)

    Location of open bracket
   
    ```text
    foo[1, 2, 3]
       ~
    ```

4. **end_l** (`Loc`)

    Location of closing bracket
   
    ```text
    foo[1, 2, 3]
               ~
    ```

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo[1, 2, 3]
    ~~~~~~~~~~~~
    ```


## IndexAsgn

    Represents assignment using indexing operation (i.e. `foo[1, 2, 3] = bar`)

Fields:

1. **recv** (`Node`)

    Receiver of the indexing

2. **indexes** (`Nodes`)

    A list of indexes

3. **value** (`MaybeNode { regexp_options: false }`)

    Value that is assigned
   
    `None` if assignment is a part of the multi-assignment.
    In such case `value` belongs to `Masgn` node.

4. **begin_l** (`Loc`)

    Location of open bracket
   
    ```text
    foo[1, 2, 3] = bar
       ~
    ```

5. **end_l** (`Loc`)

    Location of closing bracket
   
    ```text
    foo[1, 2, 3] = bar
               ~
    ```

6. **operator_l** (`MaybeLoc`)

    Location of the `=` operator
   
    ```text
    foo[1, 2, 3] = bar
                 ~
    ```
   
    `None` if assignment is a part of the multi-assignment.
    In such case operator `=` belongs to `Masgn` node.

7. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo[1, 2, 3] = bar
    ~~~~~~~~~~~~~~~~~~
    ```


## InPattern

    Represents an `in pattern` branch of the pattern matching

Fields:

1. **pattern** (`Node`)

    Value that is used for matching

2. **guard** (`MaybeNode { regexp_options: false }`)

    Guard that is used for matching
   
    Optional, so can be `None`

3. **body** (`MaybeNode { regexp_options: false }`)

    Body of the branch that is invoked if value matches pattern

4. **keyword_l** (`Loc`)

    Location of the `in` keyword
   
    ```text
    case value; in pattern; end
                ~~
    ```

5. **begin_l** (`Loc`)

    Location of the `then` keyword
   
    ```text
    case value; in pattern then; end
                           ~~~~
    ```

6. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    case value; in pattern then; 42; end
                ~~~~~~~~~~~~~~~~~~~
    ```


## Int

    Represents an integer literal (i.e. `42`)

Fields:

1. **value** (`Str { raw: false }`)

    String value of the literal, `String("42")` for `42`

2. **operator_l** (`MaybeLoc`)

    Location of unary `-` (but not `+`)
   
    ```text
    -42
    ~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    -42
    ~~~
    ```


## Irange

    Represents inclusive range (i.e. `2..4`)

Fields:

1. **left** (`MaybeNode { regexp_options: false }`)

    Begin of the range, `None` if range has no `begin` (i.e. `..4`)

2. **right** (`MaybeNode { regexp_options: false }`)

    End of the range, `None` if range has no `end` (i.e. `2..`)

3. **operator_l** (`Loc`)

    Location of the `..` operator
   
    ```text
    2..4
     ~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    2..4
    ~~~~
    ```


## Ivar

    Represents access to instance variable (i.e. `@foo`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the instance variable, `String("@foo")` in `@foo`

2. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    @foo
    ~~~~
    ```


## Ivasgn

    Represents instance variable assignment (i.e `@foo = 42`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the instance variable, `String("@foo")` in `@foo = 42`

2. **value** (`MaybeNode { regexp_options: false }`)

    Value that is assigned to instance variable.
   
    `None` if instance variable assignment is a part of the multi-assignment.
    In such case `value` is a part of the `Masgn` node.

3. **name_l** (`Loc`)

    Location of the instance variable name.
   
    ```text
    @foo = 1
    ~~~~
    ```

4. **operator_l** (`MaybeLoc`)

    Location of the `=` operator.
   
    ```text
    @foo = 1
         ~
    ```
   
    `None` if instance variable assignment is a part of the multi-assignment.
    In such case `value` is a part of the `Masgn` node.

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    @foo = 42
    ~~~~~~~~~
    ```


## Kwarg

    Represents required keyword argument (i.e. `foo` in `def m(foo:); end`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the keyword argument

2. **name_l** (`Loc`)

    Location of the name
   
    ```text
    def foo(bar:); end
            ~~~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    def foo(bar:); end
            ~~~~
    ```


## Kwargs

    Represents kwargs that are given to a method call, super or yield (i.e. `foo(bar: 1)`)

Fields:

1. **pairs** (`Nodes`)

    A list of key-value pairs

2. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo(bar: 1)
        ~~~~~~
    ```


## KwBegin

    Represents an explicit `begin; end` block.
   
    The reason why it's different is that
    ```text
    begin; foo; end while cond
    ```
    is a post-while loop (same with post-until loop)

Fields:

1. **statements** (`Nodes`)

    A list of statements

2. **begin_l** (`MaybeLoc`)

    Location of the `begin` keyword
   
    ```text
    begin; foo; end
    ~~~~~
    ```

3. **end_l** (`MaybeLoc`)

    Location of the `end` keyword
   
    ```text
    begin; foo; end
                ~~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    begin; foo; bar
    ~~~~~~~~~~~~~~~
    ```


## Kwnilarg

    Represents an special argument that rejects all keyword arguments (i.e. `def m(**nil); end`)

Fields:

1. **name_l** (`Loc`)

    Location of the `nil`
   
    ```text
    def m(**nil); end
            ~~~
    ```

2. **expression_l** (`Loc`)

    Location of the `nil`
   
    ```text
    def m(**nil); end
          ~~~~~
    ```


## Kwoptarg

    Represents an optional keyword argument (i.e. `foo` in `def m(foo: 42); end`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the optional keyword argument

2. **default** (`Node`)

    Default value of the optional keyword argument

3. **name_l** (`Loc`)

    Location of the argument name
   
    ```text
    def m(foo: 1); end
          ~~~
    ```

4. **expression_l** (`Loc`)

    Location of the argument name
   
    ```text
    def m(foo: 1); end
          ~~~~~~
    ```


## Kwrestarg

    Represents a keyword rest argument (i.e. `foo` in `def m(**foo); end`)

Fields:

1. **name** (`MaybeStr { chars: false }`)

    Name of the keyword rest argument, `String("foo")` in `def m(**foo); end`.
   
    `None` if argument has no name (`def m(**); end`)

2. **operator_l** (`Loc`)

    Location of the `**` operator
   
    ```text
    def m(**foo); end
          ~~
    ```

3. **name_l** (`MaybeLoc`)

    Location of the argument name
   
    ```text
    def m(**foo); end
            ~~~
    ```
   
    `None` if argument has no name (`def m(**); end`)

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    def m(**foo); end
          ~~~~~
    ```


## Kwsplat

    Represents a keyword arguments splat (i.e. `**bar` in a call like `foo(**bar)`)

Fields:

1. **value** (`Node`)

    Value that is converted into a `Hash` using `**`

2. **operator_l** (`Loc`)

    Location of the `**` operator
   
    ```text
    foo(**bar)
        ~~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo(**bar)
        ~~~~~
    ```


## Lambda

    Represents a lambda call using `->` (i.e. `-> {}`)
   
    Note that `Lambda` is a part of the `Block`, not other way around.

Fields:

1. **expression_l** (`Loc`)

    Location of the `->`
   
    ```text
    -> {}
    ~~
    ```


## Line

    Represents a special `__LINE__` literal

Fields:

1. **expression_l** (`Loc`)

    Location of the `__LINE__` literal
   
    ```text
    __LINE__
    ~~~~~~~~
    ```


## Lvar

    Represents access to a local variable (i.e. `foo`)
   
    Parser knows that it's a local variable because:
    1. there was an assignment to this variable **before** accessing it
    2. it's an argument of the current method / block
    3. it's been implicitly declared by `MatchWithLvasgn` node
   
    Otherwise it's a method call (see `Send`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the local variable

2. **expression_l** (`Loc`)

    Location of the local variable
   
    ```text
    foo
    ~~~
    ```


## Lvasgn

    Represents local variable assignment (i.e. `foo = 42`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the local variable

2. **value** (`MaybeNode { regexp_options: false }`)

    Value that is assigned to a local variable

3. **name_l** (`Loc`)

    Location of the local variable name
   
    ```text
    foo = 42
    ~~~
    ```

4. **operator_l** (`MaybeLoc`)

    Location of the `=` operator
   
    ```text
    foo = 42
        ~
    ```
   
    `None` if local variable assignment is a part of the multi-assignment.
    In such case `value` is a part of the `Masgn` node.

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo = 42
    ~~~~~~~~
    ```


## Masgn

    Represents mass-assignment (i.e. `foo, bar = 1, 2`)

Fields:

1. **lhs** (`Node`)

    Left hand statement of the assignment

2. **rhs** (`Node`)

    Left hand statement of the assignment

3. **operator_l** (`Loc`)

    Location of the `=` operator
   
    ```text
    foo, bar = 1, 2
             ~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo, bar = 1, 2
    ~~~~~~~~~~~~~~~
    ```


## MatchAlt

    Represents pattern matching using one of the given patterns (i.e. `foo in 1 | 2`)

Fields:

1. **lhs** (`Node`)

    Left pattern

2. **rhs** (`Node`)

    Right pattern

3. **operator_l** (`Loc`)

    Location of the `|` operator
   
    ```text
    foo in 1 | 2
             ~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo in 1 | 2
           ~~~~~
    ```


## MatchAs

    Represents matching with renaming into specified local variable (i.e. `case 1; in Integer => a; end`)

Fields:

1. **value** (`Node`)

    Pattern that is used for matching

2. **as** (`Node`)

    Variable that is assigned if matched (see `MatchVar` node)

3. **operator_l** (`Loc`)

    Location of the `=>` operator
   
    ```text
    case 1; in Integer => a; end
                       ~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    case 1; in Integer => a; end
               ~~~~~~~~~~~~
    ```


## MatchCurrentLine

    Represents implicit matching using `if /regex/`
   
    ```text
    if /.*/
    puts 'true'
    else
    puts 'false'
    end
    ```
    Prints "false".
   
    Under the hood this construction matches regex against `$_`, so the following works:
    ```text
    $_ = 'match_me'
    if /match_me/
    puts 'true'
    else
    puts 'false'
    end
    ```
    this code prints "true".

Fields:

1. **re** (`Node`)

    Given regex

2. **expression_l** (`Loc`)

    Location of the regex
   
    ```text
    if /re/; end
       ~~~~
    ```
   
    Technically this location is redundant, but keeping it is the only way to
    have the same interface for all nodes.


## MatchNilPattern

    Represents empty hash pattern that is used in pattern matching (i.e. `in **nil`)

Fields:

1. **operator_l** (`Loc`)

    Location of the `**` operator
   
    ```text
    in **nil
       ~~
    ```

2. **name_l** (`Loc`)

    Location of the name
   
    ```text
    in **nil
         ~~~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    in **nil
       ~~~~~
    ```


## MatchPattern

    Represents a one-line pattern matching that can throw an error (i.e. `foo => pattern`)

Fields:

1. **value** (`Node`)

    Value that is used for matching

2. **pattern** (`Node`)

    Pattern that is used for matching

3. **operator_l** (`Loc`)

    Location of the `=>` operator
   
    ```text
    foo => pattern
        ~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo => pattern
    ~~~~~~~~~~~~~~
    ```


## MatchPatternP

    Represents a one-line pattern matching that never throws but returns true/false (i.e. `foo in pattern`)

Fields:

1. **value** (`Node`)

    Value that is used for matching

2. **pattern** (`Node`)

    Pattern that is used for matching

3. **operator_l** (`Loc`)

    Location of the `in` operator
   
    ```text
    foo in pattern
        ~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo in pattern
    ~~~~~~~~~~~~~~
    ```


## MatchRest

    Represents a wildcard pattern used in pattern matching (i.e. `in *foo`)

Fields:

1. **name** (`MaybeNode { regexp_options: false }`)

    Name of the variable name
   
    `None` if there's no name (i.e. `in *`)

2. **operator_l** (`Loc`)

    Location of the `*` operator
   
    ```text
    case foo; in *bar; end
                 ~
    ```

3. **expression_l** (`Loc`)

    Location of the `*` operator
   
    ```text
    case foo; in *bar; end
                 ~~~~
    ```


## MatchVar

    Represents matching with assignment into a local variable (i.e. `pattern => var`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the variable that is assigned if matching succeeds

2. **name_l** (`Loc`)

    Location of the name
   
    ```text
    case foo; in pattern => bar; end
                            ~~~
    ```
   
    **Note** it can also be produced by a hash pattern
   
    ```text
    case foo; in { a: }; end
                   ~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    case foo; in pattern => bar; end
                            ~~~
    ```
   
    **Note** it can also be produced by a hash pattern
   
    ```text
    case foo; in { a: }; end
                   ~~
    ```


## MatchWithLvasgn

    Represents matching a regex that produces local variables (i.e. `/(?<match>bar)/ =~ 'bar'`)
   
    Each named group in regex declares a local variable.

Fields:

1. **re** (`Node`)

    Regex that is used for matching

2. **value** (`Node`)

    Value that is used for matching

3. **operator_l** (`Loc`)

    Location of the `=~` operatir
   
    ```text
    /(?<match>bar)/ =~ 'bar'
                    ~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    /(?<match>bar)/ =~ 'bar'
    ~~~~~~~~~~~~~~~~~~~~~~~~
    ```


## Mlhs

    Represents left hand statement of the mass-assignment (i.e. `foo, bar` in `foo, bar = 1, 2`)

Fields:

1. **items** (`Nodes`)

    A list of items that are assigned

2. **begin_l** (`MaybeLoc`)

    Location of the open parenthesis
   
    ```text
    (a, b) = 1, 2
    ~
    ```
   
    `None` if there are no parentheses

3. **end_l** (`MaybeLoc`)

    Location of the closing parenthesis
   
    ```text
    (a, b) = 1, 2
         ~
    ```
   
    `None` if there are no parentheses

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    (a, b) = 1, 2
    ~~~~~~
    ```


## Module

    Represents module declaration using `module` keyword

Fields:

1. **name** (`Node`)

    Name of the module

2. **body** (`MaybeNode { regexp_options: false }`)

    Body of the module
   
    `None` if module has no body

3. **keyword_l** (`Loc`)

    Location of the `module` keyword
   
    ```text
    module M; end
    ~~~~~~
    ```

4. **end_l** (`Loc`)

    Location of the `end` keyword
   
    ```text
    module M; end
              ~~~
    ```

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    module M; end
    ~~~~~~~~~~~~~
    ```


## Next

    Represents `next` keyword

Fields:

1. **args** (`Nodes`)

    Arguments given to `next`

2. **keyword_l** (`Loc`)

    Location of the `next` keyword
   
    ```text
    next 42
    ~~~~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    next(42)
    ~~~~~~~~
    ```


## Nil

    Represents `nil` literal

Fields:

1. **expression_l** (`Loc`)

    Location of the `nil` keyword
   
    ```text
    nil
    ~~~
    ```


## NthRef

    Represents numeric global variable (e.g. `$1`)

Fields:

1. **name** (`Str { raw: true }`)

    Name of the variable, `String("1")` for `$1`

2. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    $1
    ~~
    ```


## Numblock

    Represents a block that takes numbered parameters (i.e. `proc { _1 }`)

Fields:

1. **call** (`Node`)

    Method call that takes a block

2. **numargs** (`U8`)

    Number of parameters that block takes

3. **body** (`Node`)

    Block body

4. **begin_l** (`Loc`)

    Location of the open brace
   
    ```text
    proc { _1 }
         ~
    ```

5. **end_l** (`Loc`)

    Location of the closing brace
   
    ```text
    proc { _1 }
              ~
    ```

6. **expression_l** (`Loc`)

    Location of the open brace
   
    ```text
    proc { _1 }
    ~~~~~~~~~~~
    ```


## OpAsgn

    Represents an operation with assignment (e.g. `a += 1`)

Fields:

1. **recv** (`Node`)

    Left hand statement of the assignment

2. **operator** (`Str { raw: false }`)

    Operator, can be one of:
    1. `+=`
    2. `-=`
    3. `*=`
    4. `/=`
    5. `|=`
    6. `&=`
    7. `>>=`
    8. `<<=`
    9. `%=`
    10. `^=`
    11. `**=`

3. **value** (`Node`)

    Right hand statement of the assignment

4. **operator_l** (`Loc`)

    Location of the operator
   
    ```text
    a.b <<= c
        ~~~
    ```

5. **expression_l** (`Loc`)

    Location of the operator
   
    ```text
    a.b <<= c
    ~~~~~~~~~
    ```


## Optarg

    Represents optional positional argument (i.e. `foo` in `m(foo = 1)`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the argument

2. **default** (`Node`)

    Default value of the argument

3. **name_l** (`Loc`)

    Location of the argument name
   
    ```text
    def m(foo = 1); end
          ~~~
    ```

4. **operator_l** (`Loc`)

    Location of the `=` operator
   
    ```text
    def m(foo = 1); end
              ~
    ```

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    def m(foo = 1); end
          ~~~~~~~
    ```


## Or

    Represents `foo || bar` (or `foo or bar`) statement.

Fields:

1. **lhs** (`Node`)

    Left hand statement

2. **rhs** (`Node`)

    Right hand statement

3. **operator_l** (`Loc`)

    Location of the `||`/`or` operator
   
    ```text
    foo || bar
        ~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo || bar
    ~~~~~~~~~~
    ```


## OrAsgn

    Represents `lhs ||= rhs` assignment

Fields:

1. **recv** (`Node`)

    Left hand statement

2. **value** (`Node`)

    Right hand statement

3. **operator_l** (`Loc`)

    Location of the `||=` operator
   
    ```text
    foo ||= bar
        ~~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo ||= bar
    ~~~~~~~~~~~
    ```


## Pair

    Represents a key/value pair (e.g. a part of the `Hash` node)

Fields:

1. **key** (`Node`)

    Key of the pair

2. **value** (`Node`)

    Value of the pair

3. **operator_l** (`Loc`)

    Location of the `:` or `=>` operator
   
    ```text
    { foo: bar }
         ~
   
    { :foo => bar }
           ~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    { foo: bar }
      ~~~~~~~~
   
    { :foo => bar }
      ~~~~~~~~~~~
    ```


## Pin

    Represents a pattern based on a "pinned" variable (e.g. `^foo`)

Fields:

1. **var** (`Node`)

    Variable that is pinned

2. **selector_l** (`Loc`)

    Location of the `^` operator
   
    ```text
    case foo; in ^bar; end
                 ~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    case foo; in ^bar; end
                 ~~~~
    ```


## Postexe

    Represents `END { .. }` statement

Fields:

1. **body** (`MaybeNode { regexp_options: false }`)

    Body of the block

2. **keyword_l** (`Loc`)

    Location of the `END` keyword
   
    ```text
    END { 42 }
    ~~~
    ```

3. **begin_l** (`Loc`)

    Location of the open parenthesis
   
    ```text
    END { 42 }
        ~
    ```

4. **end_l** (`Loc`)

    Location of the closing parenthesis
   
    ```text
    END { 42 }
             ~
    ```

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    END { 42 }
    ~~~~~~~~~~
    ```


## Preexe

    Represents `BEGIN { ... }` statement

Fields:

1. **body** (`MaybeNode { regexp_options: false }`)

    Body of the block

2. **keyword_l** (`Loc`)

    Location of the `BEGIN` keyword
   
    ```text
    BEGIN { 42 }
    ~~~~~
    ```

3. **begin_l** (`Loc`)

    Location of the open parenthesis
   
    ```text
    BEGIN { 42 }
          ~
    ```

4. **end_l** (`Loc`)

    Location of the closing parenthesis
   
    ```text
    BEGIN { 42 }
               ~
    ```

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    BEGIN { 42 }
    ~~~~~~~~~~~~
    ```


## Procarg0

    Represents a sole block argument (e.g. `|foo|`)
   
    Block that takes a single array argument automatically expands it.
    Adding trailing comma after block argument disables this behavior (and then the only argument is emitted as `Arg`).

Fields:

1. **args** (`Nodes`)

    Parts of the sole block argument.
   
    `proc { |(a, b)| }` also counts as a sole argument, so this list may contain:
    1. A single `Arg` node (for `proc { |a| }` case)
    2. Multiple `Arg` nodes  (for `proc { |(a, b, c)| }` case)

2. **begin_l** (`MaybeLoc`)

    Location of the open parenthesis
   
    ```text
    proc { |(foo, bar)| }
            ~
    ```
   
    `None` if there's only one argument

3. **end_l** (`MaybeLoc`)

    Location of the open parenthesis
   
    ```text
    proc { |(foo, bar)| }
                     ~
    ```
   
    `None` if there's only one argument

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    proc { |(foo, bar)| }
            ~~~~~~~~~~
    ```


## Rational

    Represents rational literal (e.g. `1r`)

Fields:

1. **value** (`Str { raw: false }`)

    String value of the literal, `String("1r")` for `1r`

2. **operator_l** (`MaybeLoc`)

    Location of the unary `-` (but not `+`)
   
    ```text
    -1r
    ~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    -1r
    ~~~
    ```


## Redo

    Represents `redo` keyword

Fields:

1. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    redo
    ~~~~
    ```


## Regexp

    Represents regex literal (e.g. `/foo/`)

Fields:

1. **parts** (`Nodes`)

    A list of static and dynamic regex parts

2. **options** (`MaybeNode { regexp_options: true }`)

    Regex options.
   
    `None` if regex has no explicit flags

3. **begin_l** (`Loc`)

    Location of the regex begin
   
    ```text
    /foo/
    ~
   
    %r{foo}
    ~~
    ```

4. **end_l** (`Loc`)

    Location of the regex end
   
    ```text
    /foo/
        ~
   
    %r{foo}
          ~
    ```

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    /foo/mix
    ~~~~~~~~
    ```


## RegOpt

    Represents flags of the regex literal (i.e. `mix` for `/foo/mix`)

Fields:

1. **options** (`MaybeStr { chars: true }`)

    A list of flags

2. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    /foo/mix
         ~~~
    ```


## Rescue

    Represents a `rescue` block

Fields:

1. **body** (`MaybeNode { regexp_options: false }`)

    Body of the block that is wrapped into `rescue` (i.e. the part that may throw an error)

2. **rescue_bodies** (`Nodes`)

    A list of `rescue` handlers (see `RescueBody` node)

3. **else** (`MaybeNode { regexp_options: false }`)

    Else branch.
   
    `None` if there's no `else` branch

4. **else_l** (`MaybeLoc`)

    Location of the `else` keyword
   
    ```text
    begin; 1; rescue StandardError => e; 2; else; 3; end
                                            ~~~~
    ```
   
    `None` if there's no `else` branch

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    begin; 1; rescue StandardError => e; 2; else; 3; end
           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ```
   
    **Note**: `begin/end` keywords belong to `KwBegin` node


## RescueBody

    Represents a single `rescue` handler (i.e. `rescue E => e ...`)

Fields:

1. **exc_list** (`MaybeNode { regexp_options: false }`)

    A list of exception classes
   
    `None` if no classes specified (i.e. `rescue => e; ...` or just `rescue; ...`)

2. **exc_var** (`MaybeNode { regexp_options: false }`)

    Variable that captures exception
   
    `None` if no variable specified (i.e. `rescue E; ...` or just `rescue; ... `)

3. **body** (`MaybeNode { regexp_options: false }`)

    Body of the handler

4. **keyword_l** (`Loc`)

    Location of the `rescue` keyword
   
    ```text
    begin; 1; rescue E => e; 2; end
              ~~~~~~
    ```

5. **assoc_l** (`MaybeLoc`)

    Location of the `=>` operator
   
    ```text
    begin; 1; rescue E => e; 2; end
                       ~~
    ```
   
    `None` if exception is not captured.

6. **begin_l** (`MaybeLoc`)

    Location of the `then` keyword
   
    ```text
    begin; 1; rescue E => e then; 2; end
                            ~~~~
    ```
   
    `then` is optional, so `begin_l` can be `None`

7. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    begin; 1; rescue E => e then; 2; end
              ~~~~~~~~~~~~~~~~~~~~~
    ```


## Restarg

    Represents positional rest argument (i.e. `*foo` in `def m(*foo); end`)

Fields:

1. **name** (`MaybeStr { chars: false }`)

    Name of the argument.
   
    `None` if argument has no name (i.e. `def m(*); end`)

2. **operator_l** (`Loc`)

    Location of the `*` operator
   
    ```text
    def m(*foo); end
          ~
    ```

3. **name_l** (`MaybeLoc`)

    Location of the argument name
   
    ```text
    def m(*foo); end
           ~~~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    def m(*foo); end
          ~~~~
    ```


## Retry

    Represents `retry` keyword

Fields:

1. **expression_l** (`Loc`)

    Location of the `retry` keyword
   
    ```text
    retry
    ~~~~~
    ```


## Return

    Represents `return` keyword

Fields:

1. **args** (`Nodes`)

    A list of values that is returned

2. **keyword_l** (`Loc`)

    Location of the `return` keyword
   
    ```text
    return 1, 2
    ~~~~~~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    return 1, 2
    ~~~~~~~~~~~
    ```


## SClass

    Represents opening a singleton class (i.e. `class << foo; ... end;`)

Fields:

1. **expr** (`Node`)

    Expression that is used to get a singleton class
   
    `Lvar("foo")` for `class << foo; end`

2. **body** (`MaybeNode { regexp_options: false }`)

    Body of the block

3. **keyword_l** (`Loc`)

    Location of the `class` keyword
   
    ```text
    class << foo; end
    ~~~~~
    ```

4. **operator_l** (`Loc`)

    Location of the `<<` operator
   
    ```text
    class << foo; end
          ~~
    ```

5. **end_l** (`Loc`)

    Location of the `end` keyword
   
    ```text
    class << foo; end
                  ~~~
    ```

6. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    class << foo; end
    ~~~~~~~~~~~~~~~~~
    ```


## Self_

    Represents `self` keyword

Fields:

1. **expression_l** (`Loc`)

    Location of the `self` keyword
   
    ```text
    self
    ~~~~
    ```


## Send

    Represents a method call (e.g. `foo.bar(42)`)

Fields:

1. **recv** (`MaybeNode { regexp_options: false }`)

    Receiver of the method call
   
    `None` for implicit method call (e.g. `foo(42)`)

2. **method_name** (`Str { raw: false }`)

    Name of the method that is called

3. **args** (`Nodes`)

    A list of arguments

4. **dot_l** (`MaybeLoc`)

    Location of the `.` operator
   
    ```text
    foo.bar(42)
       ~
    ```
   
    `None` for implicit method call (e.g. `foo(42)`)

5. **selector_l** (`MaybeLoc`)

    Location of the method name
   
    ```text
    foo.bar(42)
        ~~~
    ```
   
    `None` in a very special case when method call is implicit (i.e. `foo.(42)`)

6. **begin_l** (`MaybeLoc`)

    Location of open parenthesis
   
    ```text
    foo(42)
       ~
    ```
   
    `None` if there are no parentheses

7. **end_l** (`MaybeLoc`)

    Location of closing parenthesis
   
    ```text
    foo(42)
          ~
    ```
   
    `None` if there are no parentheses

8. **operator_l** (`MaybeLoc`)

    Location of the operator if method is a setter
   
    ```text
    foo.bar = 42
            ~
    ```
   
    `None` otherwise

9. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo.bar(42)
    ~~~~~~~~~~~
    ```


## Shadowarg

    Represents a special block argument that "shadows" outer variable (i.e. `|;foo|`)

Fields:

1. **name** (`Str { raw: false }`)

    Name of the argument

2. **expression_l** (`Loc`)

    Location of the argument
   
    ```text
    proc { |;foo|}
             ~~~
    ```


## Splat

    Represents an arguments splat (i.e. `*bar` in a call like `foo(*bar)`)

Fields:

1. **value** (`MaybeNode { regexp_options: false }`)

    Value that is converted to array

2. **operator_l** (`Loc`)

    Location of the `*` operator
   
    ```text
    foo(*bar)
        ~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    foo(*bar)
        ~~~~
    ```


## Str

    Represents a plain non-interpolated string literal (e.g. `"foo"`)

Fields:

1. **value** (`StringValue`)

    Value of the string literal
   
    Note that it's a `StringValue`, not a `String`.
    The reason is that you can get UTF-8 incompatible strings
    from a valid UTF-8 source using escape sequences like `"\xFF"`
   
    These "\", "x", "F", "F" chars are valid separately, but together
    they construct a char with code = 255 that is invalid for UTF-8.
   
    You can use `to_string_lossy` or `to_string` methods to get a raw string value.

2. **begin_l** (`MaybeLoc`)

    Location of the string begin
   
    ```text
    "foo"
    ~
    ```
   
    `None` if string literal is a part of the words array (like `%w[foo bar baz]`)

3. **end_l** (`MaybeLoc`)

    Location of the string begin
   
    ```text
    "foo"
        ~
    ```
   
    `None` if string literal is a part of the words array (like `%w[foo bar baz]`)

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    "foo"
    ~~~~~
    ```


## Super

    Represents a `super` keyword

Fields:

1. **args** (`Nodes`)

    A list of arguments given to `super`

2. **keyword_l** (`Loc`)

    Location of the `super` keyword
   
    ```text
    super(1, 2)
    ~~~~~
    ```

3. **begin_l** (`MaybeLoc`)

    Location of the open parenthesis
   
    ```text
    super(1, 2)
         ~
    ```
   
    `None` if there are no parentheses

4. **end_l** (`MaybeLoc`)

    Location of the closing parenthesis
   
    ```text
    super(1, 2)
              ~
    ```
   
    `None` if there are no parentheses

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    super(1, 2)
    ~~~~~~~~~~~
    ```


## Sym

    Represents a plain symbol literal (i.e. `:foo`)
   
    Note that `:` in `{ foo: bar }` belongs to a `pair` node.

Fields:

1. **name** (`StringValue`)

    Value of the symbol literal
   
    Note that it's a `StringValue`, not a `String`.
    The reason is that you can get UTF-8 incompatible strings
    from a valid UTF-8 source using escape sequences like `"\xFF"`
   
    These "\", "x", "F", "F" chars are valid separately, but together
    they construct a char with code = 255 that is invalid for UTF-8.
   
    You can use `to_string_lossy` or `to_string` methods to get a raw symbol value.

2. **begin_l** (`MaybeLoc`)

    Location of the symbol begin
   
    ```text
    :foo
    ~
    ```
   
    `None` if symbol is a label (`{ foo: 1 }`) or a part of the symbols array (`%i[foo bar baz]`)

3. **end_l** (`MaybeLoc`)

    Location of the symbol end
   
    ```text
    { 'foo': 1 }
           ~
    ```
   
    `None` if symbol is **not** a string label (`:foo`) or a part of the symbols array (`%i[foo bar baz]`)

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    :foo
    ~~~~
   
    { foo: 1 }
      ~~~~
   
    %i[foo]
       ~~~
    ```


## True

    Represents a `true` literal

Fields:

1. **expression_l** (`Loc`)

    Location of the `true` keyword
   
    ```text
    true
    ~~~~
    ```


## Undef

    Represents an `undef` keyword (e.g. `undef foo, :bar`)

Fields:

1. **names** (`Nodes`)

    A list of names to `undef`

2. **keyword_l** (`Loc`)

    Location the `undef` keyword
   
    ```text
    undef foo, :bar
    ~~~~~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    undef :foo, bar
    ~~~~~~~~~~~~~~~
    ```


## UnlessGuard

    Represents an `unless` guard used in pattern matching (i.e. `in pattern unless guard`)

Fields:

1. **cond** (`Node`)

    Condition of the guard, `Lvar("foo")` in `in pattern unless guard`

2. **keyword_l** (`Loc`)

    Location of the `unless` keyword
   
    ```text
    case foo; in pattern unless cond; end
                         ~~~~~~
    ```

3. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    case foo; in pattern unless cond; end
                         ~~~~~~~~~~~
    ```


## Until

    Represents `until` loop

Fields:

1. **cond** (`Node`)

    Condition of the loop

2. **body** (`MaybeNode { regexp_options: false }`)

    Body of the loop.
   
    `None` if body is empty

3. **keyword_l** (`Loc`)

    Location of the `until` keyword
   
    ```text
    until cond do; foo; end
    ~~~~~
    ```

4. **begin_l** (`MaybeLoc`)

    Location of the `do` keyword
   
    ```text
    until cond do; foo; end
               ~~
    ```
   
    `do` is optional, and so `begin_l` can be `None`

5. **end_l** (`MaybeLoc`)

    Location of the `end` keyword
   
    ```text
    until cond do; foo; end
                        ~~~
    ```
   
    `None` if loop is a modifier (i.e. `foo until bar`)

6. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    until cond do; foo; end
    ~~~~~~~~~~~~~~~~~~~~~~~
   
    foo until bar
    ~~~~~~~~~~~~~
    ```


## UntilPost

    Represents a post-until loop
   
    ```text
    begin
    foo
    end until bar
    ```

Fields:

1. **cond** (`Node`)

    Condition of the loop

2. **body** (`Node`)

    Body of the loop

3. **keyword_l** (`Loc`)

    Location of the `until` keyword
   
    ```text
    begin; foo; end until bar
                    ~~~~~
    ```

4. **expression_l** (`Loc`)

    Location of the `until` keyword
   
    ```text
    begin; foo; end until bar
    ~~~~~~~~~~~~~~~~~~~~~~~~~
    ```


## When

    Represents a branch of the `case` statement (i.e. `when foo`)

Fields:

1. **patterns** (`Nodes`)

    A list of values to compare/match against

2. **body** (`MaybeNode { regexp_options: false }`)

    Body of the `when` branch

3. **keyword_l** (`Loc`)

    Location of the `when` keyword
   
    ```text
    case foo; when bar; end
              ~~~~
    ```

4. **begin_l** (`Loc`)

    Location of the `then` keyword
   
    ```text
    case foo; when bar then baz; end
                       ~~~~
    ```
   
    `then` is optional, and so `begin_l` can be `None`

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    case foo; when bar then baz; end
              ~~~~~~~~~~~~~~~~~
    ```


## While

    Represents `while` loop

Fields:

1. **cond** (`Node`)

    Condition of the loop

2. **body** (`MaybeNode { regexp_options: false }`)

    Body of the loop.
   
    `None` if body is empty

3. **keyword_l** (`Loc`)

    Location of the `while` keyword
   
    ```text
    while cond do; foo; end
    ~~~~~
    ```

4. **begin_l** (`MaybeLoc`)

    Location of the `do` keyword
   
    ```text
    while cond do; foo; end
               ~~
    ```
   
    `do` is optional, and so `begin_l` can be `None`

5. **end_l** (`MaybeLoc`)

    Location of the `end` keyword
   
    ```text
    while cond do; foo; end
                        ~~~
    ```
   
    `None` if loop is a modifier (i.e. `foo while bar`)

6. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    while cond do; foo; end
    ~~~~~~~~~~~~~~~~~~~~~~~
   
    foo while bar
    ~~~~~~~~~~~~~
    ```


## WhilePost

    Represents a post-while loop
   
    ```text
    begin
    foo
    end while bar
    ```

Fields:

1. **cond** (`Node`)

    Condition of the loop

2. **body** (`Node`)

    Body of the loop

3. **keyword_l** (`Loc`)

    Location of the `while` keyword
   
    ```text
    begin; foo; end while bar
                    ~~~~~
    ```

4. **expression_l** (`Loc`)

    Location of the `while` keyword
   
    ```text
    begin; foo; end while bar
    ~~~~~~~~~~~~~~~~~~~~~~~~~
    ```


## XHeredoc

    Represents a executable here-document literal (both with and without interpolation)
   
    It's similar to `Xstr` in terms of abstract syntax tree, but has different source maps.

Fields:

1. **parts** (`Nodes`)

    A list of string parts (static literals and interpolated expressions)

2. **heredoc_body_l** (`Loc`)

    Location of the executable here-document body
   
    ```text
    <<-`HERE`\n  a\n   #{42}\nHERE
             ~~~~~~~~~~~~~~~
    ```

3. **heredoc_end_l** (`Loc`)

    Location of the executable here-document end
   
    ```text
    <<-`HERE`\n  a\n   #{42}\nHERE
                              ~~~~
    ```

4. **expression_l** (`Loc`)

    Location of the executable here-document identifier
   
    ```text
    <<-`HERE`\n  a\n   #{42}\nHERE
    ~~~~~~~
    ```
   
    **Note**: This is the only node (with `Heredoc`) that has `expression_l` smaller that all other sub-locations merged.
    The reason for that is that it's possible to add more code after here-document ID:
   
    ```text
    <<-`HERE` + "rest"
    content
    HERE
    ```


## Xstr

    Represents an executable string (i.e. `` `sh #{script_name}` ``)

Fields:

1. **parts** (`Nodes`)

    A list of string parts (static literals and interpolated expressions)

2. **begin_l** (`Loc`)

    Location of the string begin
   
    ```text
    `#{foo}`
    ~
   
    %X{#{foo}}
    ~~~
    ```

3. **end_l** (`Loc`)

    Location of the string end
   
    ```text
    `#{foo}`
           ~
   
    %X{#{foo}}
             ~
    ```

4. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    `#{foo}`
    ~~~~~~~~
   
    %X{#{foo}}
    ~~~~~~~~~~
    ```


## Yield

    Represents an `yield` keyword

Fields:

1. **args** (`Nodes`)

    A list of arguments given to `yield`

2. **keyword_l** (`Loc`)

    Location of the `yield` keyword
   
    ```text
    yield 1, 2
    ~~~~~
    ```

3. **begin_l** (`MaybeLoc`)

    Location of the open parenthesis
   
    ```text
    yield(1, 2)
         ~
    ```
   
    `None` if there are no parentheses

4. **end_l** (`MaybeLoc`)

    Location of the closing parenthesis
   
    ```text
    yield(1, 2)
              ~
    ```
   
    `None` if there are no parentheses

5. **expression_l** (`Loc`)

    Location of the full expression
   
    ```text
    yield(1, 2)
    ~~~~~~~~~~~
    ```


## ZSuper

    Represents a `super` call without arguments and parentheses
   
    It's different from `super()` as it implicitly forwards current arguments

Fields:

1. **expression_l** (`Loc`)

    Location of the `super` keyword
   
    ```text
    super
    ~~~~~
    ```

