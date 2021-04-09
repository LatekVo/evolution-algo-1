# evolution-algo-1
All things written here may be work in progress, as such, may not be **yet** included in the code.

Improved version of my old evolution-algo-0. <br>
Dubbed face-fronter, but able of manipulation of any vector of numbers (32-bit float). <br>
Unlike old algorithm, this one was designed with modularity in mind from start, as such, <br>
it divides every structure as parent of multiple children with vectors. <br>
Added "super-nodes" which store multiple other nodes, which makes things more efficient as long as we dont count unused nodes inside, <br>
as this enables each node to "choose" type of equasion from those that happen to be inside "supernode" and enable more <br>
complicated equsaions without really adding layers. Now inputs dont effect all nodes, but only affect fraction of nodes that are "input nodes", <br>
this will help spot and eliminate unused nodes and skip them. <br>
