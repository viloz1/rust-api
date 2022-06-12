//! Handle everything related to the frontend of this crate.
//!
//! #Layout
//! 
//! There are two modules: guards and pages. The guards module handles the guards for the Rocket frontend,
//! and the pages module handle the different webpages. There are also several smaller modules, that
//! handle more specific tasks that can't be generalised. See the modules for more information.

pub mod github;
pub mod pages;
pub mod states;
pub mod auth;
pub mod guards;