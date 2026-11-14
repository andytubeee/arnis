execute if score #step arnis.story >= #steps arnis.story run return run tellraw @s {"text":"The story is over. Passers-by still have errands.","color":"gray"}
tellraw @a [{"selector":"@s","color":"gold"},{"text":" skipped a step, in case the place is hard to reach.","color":"gray"}]
function arnis:story/advance
