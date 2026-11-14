summon minecraft:allay ~ ~1 ~ {CustomName:{text:"Pip",color:"aqua"},CustomNameVisible:1b,PersistenceRequired:1b}
playsound minecraft:entity.allay.ambient_with_item master @a[distance=..32] ~ ~ ~
give @a[distance=..32] minecraft:amethyst_shard 4
function arnis:story/finale/end
