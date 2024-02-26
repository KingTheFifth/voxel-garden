#set page(paper: "a4", margin: (x: 3cm, y: 2cm))

#set text(size: 12pt)

= Voxel Garden

Deltagare:

- Gustav Sörnäs (gusso230)
- Johannes Kung (johku144)
- Martin Högstedt (marho227)

== Beskrivning

En voxel-trädgård som man går runt i. Generera olika typer av växtlighet (gräs,
blommor, träd, buskar) och djur/insekter (fåglar, grodor). Mysigt ljus. Mycket
fokus på procedurell generation.

== Kommer göra

- Gå runt med WASD och mus. Hoppa med mellanslag? Kamera i första-person. Kan ta mycket från labbserien.
- Voxlar, med inte så mycket texturing. En voxel är en enda färg, platt.
- Ljus: gouraud. En sol, directional.
- Procedurell genereration av terräng/"biomes", träd, blommor, gräs
- Terräng-generering påverkas av en ritad karta, ungefär som en splat map. Olika
  bilder för platt/berg, slätt/skog etc.
- Partiklar
  - Pollen
- Baka kladdkaka

== Kommer kanske göra

- Skuggor. Vi vet inte exakt hur vi vill göra, men en tanke just nu är att byta solen till positional om det innebär att skuggorna blir lättare. Det såg ut så när vi gjorde lite snabb efterforskning.
- Ljudeffekter: regn, vind, fåglar.
- Grafiska vädereffekter: regn, vind. Påverkar fysiken (gräs/löv blåser i vinden).
- Att gå runt påverkar fysiken (puttar undrar gräs/löv).
- Placera ut växter, terräng, interaktivt.
- Nätverking, gå runt i varandras trädgårdar. (Inte så kul utan interaktivitet.)
- Dag/natt-cykel.
- Vattendrag.

== Uppdelning

I början av projektet behöver vi programmera tillsammans för att få till en gemensam grund. Då fokuserar vi på:

- Rendera en lista av voxlar med olika färger. Fokusera på att hitta en datastruktur.
- Sätt upp funktioner för procedurell generation. Implementera inte än!\ `vector<voxel> flower(int seed);`
- Flytta över kamera och movement från labbarna.

Nu kan vi dela upp oss! Implementera funktioner för procedurell generation. Vi kan testa terrängen genom att rita ut platser där t.ex. blommor ska placeras ut, vilket inte kräver färdiga implementationer.
