# Clipmanager_rs
Clipboard Manager for Wayland written in Rust

## Dependencies Required
wl-clipboard

## Usage
### To store clipboard run the following in the background on system startup
```wl-paste --watch <path to clipmanager-rs binary> --store```

### Show clipboard contents in the terminal 
```<path to clipmanager-rs binary> --list```

### Open clipboard contents in GUI
```<path to clipmanager-rs binary> --gui```

Clicking older entries will add it to the clipboard

### Clear all the stored clipboard entries
```<path to clipmanager-rs binary> --gui```
## Notes
- The database for the clipboard will be created in the working directory. Use the same directory for all the arguments. Using different directory will result in error or unexpected behaviour. This behaviour will be solved in future releases

## My sway Config

``` exec --no-startup-id wl-paste -t text --watch /mnt/Data/Rust/clipmanager_rs/target/release/clipmanager_rs -s  ```

---

**Not Production ready. Only for educational purpose**


