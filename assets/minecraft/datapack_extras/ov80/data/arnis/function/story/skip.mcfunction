# Two skips, or a skip and a step, in one tick move the story once.
execute if score #moved arnis.story matches 1 run return 0
execute if score #step arnis.story >= #steps arnis.story run return run tellraw @s {"text":"That story's done. The next one starts in a moment.","color":"gray"}
tellraw @a [{"selector":"@s","color":"gold"},{"text":" skipped a step, in case the place is hard to reach.","color":"gray"}]
function arnis:story/advance
