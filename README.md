zenful-clicks
=============

A small terminal app to configure repeated key presses and toggles.

Quick start
-----------

- Build and run:

```
cargo run
```

Modifier-only keys (Shift/Ctrl/Alt/Meta)
---------------------------------------

If you want to configure standalone modifiers (for example `<Shift>`), run the app directly in a supported terminal and do not run it inside a multiplexer like `tmux`. If were to run the app in `tmux` it will not be able to configure standalone modifier keys.

If you must use a multiplexer, run the app outside it when capturing modifiers, or accept that modifier-only events may not be available.
