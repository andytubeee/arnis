$execute positioned $(x) $(y) $(z) unless entity @a[distance=..48] run return 0
$execute positioned $(x) $(y) $(z) unless loaded ~ ~ ~ run return 0
$summon minecraft:villager $(x) $(y) $(z) {Tags:["arnis.person","arnis.story_npc","arnis.story_new"],NoAI:1b,Invulnerable:1b,Silent:1b,PersistenceRequired:1b,Glowing:1b,CustomNameVisible:1b,VillagerData:{profession:"minecraft:nitwit",level:1,type:"minecraft:plains"}}
# On the ground where the place is, as travel lands players.
$spreadplayers $(x) $(z) 0 1 under $(y) false @e[type=minecraft:villager,tag=arnis.story_new]
data modify entity @e[type=minecraft:villager,tag=arnis.story_new,limit=1] CustomName set value {text:"",color:"aqua"}
data modify entity @e[type=minecraft:villager,tag=arnis.story_new,limit=1] CustomName.text set from storage arnis:story cur.npc
scoreboard players set @e[type=minecraft:villager,tag=arnis.story_new] arnis.wk -1
tag @e[type=minecraft:villager,tag=arnis.story_new] remove arnis.story_new
