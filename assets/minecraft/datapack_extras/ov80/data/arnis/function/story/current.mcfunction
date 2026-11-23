# arnis:story cur: the current step, or the last one once the story is over.
scoreboard players operation #i arnis.story = #step arnis.story
execute if score #i arnis.story >= #steps arnis.story run scoreboard players operation #i arnis.story = #steps arnis.story
execute if score #i arnis.story >= #steps arnis.story run scoreboard players remove #i arnis.story 1
execute store result storage arnis:story i.i int 1 run scoreboard players get #i arnis.story
function arnis:story/current_at with storage arnis:story i
# #sid tells this story's step apart from the same step of any other story, for the person
# waiting there and anything hidden.
scoreboard players operation #sid arnis.story = #story arnis.story
scoreboard players operation #sid arnis.story *= #16 arnis.story
scoreboard players operation #sid arnis.story += #step arnis.story
execute store result storage arnis:story cur.sid int 1 run scoreboard players get #sid arnis.story
data modify storage arnis:story cur.finale set from storage arnis:story finale
data modify storage arnis:story cur.gift set from storage arnis:story now.gift
