function arnis:story/finale/fireworks
setblock ~1 ~ ~ minecraft:cake keep
playsound minecraft:music_disc.cat record @a[distance=..48] ~ ~ ~ 1 1
give @a[distance=..32] minecraft:cake
give @a[distance=..32] minecraft:firework_rocket 8
function arnis:story/finale/end
