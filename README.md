# timepiece (`tp`)
![Your command-line Rolex](https://repository-images.githubusercontent.com/621942877/94dd7a7e-b876-41a6-9f32-67f134324247)

### Features:
 - Get the current time (`tp time`), date (`tp date`), or both (`tp now`)
   - Keep getting it with `tp clock`
   - Stop with <kbd>Escape</kbd> or <kbd>Q</kbd>
   - Lap with <kbd>Enter</kbd> or <kbd>Space</kbd>

 - Start a stopwatch (`tp stopwatch`)
   - Stop with <kbd>Escape</kbd> or <kbd>Q</kbd>
   - Lap with <kbd>Enter</kbd>, <kbd>Space</kbd>, or <kbd>L</kbd>
   - If piped (e.g. `sleep 5 | tp stopwatch`), times process while relaying stdin

 - Set a timer (`tp timer [[[hours:]minutes:]seconds]`)
   - Cancel with <kbd>Escape</kbd> or <kbd>Q</kbd>
   - Pause/unpause with <kbd>Enter</kbd> or <kbd>Space</kbd>
   - Add 5 seconds with <kbd>Right</kbd> or <kbd>A</kbd>
   - Remove 5 seconds with <kbd>Left</kbd> or <kbd>D</kbd>
   - Rings using the BEL character

 - Set an alarm (`tp alarm hour:minute:second AM|PM`)
   - Cancel with <kbd>Escape</kbd> or <kbd>Q</kbd>
   - Rings using the BEL character

 - Enable desktop notifications with feature flag `notify`, for:
   - Timers
   - Alarms
   - Piped stopwatches

### Installation

The best way to install, if you have `cargo` installed, is through that:

```shell
$ cargo install timepiece # add `--features notify` if you want desktop notifs
```

### Non-features:
 - Background/invisible timers/alarms (make a shim, possibly)
   - This is because there don't seem to be any cross-platform fork libraries
 - Configuration of any sort (edit the source, not to sound like suckless)

### If you want this name on crates.io, [contact me!](mailto:fuzzymuffin343@gmail.com)
