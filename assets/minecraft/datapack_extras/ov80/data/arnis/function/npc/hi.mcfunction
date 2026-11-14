execute store result storage arnis:npc pick.i int 1 run random value 0..11
data modify storage arnis:npc pick.list set value "replies"
function arnis:npc/say with storage arnis:npc pick
playsound minecraft:entity.villager.ambient neutral @a[distance=..12] ~ ~ ~
