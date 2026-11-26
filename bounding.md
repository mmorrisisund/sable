# Bounding Box
    
## Properties
  - Min, Max Vec3

## Methods
  - Center
  - Size
  - union
    - min other min
    - max other max
  - is finite
    - min/max is finite
  - transform (with matrix)
    - list of corners
    - new min/max at extremities
    - transform each corner with matrix
    - compare with new_min/max
    - return new_min/max as box

# Compute Bounding Volume
 - Get position attrs or exit
 - verify vec3f
 - Get actual data points
 - get description of data: count, stride, offset
 - most min/max points in geometry


# Scene - Get BBox of Node
 - take node_handle and asset_server(maybe)
 - get initial bbox of node
 - get node
 - for all children
   - get bbox of child
   - if initial bbox exists union with child
   - else initial becomes child
 - return initial (combined)

 ## Get BBox of One Node
  - get node if possible
  - if not visible exit
  - get mesh if possible
  - if not visible exit
  - get local bbox of mesh's geometry
  - transform local bbox with node's world matrix
  - return Some(local_bbox)