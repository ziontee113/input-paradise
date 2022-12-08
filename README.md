## Entry point:

- `src/interceptor/mod.rs` --> Collects and print out the raw input events.

### To run it on your computer:

You must know your keyboard's path:
- In `main()`, call `devices::input::print_paths()`,
it will show you a list of paths for your input devices.
- `cargo run`, identify and copy your keyboard's path,
it usually ends with `/input0`.
- Goto `src/utils/mod.rs` and modify the HashMap in
`mock_device_alias()` function with your keyboard path.

Danger incoming! Before proceed:
- `interceptor::start()` will completely disable your keyboard,
you'd need to press `Tab` in order to quit the program.
The function responsible for quitting the program is
`dev_clear()`, can be found in `src/utils/dev_print.rs`.

- Go back to `main()`, uncomment `interceptor::start()`,
you can `cargo run`, press some keys and see things happen.
