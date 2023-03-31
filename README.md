# timepiece
## A command-line all-purpose timepiece

### Features:
 - Get the current time (`tp time`), date (`tp date`), or both (`tp now`)
   - Keep getting it with `tp clock`
 - Start a stopwatch (`tp stopwatch`)
   - Stop with _Ctrl-C_, lap with _Enter_
 - Set a timer (`tp timer [[[hours:]minutes:]seconds]`)
   - Cancel with _Ctrl-C_
   - Rings using the terminals implementation of the BEL character
 - Set an alarm (`tp alarm hour:minute:second AM|PM`)
   - Cancel with _Ctrl-C_
   - Rings using the terminals implementation of the BEL character

### Non-features:
 - Actual user input (it just uses generic terminal behavior)
 - Background/invisible timers/alarms (use `tp ... &`)
 - Configuration of any sort (edit the source, I suppose)