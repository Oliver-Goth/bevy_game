pub mod window;
pub mod camera;

pub use window::WindowConfigPlugin;
pub use camera::{spawn_camera, camera_follow_player, camera_zoom, MainCamera};