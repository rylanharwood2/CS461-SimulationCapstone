
# Software Development Process
Team: Team 8 Engineering Simulations 2
Contributors: Taye Rieks, Joshua Lim, Nick Wisler, Rylan Harwood
Course: CS 461

**Contents**
Scope
Principles
Process
Roles
Tooling
Tech Stack
Documentation Guidelines
Coding Standards
Definition of Done (DoD)
Release Cycle
Phases (timeline / milestones)
Environments

# Problem Statement
Whether someone is designing an aircraft or training to fly one, it can be difficult to understand how the craft will perform in the real world without actual experience. This experience usually requires testing in the real world, which can be both costly and time consuming.


#  Scope
Our goal for this project is to create a virtual world with terrain traversable by a flyable aircraft with proper physics that runs efficiently. The flyable aircraft, and environment are modifiable. The simulator will be equipped with simulation saving, user interface, sound, graphics, and modifiable aircrafts. 


#  Major Deliverables
* Procedural Terrain Generation based on real world data
* Customizable aircraft, terrain, environment
* 3D Flight Physics
* Simulation Replay

# Principles
* Responsive within 24 hours in Discord
* Responsive within 6 hours for critical messages
* We use our discord channel as our primary source of communication
* The backlog should always be updated on Monday of every other week.
* We should have checkups on the progress of our sprint every other day. This could be discord messages or scheduled voice meetings.
* A work item needs a technical description, dependencies, estimate, and acceptance criteria before anyone can work on it.
* Must create a “dev-name” branch when working on a “feature-name” branch or the main branch.
* All merges must be from pull requests, never a forced merge.
* PRs are reviewed by at least two members, one of which must have some knowledge on the code and context.
* PRs must align with DoD.
* Work items measured by the modified Fibonacci series are 0, 1, 2, 3, 5, 8, 13, 20, 40, 100. These are not necessarily hours but the estimated effort needed by a person.
* Work items must have a clear description and acceptance criteria
* Each work item has a unique issue attached
* Weekly Discord call meetings

# Process
* Backlog updates and planning every week
* Board to keep track of who is implementing what on a given week
* All group members review completed code in a discord call before merging, if some members are unavailable one will suffice but the others should review the code whenever possible. 
* Meeting with our stakeholder at the end of every week showing off the features we have implemented.

# Roles
**Management Roles:**
Scrum master - weekly rotation - Manage meetings, backlogs, and lead any meetings during that week. Also in charge of submitting any group projects that week. 


**Technical Subteams:**
**1. Aircraft (flight simulation): Taye Rieks, Nicholas Wisler**

> **Rigidbody Physics:** Rigid Body kinematics, Rigidbody collision

**2. Terrain (environment): Josh Lim, Rylan Harwood**

> **If Procedural:** Mesh Generation, Heightmap streaming
> 
> **If Static:** Mesh Generation

# Tooling
**Version Control**
Github
**Project Management**
Github Issues and Projects
**Documentation**
Github Wiki
**Test Framework**
Bevy/Rust
**Linting and Formatting**
Prettier
**CI/CD**
Github Actions
**IDE**
Visual Studio Code
**Graphic Design**
Figma, Photoshop, Illustrator
	

# Tech Stack
**Programming Language**
Rust

**Game Engine: Bevy Game**
Engine

# Documentation Guidelines
* Comment all function headers with parameters and description
   * Give summary of function
   * Summary of each parameter
   * Summary of return values
* Periodic functionality comments in complex functions
* Readme
   * Summary of major code components
   * Summary of code architecture
   * Environment setup instructions
   
# Coding Standards
* (DRY principle) Create functions for any repeat code
* (Lengthy functions) Separate the code within functions, labeling each section with comments
* Follow Rust Coding Convention: https://rust-lang.github.io/api-guidelines/naming.html

# Definition of Done (DoD)
* Criteria of the task have been met
* Changes have been merged
* Documentation has been made/updated
* Any bugs have been addressed or a plan has been made to solve them
* Testing has been completed to ensure the stakeholder demo will go well
* Changes are thoroughly playtested in the simulation to manual check for bugs
* Changes pass all unit and/or integration tests we have implemented
* Pull request was reviewed by at least one member
* No regressions

# Release Cycle
* Every merge with main will be indicated by updating the version
* Every ~3 months we should have a major version update
* Deploy to production every release
* Use semantic versioning Major.Minor.Patch
   * Patch is incremented for bug fixes
   * Minor is incremented for new features
   * Major is incremented for large API changes or major feature changes

# Phases (timeline / milestones):
### Prototype:
* Basic Movable Camera: December 8, 2023
* Basic Terrain: December 8, 2023
* Rigidbody Physics: TBD
* Flight Physics and Airplane Control: TBD

### Minimum viable product:
* Basic Movable Camera: December 8, 2023
* Basic Terrain: December 8, 2023
* Rigidbody Physics: TBD
* Flight Physics and Airplane Control: TBD
* Customizable Airplane parameters: TBD
* User Interface (Menu): TBD

### Final Product:
* Movable Camera: December 8, 2023
* Basic Terrain: December 8, 2023
* Rigidbody Physics: TBD
* Flight Physics and Airplane Control: TBD
* Customizable Airplane parameters: TBD
* User Interface (Menu): TBD
* Procedural Terrain: TBD
* Final Airplane model, sound, etc… : TBD
* Heightmap Streaming for Terrain: TBD
* Simulation Replay: TBD

### Stretch Goals:
* Multiple airplane models
* Volumetric Clouds
* More realistic environment textures
* 3D city/building environments

# Environments
**Production**
Infrastructure: Bevy Game Engine
Deployment: Release
What is it for? Finished product ready for release

**Dev**
Infrastructure: Local (Windows)
Deployment: Commit
What is it for? Development and testing
