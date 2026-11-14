summon minecraft:cat ~ ~ ~ {Tags:["arnis.story_pet"],CustomName:{text:"Mayor Whiskers",color:"gold"},CustomNameVisible:1b,PersistenceRequired:1b,variant:"minecraft:calico"}
data modify entity @e[type=minecraft:cat,tag=arnis.story_pet,limit=1,sort=nearest] Owner set from entity @p UUID
tag @e[tag=arnis.story_pet] remove arnis.story_pet
playsound minecraft:entity.cat.purreow master @a[distance=..32] ~ ~ ~
give @a[distance=..32] minecraft:cod 6
function arnis:story/finale/end
