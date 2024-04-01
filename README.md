# The Goal
The goal of this project is two-fold:
1. Learn some Rust
2. Build a terminal notes app


The first goal is self-explanatory. Now let's elaborate on the second goal.

## Terminal Notes App

The app should allow me to quickly write notes in my terminal and have them automatically saved on my local filesystem. No internet connectivity, no crazy editor features, just simple text in files.

I envision having a command called `note` that will open a vim screen where I write a note, save it, and then the note is automatically saved in a `$NOTEHOME` directory with the following structure:

```
- $NOTEHOME
     |
     - 2024
         |
         - March
            |
            - 30.txt
            - 31.txt
    ...etc
```

Later, it'd be nice to add the following:

- Cloud backup
- search
- backlinks
