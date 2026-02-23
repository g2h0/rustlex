# dialup — terminal dive watch demo
# Features shown: watch face, bezel rotation, star background, lume mode, smooth sweep

Output assets/demo.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 800
Set Theme "GruvboxDark"
Set WindowBar Colorful
Set WindowBarSize 36
Set BorderRadius 12
Set Padding 8
Set Framerate 24
Set LoopOffset 10%

# ── Build the release binary (hidden from recording) ────────────────────────
Hide
Type "cd /Users/g2h0/Library/CloudStorage/Dropbox/Dev/dialup && cargo build --release 2>&1 | tail -1"
Enter
Sleep 120s
Show

# ── Launch the watch ─────────────────────────────────────────────────────────
Type "./target/release/dialup"
Enter

# Let the watch face settle
Sleep 4s

# ── Bezel rotation — sweep clockwise then back ───────────────────────────────
Type@80ms "]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]"
Sleep 1s
Type@80ms "[[[[[[[[[[[[[[[[[[[[[[[[[[[[[["
Sleep 2s

# ── Stars on ─────────────────────────────────────────────────────────────────
Type "s"
Sleep 4s

# ── Lume mode — hands & markers glow green in darkness ───────────────────────
Type "l"
Sleep 5.5s

# Back to daylight
Type "l"
Sleep 2s

# Stars off (cleaner for next feature)
Type "s"
Sleep 1s

# ── Smooth sweep — mechanical glide vs quartz tick ───────────────────────────
Type "m"
Sleep 5s

# Back to quartz tick
Type "m"
Sleep 2.5s

# ── Quit ─────────────────────────────────────────────────────────────────────
Type "q"
Sleep 1s
