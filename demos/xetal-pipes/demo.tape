# The README's terminal recording: just tape xetal-pipes (vhs) writes
# demo.gif. Each xetal<cmd> runs stages/<cmd>.xtl with X_eTaL.
Output demo.gif
Set Shell "bash"
Set FontSize 17
Set Width 1100
Set Height 640
Set Padding 18
Set TypingSpeed 35ms
Set Theme "Catppuccin Mocha"

Hide
Type "export PATH=$PWD/bin:$PATH PS1='$ ' && clear"
Enter
Show

Type "cat stages/head.xtl"
Enter
Sleep 3s
Type "xetalcat sample.txt | xetalhead -n 4"
Enter
Sleep 2s
Type "xetalcat sample.txt | xetaluniq | xetalwc -l"
Enter
Sleep 2s
Type "xetalcat sample.txt | xetalgrep the | xetalsort | xetalhead -n 3"
Enter
Sleep 2s
Type "xetalcat sample.txt | xetalsort | xetaluniq | xetaltail -n 4"
Enter
Sleep 2s
Type "xetalcat sample.txt | xetalsort | xetaluniq | xetalwc"
Enter
Sleep 4s
