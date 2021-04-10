In here, you can put training data, labeled:<br>
<br>
o\_0.png<br>
i\_0\_0.png<br>
i\_0\_1.png<br>
i\_0\_2.png<br>
o\_1.png<br>
i\_1\_0.png<br>
<br>
and so on.<br>
<br>
Algorithm is gonna be adapted to accept all types of media, but currently, it only accepts .png files.<br>
You can write your own file parser, since whole system is modular and data gathering is just an overwritable function.<br>

Current algorithm will automatically create data nodes, such that each node has single output and multiple inputs.
