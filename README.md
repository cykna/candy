CandyLib

The Candy library is more like a library with a 'protocol' or a 'specification' with a default implementation.
The 'specification' part only defines what the library expects, but now how the implementor will do so.
By default it uses skia for the 2D and Wgpu for the 3D, but as this is a protocol, the unique thing is that the structs that
are used, implement the same high level methods it's expected. So an OpenGL renderer focused on 3D can be used if it is implemented
correctly.

Todo:
  2D stuff:
    Element Tree,
    Elements themself
    Add specifications for the renderers

  3D stuff:
    Initialize
    Add specification for renderers
    
  CrossPlatform
