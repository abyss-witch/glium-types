0.6.1
Added changelog
Added conversion from vectors to its tuple and array form

0.7.0
added conversions from matrices to its array form (both column and row major)
fixed typo in mat4 and dmat4 `transpose` function
changed transpose, extend, splat and truncate to be const functions
added dividing to matrices
added matrix, quaternion and vector multiplying, dividing and rem by and from scalar
added matrix multiplying and dividing by vectors
removed unecessary borrowing
fixed doubble vectors requiring a float matrix rather than a double matrix
added boolean vectors
added comparisons to vectors that return bool vectors
changed prelude imports to use all types rather than just the float variants
changed the mesh macro to return a `Result<_, MeshError>` to make it more useful
changed exports so that glium is rexported
fixed vertex shaders normals and changed `view` uniform to `perspective`
changed `view_matrix_3d` and `view_matrix_2d` to `perspective_3d` and `perspective_2d`
added quaternion MulAssign and Dividing
