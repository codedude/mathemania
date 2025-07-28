# Research

**Delaunay Triangulation** = **DT**

## Different implementations (fastest one)

- Divide & Conquer, Guibas and Stolfi
- Divide & Conquer, Dwyer improvements over G&S
- Divide & Conquer, n-dimension DeWall

> Intelligently pick the splitting lines is the key

### How it works

- [Step by step Guibas and Stolfi](http://www.geom.uiuc.edu/~samuelp/del_project.html)
  - Constrained edges: [Applications in real life](https://gwlucastrig.github.io/TinfourDocs/DelaunayIntroCDT/index.html)

### Implementations

- [Great article with visualization](https://ianthehenry.com/posts/delaunay/)
- [Guibas and Stolfi fix consideration](https://people.eecs.berkeley.edu/~jrs/meshpapers/GSflaws)
- [2D quality mesh generator, Shewchuk paper](https://people.eecs.berkeley.edu/~jrs/papers/triangle.pdf)
- [Dwyer algorithm paper](https://dl.acm.org/doi/pdf/10.1145/10515.10545)

### Keywords

- Dual graph
- Quad edge

### Going further

- [Gabriel graph](https://en.wikipedia.org/wiki/Gabriel_graph)
- [Voronoi diagram](https://en.wikipedia.org/wiki/Voronoi_diagram)
- [Dual graph](https://en.wikipedia.org/wiki/Dual_graph)
- [Spanning tree](https://en.wikipedia.org/wiki/Spanning_tree)

## Language and libraries

- Rust
- Target: MacOS, Linux, Wasm
- Graphic: [three-d](https://github.com/asny/three-d?tab=readme-ov-file)
