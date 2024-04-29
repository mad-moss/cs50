It's a library.

`get_input` is a generic replacement of `get_int`, `get_float`, etc.

it works for any type that implements `std::str::FromStr`

use it like this

`get_input::<your_type>("ask the user a question")`

it *might* work without the explicit type declaration, depending on the context, but I haven't tested it so no promises
