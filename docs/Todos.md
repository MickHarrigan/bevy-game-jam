## Debug boids
- [ ] Spawn a debug boid after loading level. It should have a different texture and still considered a bee
- [ ] Show it's sight radius around it with a 2D shape
```rust
// Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });
```
- [ ] Verify that it's sight is only in a cone in front of the boid
	- `Dot product(ourDirection, otherBoidDirection) > visionConeThreshold`

- [ ] Verify separation works as intended. This may fix our stacking problem
	- All boids in range exert an opposing direction, intensity that decreases with the distance of the two

- [ ] How hard would it be to display the edges of our QuadTree as part of a toggable debug ui?

## Boid gameplay
- [ ] Each boid should have a destination position that it tries to navigate towards, if in traveling state, instead of just forwards

- [ ] Introduce collision avoidance for walls and fog of war if not exploring
	- Fetch overlapping colliders in a radius and just like the separation rule, colliders exert an opposed direction

- [ ] Steering / Fog of war rule
	- Each tile has a discovered boolean, determining if it is in fog or not
	- If an exploring bee touches an undiscovered tile, it toggles preeminently to discovered
	- Add a separation rule just like for collisions, but attracted to undiscovered tiles