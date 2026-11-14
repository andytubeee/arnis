# Every finale ends here: a fanfare for whoever came, and the end for everyone.
playsound minecraft:ui.toast.challenge_complete master @a[distance=..48] ~ ~ ~ 1 1
title @a[distance=..48] times 10 60 20
title @a[distance=..48] title {"text":"The End","color":"gold","bold":true}
tellraw @a {"text":"That's the story. Thanks for playing! Passers-by still have errands, and the city is all yours.","color":"gold"}
