Output assets/demo.gif

Set FontSize 14
Set Width 1200
Set Height 800
Set Theme "Aardvark Blue"
Set Margin 0
Set Padding 0
Set BorderRadius 0
Set Shell "powershell"

Hide
Type "sprout sample"
Enter
Sleep 3s
Show

Sleep 1s

# Menu is in focus at start, Tab to enter dashboard
Tab
Sleep 0.5s

# Reading (numeric, goal 30)
Enter
Sleep 0.5s
Type "30"
Sleep 0.3s
Enter
Sleep 1s

# Exercise (binary)
Down
Sleep 0.5s
Enter
Sleep 1s

# Meditation (numeric, goal 15)
Down
Sleep 0.5s
Enter
Sleep 0.5s
Type "15"
Sleep 0.3s
Enter
Sleep 1s

# Coding (numeric, goal 60)
Down
Sleep 0.5s
Enter
Sleep 0.5s
Type "60"
Sleep 0.3s
Enter
Sleep 1s

# Journaling (binary)
Down
Sleep 0.5s
Enter
Sleep 1s

# Pause to see completed dashboard
Sleep 2s

# Tab to enter goal progress pane
Tab
Sleep 1s

# Daily tab - scroll through habits
Down
Sleep 0.5s
Down
Sleep 0.5s
Down
Sleep 1s

# Switch to weekly tab
Right
Sleep 1s

# Switch to monthly tab
Right
Sleep 1s

# Switch to yearly tab
Right
Sleep 1.5s

# Tab to best streaks, Tab again to menu
Tab
Sleep 0.5s
Tab
Sleep 0.5s

# Menu at Dashboard (0), Down to Heatmap (1), Tab to enter
Down
Sleep 0.5s
Tab
Sleep 2s

# m to menu, Down to Stats (2), Tab to enter
Type "m"
Sleep 0.5s
Down
Sleep 0.5s
Tab
Sleep 2s

# m to menu, Down to Settings (3), Tab to enter
Type "m"
Sleep 0.5s
Down
Sleep 0.5s
Tab
Sleep 2s

# m to menu, Up Up Up back to Dashboard (0), Tab to enter
Type "m"
Sleep 0.5s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Tab
Sleep 1s

# Press a to open add modal
Type "a"
Sleep 0.5s

# Type habit name
Type "Running"
Sleep 0.5s

# Tab to daily goal
Tab
Sleep 0.3s
Type "10"
Sleep 0.3s

# Tab to weekly goal
Tab
Sleep 0.3s
Type "50"
Sleep 0.3s

# Tab to monthly goal
Tab
Sleep 0.3s
Type "150"
Sleep 0.3s

# Tab to yearly goal
Tab
Sleep 0.3s
Type "1000"
Sleep 0.5s

# Submit
Enter
Sleep 2s

# Quit
Type "q"