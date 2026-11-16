# The grid cells at #cx, from #cz - 1 to #cz + 1: their footway spots onto the list.
execute store result storage arnis:story cell.cx int 1 run scoreboard players get #cx arnis.walk
execute store result storage arnis:story cell.cz int 1 run scoreboard players remove #cz arnis.walk 1
function arnis:story/leave_cell with storage arnis:story cell
execute store result storage arnis:story cell.cz int 1 run scoreboard players add #cz arnis.walk 1
function arnis:story/leave_cell with storage arnis:story cell
execute store result storage arnis:story cell.cz int 1 run scoreboard players add #cz arnis.walk 1
function arnis:story/leave_cell with storage arnis:story cell
scoreboard players remove #cz arnis.walk 1
