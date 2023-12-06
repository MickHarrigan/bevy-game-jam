
# Objective
- Discover more of the map around you, covered by fog of war
	- Bees must **explore** into the fog for those tiles to become visible and targeted in the future
- Interact with entities / points of interest in discovered view
	- Some useful like *Flowers* whose pollen will help spawn more bees
	- Some harmful which may kill bees
	- Some act as objectives, which must be brought back to base to advance
- Balance between having some bees explore the map, interacting with entities, and and protecting the home hive from other threats? 

# Player Actions
- Highlight in a region bee to control
	- Left / Right click and drag spawns a highlight box
- Selected bees can then preform bee specific actions
	- Click on ground = move towards that world coordinate
	- Click on level entity = group up at and interact with entity
	- Click on wonder button = bees slowly disperse in local area
	- Click on **explore** button = bees more aggressively 

# Bee Behavior

```rust
enum BeeBehaviorState {
	Traveling (Move twoards players location),
	Wondering (Stay put in radius around self),
	Exploring (Prioritize fog of war),
	Amalgamate (Waiting for other bees to interact with an entity)
}
```