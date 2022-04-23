# nodes repo

Shared build scripts and metadata for `lib-ruby-parser` repos.

## Documentation

+ [Rendered NODES.md](NODES.md)
+ [Rendered MESSAGES.md](MESSAGES.md)

## Basic API

### Nodes

AST Nodes information is represented by `Node`/`NodeField`/`NodeFieldType` types, you can retrieve all nodes
by calling `nodes()` function. Every known Node has name and a list of fields (and both have comments as data):

```rust
use lib_ruby_parser_nodes::{nodes, Node, NodeField, NodeFieldType};

let all_nodes = nodes();
assert_eq!(nodes().len(), 124);

let def_node: &Node = all_nodes
    .iter()
    .find(|node| node.camelcase_name == "Def")
    .unwrap();
assert_eq!(def_node.fields.len(), 8);

let expression_l_field: &NodeField = def_node
    .fields
    .iter()
    .find(|field| field.snakecase_name == "expression_l")
    .unwrap();
assert_eq!(expression_l_field.field_type, NodeFieldType::Loc);
```

### Messages

Messages have the same API, but instead `Message`/`MessageField`/`MessageFieldType` types are used:

```rust
use lib_ruby_parser_nodes::{messages, Message, MessageField, MessageFieldType};

let all_messages = messages();
assert_eq!(messages().len(), 90);

let nth_ref_is_too_big_message: &Message = all_messages
    .iter().find(|message| message.camelcase_name == "NthRefIsTooBig").unwrap();
assert_eq!(nth_ref_is_too_big_message.fields.len(), 1);

let nth_ref_field: &MessageField = nth_ref_is_too_big_message
    .fields
    .iter()
    .find(|field| field.snakecase_name == "nth_ref")
    .unwrap();
assert_eq!(nth_ref_field.field_type, MessageFieldType::Str);
```

## Templates support

This repo is use mostly for code generation in other repos. There are more than 100 nodes and 100 messages, and so code generation is the best way to handle them.

[Liquid](https://github.com/cobalt-org/liquid-rust) is used as a primary template language:

```rust
use lib_ruby_parser_nodes::LiquidTemplate;

let template = LiquidTemplate::new_eval("
Nodes count: {{ nodes.size }}
Messages count: {{ messages.size }}
");
// Or LiquidTemplate::new("path/to/file.liquid")

assert_eq!(
    template.render().trim(),
    "Nodes count: 124\nMessages count: 90"
);
```

By default the following globals are available:

+ `nodes` - set to `lib_ruby_parser_nodes::nodes()`
+ `messages` - set to `lib_ruby_parser_nodes::messages()`

It's possible to register additional data by calling `.with_global` on a template:

```rust
use lib_ruby_parser_nodes::{
    LiquidTemplate,
    reexports::liquid::value,
};

let output = LiquidTemplate::new_eval("{{ custom_global }} bar")
    .with_global("custom_global", value!("foo"))
    .render();

assert_eq!(output, "foo bar")
```

By default the following filters are available:

+ [all default liquid filters](https://github.com/Shopify/liquid/wiki/Liquid-for-Designers#standard-filters)
+ `| camelcase_to_snakecase` - converts `FooBar` string to `foo_bar`
+ `| snakecase_to_camelcase` - converts `foo_bar` string to `FooBar`
+ `| escape_c_keyword` - appends `_` to a string if it's a C keyword
+ `| escape_rust_keyword` - appends `_` to a string if it's a Rust keyword
+ `| render_comment: "//", 4` - renders array of strings (like `node.comment`) to a string where each line is prefixed with "//" and has 4 spaces padding (except for the first line, it has no padding)

Custom filters can be registered by calling `.with_filter` on a template:

```rust
use liquid_core::{
    Result, Runtime, Value, ValueView,
    Display_filter, Filter, FilterReflection, ParseFilter,
};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "append_foo",
    description = "Appends 'foo' to a given string.",
    parsed(AppendFooFilter)
)]
pub struct AppendFoo;

#[derive(Debug, Default, Display_filter)]
#[name = "append_foo"]
struct AppendFooFilter;

impl Filter for AppendFooFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let input = input.to_kstr();
        let output = format!("{}foo", input);
        Ok(Value::scalar(output))
    }
}

use lib_ruby_parser_nodes::LiquidTemplate;

let template = LiquidTemplate::new_eval("{{ 'data ' | append_foo }}")
    .with_filter(AppendFoo)
    .render();

assert_eq!(template, "data foo");
```

You can also check more complicated filter (like [`render_comment`](https://github.com/lib-ruby-parser/nodes/blob/master/src/filters/render_comment.rs)) to understand how parameters can be passed.
