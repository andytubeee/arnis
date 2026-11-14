# The ground around blooms: flowers where there's grass and room.
fill ~-4 ~ ~-4 ~4 ~ ~4 minecraft:poppy replace minecraft:short_grass
fill ~-4 ~ ~-4 ~4 ~ ~4 minecraft:cornflower replace minecraft:air
particle minecraft:happy_villager ~ ~0.5 ~ 4 0.5 4 0 80
give @a[distance=..32] minecraft:sunflower 4
give @a[distance=..32] minecraft:bone_meal 16
function arnis:story/finale/end
