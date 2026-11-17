# The crew is back: a moment together, the person's line, a thank-you, and the story goes on.
tellraw @a [{"text":"The crew is back together: ","color":"gold"},{"selector":"@a[gamemode=!spectator,distance=..12]","color":"yellow"}]
tellraw @a [{"storage":"arnis:story","nbt":"cur.npc","color":"aqua"},{"text":": ","color":"aqua"},{"storage":"arnis:story","nbt":"cur.after","color":"white"}]
particle minecraft:totem_of_undying ~ ~1.5 ~ 1.5 1 1.5 0.3 150
function arnis:story/finale/fireworks
playsound minecraft:ui.toast.challenge_complete master @a[distance=..48] ~ ~ ~ 1 1
give @a[gamemode=!spectator,distance=..12] minecraft:emerald 3
function arnis:story/advance
