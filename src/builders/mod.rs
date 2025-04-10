/*!
 * # Gevulot Message Builders
 *
 * This module provides builder implementations for creating protocol messages used in the Gevulot network.
 * These builders help construct properly-formatted messages for communication with the Gevulot blockchain.
 *
 * ## Builder Categories
 *
 * - **Task Builders**: For creating, managing, and reporting on computational tasks
 * - **Pin Builders**: For data pinning and availability management 
 * - **Worker Builders**: For registering and managing compute providers
 * - **Workflow Builders**: For creating and managing multi-stage task workflows
 * - **Params Builders**: For updating module parameters
 * - **Admin Builders**: For administrative operations (sudo commands)
 *
 * Each builder follows a consistent pattern using the derive_builder crate, allowing for
 * fluid, type-safe construction of protocol messages with appropriate defaults.
 */

mod common;
mod task_builders;
mod pin_builders;
mod worker_builders;
mod admin_builders;
mod workflow_builders;
mod params_builders;

// Re-export all items from submodules to maintain backward compatibility
pub use common::*;
pub use task_builders::*;
pub use pin_builders::*;
pub use worker_builders::*;
pub use admin_builders::*;
pub use workflow_builders::*;
pub use params_builders::*; 