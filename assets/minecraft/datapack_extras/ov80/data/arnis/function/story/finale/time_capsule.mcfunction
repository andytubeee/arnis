particle minecraft:poof ~ ~0.5 ~ 0.5 0.3 0.5 0.02 20
playsound minecraft:block.chest.open master @a[distance=..32] ~ ~ ~
give @a[distance=..32] minecraft:written_book[minecraft:written_book_content={title:"Letters from 1985",author:"Mr. Hale's class",pages:["Dear future us, I hope there are flying cars by now. - Dana","If you're reading this, I still think I'd win the bike race. - Marco","Plant a tree for us. Love, room 12."]}]
give @a[distance=..32] minecraft:clock
function arnis:story/finale/end
