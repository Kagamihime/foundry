# Foundry
A GPU-accelerated cellular automata library using Vulkan.

It is a library that aims at manipulating life cellular automata grids.
For now it supports:
* toroidal and resizable grids
* loading and saving files containing grid data
* stepping forward the generations of a grid (if it is a resizable grid, it will make sure the pattern is always at the center of the grid)

## What this library aims at
* Support for distributed and parallel computation of grids' operations (GPUs and networking).
* Usage of machine learning for pattern analysis.

## About the file formats used
For now, Foundry uses two internal file formats: Resizable Life and Toroidal Life.

### Resizable Life
This file format is close to the Life 1.06 format:
* The "#Resizable Life" is followed by optional description lines, which begin with "#D". Leading and trailing spaces are ignored.
* Next comes an optional rule specification. The patterns in the collection here enforce "Normal" Conway rules using the "#N" specifier. Alternate rules use "#R" ("#N" is exactly the same as "#R 23/3"). Rules are encoded as Survival/Birth, each list being a string of digits representing neighbor counts. Since there are exactly eight possible neighbors in a Conway-like rule, there is no need to separate the digits, and "9" is prohibited in both lists.
* And finally comes a list of (x y) coordinates with live cells.

### Toroidal Life
This file format is close to the Life 1.06 format:
* The "#Toroidal Life" is followed by optional description lines, which begin with "#D". Leading and trailing spaces are ignored.
* Next comes an optional rule specification. The patterns in the collection here enforce "Normal" Conway rules using the "#N" specifier. Alternate rules use "#R" ("#N" is exactly the same as "#R 23/3"). Rules are encoded as Survival/Birth, each list being a string of digits representing neighbor counts. Since there are exactly eight possible neighbors in a Conway-like rule, there is no need to separate the digits, and "9" is prohibited in both lists.
* Next there is a line like this "#S <rows> <cols>" which define the size of the grid.
* And finally comes a list of (x y) coordinates with live cells.
