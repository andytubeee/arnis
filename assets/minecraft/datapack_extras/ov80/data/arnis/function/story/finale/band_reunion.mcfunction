playsound minecraft:music_disc.chirp record @a[distance=..48] ~ ~ ~ 1 1
particle minecraft:note ~ ~2 ~ 1.5 0.5 1.5 1 20
give @a[distance=..32] minecraft:jukebox
give @a[distance=..32] minecraft:music_disc_chirp
function arnis:story/finale/end
