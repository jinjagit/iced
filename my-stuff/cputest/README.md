## A simple system monitor
Uses Stopwatch example as basis to develop simple cpu usage monitor.

The Stopwatch app, that was used as the starting point for this app can be found at: https://github.com/jinjagit/iced/tree/master/examples/stopwatch which is a fork of: https://github.com/hecrj/iced/tree/master/examples/stopwatch

Made simply to practice and explore using the iced crate.

The stop / start button is not really useful at all, but is included just to include some UI interaction.

I have 2 main questions, after getting this far:
1. `iced::time`, which is used in the subscription that triggers `Tick` in this app, seems to require an `Instant` is passed around, but I am not exactly sure why. The compiler warns that the `Instant` field in the `Ticking` state is dead code, and yet removing it will break the code. I feel like I should be able to greatly simplify this part of this app, since I am only concerned with triggering an update regularly, not with calculating / displaying time related data (as in the Stopwatch example). How can I simplify this aspect?
2. How can I pull all the necessary features from the crate, without embedding my app within a fork of the `iced` repo? For example, if I use `iced = { version = "0.2.0", features = ["smol"] }` in dependencies, in my `Cargo.toml` file, I get the error `the package `cputest` depends on `iced`, with features: `smol` but `iced` does not have these features.` when building the app.

## Screen-capture:
![iced_gui.gif](https://github.com/jinjagit/iced/blob/master/my-stuff/cputest/gif/iced_gui.gif)
