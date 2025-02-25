<div align="center">

# Fortress
### A tower defense game written in rust

<br><br>
[![Play](https://gist.githubusercontent.com/cxmeel/0dbc95191f239b631c3874f4ccf114e2/raw/play.svg)](https://tvdboom.itch.io/fortress)
<br><br>
</div>

<img src="https://github.com/tvdboom/fortress/blob/master/assets/scenery/s1.png?raw=true" alt="Early game">
<img src="https://github.com/tvdboom/fortress/blob/master/assets/scenery/s2.png?raw=true" alt="Population">
<img src="https://github.com/tvdboom/fortress/blob/master/assets/scenery/s3.png?raw=true" alt="Technologies">
<img src="https://github.com/tvdboom/fortress/blob/master/assets/scenery/s4.png?raw=true" alt="Armory">
<img src="https://github.com/tvdboom/fortress/blob/master/assets/scenery/s5.png?raw=true" alt="Late game">
<img src="https://github.com/tvdboom/fortress/blob/master/assets/scenery/s6.png?raw=true" alt="Constructions">

<br>

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

When a bug reaches the fortress (i.e., it exists the screen on the bottom side), it
fights with the existing population. If there are any soldiers, they will fight the
bug first.

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
which have increased strength when fighting bugs that enter the fortress. Note that when
you move a slider, the rest move as well (without changing the values). This is by design,
since the idle population diminishes and all sliders must be at the far right when it reaches
zero.

#### Constructions

Upgrade buildings to increase the amount of resources collected during the day. The player
can also (re)build the wall and a fence to stop the bugs from reaching the fortress.

#### Armory

Buy, upgrade and place/reorder weapons on the fortress' wall. The player can also buy
one-off explosives.

#### Technology

Research new technologies to improve the fortress' capabilities.

#### Expeditions

Send out expeditions. Expeditions cost resources and are away for a number of days.
If they return, they can bring back significant rewards. Requires the `charts`
technology.

<br>

### Key bindings

- `enter`: When in the day menu, enter the next night.
- `space`: Pause/unpause the game during the night.
- `e`: Open/close the enemy info panel for an overview of all enemies and
  their characteristics.

<br>

## 💡 Credits

 - Game design and implementation: [Mavs](https://github.com/tvdboom)
 - Weapon sprites: [3HST有限公司](https://steamcommunity.com/workshop/filedetails/?l=english&id=2915717417)
 - Bug sprites: [W_K_Studio](https://whiteknightstudios.itch.io/)
 - Icons: [flaticon.com](https://www.flaticon.com)
 - Buildings: [pngtree.com](https://pngtree.com/)