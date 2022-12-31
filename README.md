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

---

Not Production ready. Only for educational purpose
