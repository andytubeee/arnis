summon minecraft:iron_golem ~2 ~ ~ {CustomName:{text:"The Guardian",color:"gray"},CustomNameVisible:1b,PersistenceRequired:1b,PlayerCreated:1b}
playsound minecraft:entity.iron_golem.repair master @a[distance=..32] ~ ~ ~
give @a[distance=..32] minecraft:poppy
give @a[distance=..32] minecraft:iron_ingot 6
function arnis:story/finale/end
