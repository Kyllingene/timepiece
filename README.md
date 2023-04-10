# timepiece
## A command-line all-purpose timepiece

### Features:
 - Get the current time (`tp time`), date (`tp date`), or both (`tp now`)
   - Keep getting it with `tp clock`
   - Stop with <kbd>Escape</kbd> or <kbd>Q</kbd>, lap with <kbd>Enter</kbd> or <kbd>Space</kbd>

 - Start a stopwatch (`tp stopwatch`)
   - Stop with <kbd>Escape</kbd> or <kbd>Q</kbd>, lap with <kbd>Enter</kbd> or <kbd>Space</kbd>

 - Set a timer (`tp timer [[[hours:]minutes:]seconds]`)
   - Cancel with <kbd>Escape</kbd> or <kbd>Q</kbd>, pause/unpause with <kbd>Enter</kbd> or <kbd>Space</kbd>
   - Rings using the terminals implementation of the BEL character

 - Set an alarm (`tp alarm hour:minute:second AM|PM`)
   - Cancel with <kbd>Escape</kbd> or <kbd>Q</kbd>
   - Rings using the terminals implementation of the BEL character

### Non-features:
 - Background/invisible timers/alarms (make a shim)
 - Configuration of any sort (edit the source, I suppose)

### If you want this name on crates.io, [contact me!](mailto:fuzzymuffin343@gmail.com)