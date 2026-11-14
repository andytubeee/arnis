scoreboard players set @s arnis.story.seen 0
execute if score #step arnis.story >= #steps arnis.story run return run tellraw @s [{"text":"City Stories: ","color":"gold","bold":true},{"text":"this city's story is already told; its steps are in your advancements. Passers-by still wave you over for errands.","color":"yellow","bold":false}]
function arnis:story/current
tellraw @s [{"text":"City Stories: ","color":"gold","bold":true},{"text":"someone at ","color":"yellow","bold":false},{"storage":"arnis:story","nbt":"cur.name","color":"white","bold":false},{"text":" is waiting for you. Look for the glowing person and right-click them. The bar at the top shows where to go next, and the pause menu points the way. Anyone can do a step for everyone.","color":"yellow","bold":false}]
