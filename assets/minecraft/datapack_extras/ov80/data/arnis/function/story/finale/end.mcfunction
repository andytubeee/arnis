# Every finale ends here: a fanfare for whoever came, and a breather before the next story.
playsound minecraft:ui.toast.challenge_complete master @a[distance=..48] ~ ~ ~ 1 1
title @a[distance=..48] times 10 60 20
title @a[distance=..48] title {"text":"Story complete","color":"gold","bold":true}
tellraw @a {"text":"That's the story! Another one starts somewhere else in town in a moment.","color":"gold"}
