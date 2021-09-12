A simple dice roller in Rust for _Four Against Darkness_ 
style dice codes. 

Currently works with the following codes:

* d6
* d6+1
* 2d6
* 2d6+1
* 2d6+2
* d6-1
* 3d6-2
* d6xd6
* d66
* d3
* d8

Any single-digit die type will work. 

Currently, only d6xd6 and d66 are implemented - d88 will not do what you expect.
(It will treat it like a 'd8'.)

Exploding only works with d6 rolls (not d66 or d6xd6), and you currently cannot 
turn if off.
