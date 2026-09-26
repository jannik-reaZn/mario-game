# How Physics and Collisions Work

A high-level tour of how the player falls, jumps and bumps into things.
No code details here, only the ideas.

---

## 1. Gravity and Velocity

### The core idea

The player has a **velocity**: a single number for vertical speed.

- A **positive** velocity means moving **down** the screen.
- A **negative** velocity means moving **up**.

This looks backwards if you are used to maths class. Screens put the origin
(0, 0) in the **top-left** corner, so `y` grows as you go _down_.

Two simple rules drive all vertical motion:

| Rule                          | In plain words                                                    |
| ----------------------------- | ----------------------------------------------------------------- |
| **Gravity changes velocity**  | Every moment, gravity makes the player a little faster downwards. |
| **Velocity changes position** | Every moment, the player moves by "speed × time".                 |

That is the whole model. Falling looks natural because speed _builds up_
gradually instead of being constant.

### Time is measured in frames

The game loop runs many times per second. Each pass ("frame") knows how much
real time passed since the last one. This is called the **delta time** (`dt`).

Everything is scaled by `dt`:

```
velocity  = velocity + gravity × dt
position  = position + velocity × dt
```

Why scale by `dt`? Without it, the game would run faster on a fast computer
and slower on a slow one. With it, one second of falling looks the same on
every machine.

### One frame, step by step

```
   ┌──────────────────────────────────────────────┐
   │ 1. Read input (left / right / jump)          │
   │ 2. Gravity speeds the player up (downwards)  │
   │ 3. Velocity moves the player                 │
   │ 4. Collision check corrects the position     │
   │ 5. Draw everything                           │
   └──────────────────────────────────────────────┘
```

Order matters. The player is first moved _as if nothing were in the way_,
and only afterwards do we look for problems and fix them (see part 2).

### Jumping

A jump is not a special movement. It is just a **one-time kick to velocity**:

1. The player presses Space while standing on something.
2. Velocity is set to a strong **negative** value (fast upwards).
3. From then on, normal gravity takes over. Each frame gravity reduces the
   upward speed, until it hits zero at the top of the arc, and then the player
   starts falling.

The result is a natural arc, and we never had to describe the arc itself.

A jump is only allowed when the player is **grounded**. Without that check the
player could jump again in mid-air and fly forever.

### Horizontal movement

Left and right are deliberately simpler: while a key is held, the player
moves at a **constant speed**. There is no acceleration or sliding.

**Tradeoff:** Constant speed feels very direct and is easy to reason about.
Real acceleration would feel more "Mario-like", but adds more numbers to tune.
Starting simple is a good choice here.

### Player state

The player is either **grounded** or **jumping** (airborne). This tiny piece of
state answers one question: _"is the player allowed to jump right now?"_

---

## 2. Collision Detection

Collisions are handled in **two stages**, every frame:

1. **Detect**: is the player overlapping an obstacle, and from which side?
2. **Resolve**: push the player back out and adjust velocity and state.

### Stage 1: Everything is a box

To keep the maths easy, the player and every obstacle are treated as
**rectangles** aligned with the screen axes (an _AABB_, "axis-aligned bounding
box"). Each rectangle is described by four edges: left, right, top, bottom.

### Do two boxes overlap?

Two boxes overlap only if they overlap **on both axes at the same time**.

```
   No overlap (gap on x)         Overlap (both axes)
   ┌────┐                        ┌────┐
   │ P  │   ┌────┐               │ P ┌┼───┐
   └────┘   │ O  │               └───┼┘ O │
            └────┘                   └────┘
```

If there is a gap on either axis, they cannot touch. Boxes that merely share
an edge do **not** count as colliding. This matters: a player standing
exactly on a floor is not "inside" it.

### Which side was hit?

Knowing that we overlap is not enough. Landing on a platform and walking into
its wall need _different reactions_. So the game asks:

> How deep is the player sunk into each of the obstacle's four faces?

It measures the overlap depth against the top, bottom, left and right face.
**The smallest depth wins**, because that is the face the player most likely
came through.

```
   Player sinks 2px into the top face, but 15px into the left face
   → the top face has the smallest depth → treat it as a Top hit (landing)
```

**Tradeoff:** This is a cheap and good-enough guess. It only looks at the
current frame's overlap, not at where the player _was_ in the previous frame.
Tracking the previous position would be more accurate, but more complex. At
normal speeds, the simple approach works well. A very fast player could in
theory skip through a thin obstacle in a single frame (called _tunnelling_).

### Stage 2: Resolving the collision

Each of the four sides has its own reaction:

| Side hit   | Meaning                                | Reaction                                                                                     |
| ---------- | -------------------------------------- | -------------------------------------------------------------------------------------------- |
| **Top**    | Player lands on the obstacle           | Place the player standing on top, stop falling, mark as **grounded**.                        |
| **Bottom** | Player bumps their head                | Place the player just below, stop upward speed, stay **airborne** (they now fall back down). |
| **Left**   | Player runs into a wall from the left  | Push the player back out to the left of the wall.                                            |
| **Right**  | Player runs into a wall from the right | Push the player back out to the right of the wall.                                           |

For walls, velocity is left alone because horizontal movement has no
velocity, it is set directly from input each frame.

### Why "airborne first"?

At the start of every collision pass, the player is **assumed to be in the
air**. Only if a collision from above is found does the player become
grounded again.

This one trick handles a tricky situation: walking off the edge of a
platform. Nothing is under the player any more, so nothing marks them as
grounded, so they become airborne and gravity pulls them down. No special
"am I still on the ground?" logic is needed.

### It all works together

Here is why standing still on the floor is stable:

1. Gravity pulls the player a tiny bit into the floor.
2. The collision check sees a small overlap from the **top**.
3. The player is placed back on top, velocity is reset to zero and the state
   is set to grounded.
4. Next frame, the same thing happens again.

The player looks perfectly still, but each frame is a small "fall and correct"
cycle. That is also why the player can jump right away: the state is
reliably grounded whenever they stand on something.

### Summary

| Concept        | One-line takeaway                                            |
| -------------- | ------------------------------------------------------------ |
| Velocity       | Vertical speed; positive is down.                            |
| Gravity        | Adds to velocity every frame.                                |
| Delta time     | Keeps behaviour independent of frame rate.                   |
| Jump           | A one-time upward velocity kick, gravity does the rest.      |
| AABB           | Everything is a rectangle; overlap on both axes means a hit. |
| Smallest depth | Decides which side was hit.                                  |
| Resolve        | Push the player out and fix velocity and state per side.     |
| Airborne first | Grounded is only true if something proves it this frame.     |
