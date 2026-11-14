# #errand: 0 none, 1 offered by someone waving, 2 taken. Nothing fails; an errand nobody
# takes or finishes quietly goes away.
scoreboard players add #errand_t arnis.story 1
execute if score #errand arnis.story matches 0 if score #errand_t arnis.story >= #errand_wait arnis.story run function arnis:story/errand/offer
execute if score #errand arnis.story matches 1 unless entity @e[tag=arnis.errand_npc] run function arnis:story/errand/drop
execute if score #errand arnis.story matches 1 if score #errand_t arnis.story matches 120.. run function arnis:story/errand/drop
execute if score #errand arnis.story matches 2 run function arnis:story/errand/check
execute if score #errand arnis.story matches 2 if score #errand_t arnis.story matches 600.. run function arnis:story/errand/drop
