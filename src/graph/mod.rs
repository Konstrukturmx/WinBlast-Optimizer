//! High-performance Graph Algorithms Engine for WinBlast0 1.0.
//!
//! Provides Dijkstra's shortest-path execution scheduler, A* goal-driven optimization planner,
//! and the Raven multi-criteria decision and conflict resolution engine.

pub mod astar;
pub mod dijkstra;
pub mod raven;
pub mod topological_scheduler;

pub use astar::{AStarPlanner, OptimizationGoal, OptimizationPlan};
pub use dijkstra::{DijkstraScheduler, ExecutionOrder, ExecutionStep};
pub use raven::{ConflictResolution, RavenEngine, TweakCriteria};
pub use topological_scheduler::{schedule_optimizations_topological, MAX_NODES};
