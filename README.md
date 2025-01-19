<div align="center">

# Fortress
### A tower defense game written in rust

<br><br>
[![Play](https://gist.githubusercontent.com/cxmeel/0dbc95191f239b631c3874f4ccf114e2/raw/play.svg)](https://tvdboom.github.io/fortress/)
<br><br>

<p align="center">
	<img src="https://github.com/tvdboom/fortress/blob/master/assets/scenery/s1.png?raw=true" title="Fortress" height="540" width="960"/>
</p>

</div>

## 📜 Introduction

The world has been conquered by insects. Together with a handful of survivors, 
you have built a fortress to defend yourself from their ferocious attacks.
Every night, an ever-increasing swarm attacks the fortress. Kill them before
they enter the fortress and finish the remaining population!

During the day, you can collect resources, research technologies, send expeditions,
and most importantly, upgrade your weapon arsenal to prepare yourself for the
following night. During the attack, you can choose how/when to use the weapons
you have at your disposal. But be careful, everything has a cost! Manage your
resources wisely or you won't be able to stop the bugs tomorrow...

<br>

## 🎮 Gameplay

The game consists of two stages (night and day) that alternate. The game starts
on the night of the first cycle.

### Night

During the night, the bugs attack the fortress. They come from the north and move
in straight lines towards the fortress at the bottom of the screen. There are two
defense structures that stop the bugs from directly reaching the fortress: the wall
and the fence. The player starts with a wall, and the fence can be built later.
When a bug reaches these structures, their movement stops, and they start attacking
the structure. If the structure is destroyed, the bugs can move southwards again.
Some bugs at later levels can fly over the wall and fence!

When a bug reaches the fortress (i.e., it dissapears from the screen), it fights with
the existing population. If there are any soldiers, they will fight the bug first.

On the fortress' wall, you can place weapons that shoot at the incoming enemies.
The weapons panel, on the right-hand side, shows the options you have available.
Use these settings during the night to minimalize the number of resources spent
while preventing the bugs from entering the fortress.

If all the population within the fortress is killed, the game is over.

<br>

### Day

During the day, the player can manage the fortress and prepare for the next night.
When the day starts, a random number of survivors joins the fortress, and new
resources are collected. The following operations can be performed:

#### Population

Assign the available population to one of the resources: bullets, gasoline, materials
or technology. The next day, the number of resources you receive will be proportional
to the number of people assigned to that resource. It's also possible to assign soldiers,
which have increased strength when fighting bugs that enter the fortress.

#### Constructions

Upgrade buildings to increase the amount of resources collected during the day. The player
can also (re)build the wall and a fence to stop the bugs from reaching the fortress.

#### Armory

Buy, improve and position weapons on the fortress' wall. The weapons have different
characteristics, such as range, damage, and fire rate. The player can also research
new weapons and upgrade existing ones.

#### Technology

Research new technologies to improve the fortress' capabilities.

#### Expeditions

Send out expeditions. Expeditions cost resources and are away for a number of days.
If they return, they can bring back significant rewards. Requires the `charts`
technology.

<br>

### Key bindings

- `enter`: Go to next stage (night -> day, day -> night).
- `space`: Pause/unpause the game.
- `e`: Open/close the enemy info panel for an overview of all enemies and
  their characteristics.
