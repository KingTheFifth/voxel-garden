#set page(paper: "a4", margin: (x: 4cm))

#set text(size: 12pt)

= Voxel Garden

Deltagare:

- Johannes Kung (johku144)
- Martin Högstedt (marho227)
- Gustav Sörnäs (gusso230)

== Beskrivning

En voxel-trädgård som man går runt i. Generera olika typer av växtlighet (gräs,
blommor, träd, buskar) och djur/insekter (fåglar, grodor). Mysigt ljus. Mycket
fokus på procedurell generation.

== Kommer göra

- Gå runt med WASD och mus. Hoppa med mellanslag. Kamera i första-person.
- Voxlar, med inte så mycket texturing. En voxel är en enda färg, platt.
- Ljus: gouraud. Många ljuskällor (vedeld, eldflugor).
- Procedurell genereration av
  - terräng/"biomes"
  - träd
  - blommor
  - gräs
- Terräng-generering påverkas av en ritad karta, ungefär som en splat map. Olika
  kartor för platt/berg, slätt/skog etc.
- Partiklar
  - Pollen

== Kommer kanske göra

- Ljudeffekter
- Nätverking, gå runt i varandras trädgårdar
- Dag/natt-cykel
- Placera ut växter, terräng, interaktivt
- Animation från vind och personer som går runt.
- Vattendrag
- Liten ray trace/cast för skuggor?
