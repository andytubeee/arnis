# One footway spot ([lane, point]) off the list: kept in arnis:walk spawn if the nearest yet.
data modify storage arnis:story e set value {}
data modify storage arnis:story e.s set from storage arnis:story spots[0][0]
data modify storage arnis:story e.a set from storage arnis:story spots[0][1]
function arnis:story/leave_point with storage arnis:story e
data remove storage arnis:story spots[0]
execute if data storage arnis:story spots[0] run function arnis:story/leave_spot
