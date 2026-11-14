summon minecraft:wolf ~ ~ ~ {Tags:["arnis.story_pet"],CustomName:{text:"Biscuit",color:"gold"},CustomNameVisible:1b,PersistenceRequired:1b,variant:"minecraft:rusty"}
data modify entity @e[type=minecraft:wolf,tag=arnis.story_pet,limit=1,sort=nearest] Owner set from entity @p UUID
tag @e[tag=arnis.story_pet] remove arnis.story_pet
particle minecraft:heart ~ ~1 ~ 0.6 0.4 0.6 0 12
give @a[distance=..32] minecraft:bone 8
function arnis:story/finale/end
