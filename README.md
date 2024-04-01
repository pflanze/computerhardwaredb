# Computer hardware database

This is a collection of computer parts, currently just CPUs (mostly
AMD), with purchase options and detailed enough info to judge their
value for certain purposes, so as to decide what to buy.  Currently,
the value estimation is for the task of compiling Rust and C++ code,
but I want to also add calculations for their suitability for running
VMs and AI. It's a work in progress, currently only one shop in
Switzerland is covered, and the value calculation is currently
simplicistic. The output is currently just the list of the purchase
options, sorted by value (speed for the task divided by price).

The code is meant to be simple but have a sophisticated enough data
model to retain those details provided by manufacturers and sellers
that might prove useful for the evaluation.  Both the data model and
the data is defined in source code as structs/enum instances held in
memory. In other words, the source code is the database, which should
be useful for merging entries from other people while allowing
potential changes in the data model. The main files are
[types.rs](src/types.rs) for the main type definitions, and
[main.rs](src/bin/main.rs) for the data.  There are currently just two
main types/tables, `CPU` and `SoldAt`. The latter references the
`name` field in `CPU` by string (foreign key), the code builds an
index and verifies that there are no broken references, also whether
there are any CPUs that have no SoldAt entries.

I welcome forks and contributions. It would be interesting to get
wider coverage and improvements especially in these areas:

* Cover more shop and CPU options.

* Better performance estimate calculation (I have spent almost no time
  on this yet) in the `anticipated_compilation_performance` function,
  and add calculations for other purposes, e.g. AI (choosable via
  command line options).

* Compare performance estimates with reviews to validate the
  calculations.

* Add other computer parts, especially motherboards.

* A web or other UI could be made.

## Hacking

### Value wrapper

The `Value` type wrapper allows to specify missing values, as well as
values with notes about their source if in doubt. If such a value is
required for score evaluation at runtime, an error will happen and be
displayed, but without any information about what in structure it
should reside or even what type of value--instead of investing into
building tracking into this, I've decided to simply add a flag to
allow the code to panic (set the `DEBUG_VALUE` env var to any value,
e.g. the empty string), and running under the debugger then allows to
track down where the error happens and what the context is; something
like

    $ cargo build && DEBUG_VALUE= rust-gdb target/debug/main
    (gdb) run
    ..
    Program received signal SIGABRT, Aborted.
    ..
    (gdb) bt
    ...
    (gdb) f 15
    #15 0x0000555555590d7e in main::anticipated_compilation_performance (cpu=0x7ffffffeeeb0)
        at src/bin/main.rs:38
    38	        let launch_date_sec = cpu.launch_date.value()?.unixtime();
    (gdb) p cpu.name 
    $1 = computerhardwaredb::types::ArticleName ("AMD Ryzen 9 7950X3D")
    (gdb) 

shows the name of the `CPU` instance that is missing the value for `launch_date`.

Feel free to suggest a better alternative (that doesn't make the code
much more complicated).

### `into` and `try_into`

Some types, like `Date`, allow parsing from a string, for which
purpose the `TryFrom` trait is implemented (allowing
`try_into()?`). For non-failing conversions, the `From` trait is
implemented instead, allowing `into()` calls; this includes fields
that are defined to be of type `Value<...>`--the following uses
`into()` because the `graphics_model` field is of type
`Value<GraphicsModel>`, also allowing `Value::Missing`:

            graphics_model: GraphicsModel::None.into(),

In this case, the CPU does not have built-in graphics, but that info
was specified. This is distinct from `Missing`, which would mean that
the information wasn't provided by the manufacturer.
