mod menu;
mod sqlite;

use menu::MenuPlugin;
use sqlite::DatabasePlugin;

pub mod prelude {
    pub use bevy::prelude::*;

    pub use crate::sqlite::*;

    #[cfg(feature = "debug")]
    pub use bevy::dev_tools::states::log_transitions;
}

use prelude::*;

#[macro_export]
macro_rules! embed_asset {
    ($app: ident, $path: expr) => {{
        let embedded = $app
            .world_mut()
            .resource_mut::<::bevy::asset::io::embedded::EmbeddedAssetRegistry>();

        embedded.insert_asset(
            concat!(env!("CARGO_MANIFEST_DIR"), "/", $path).into(),
            ::std::path::Path::new($path),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path)),
        );
    }};
}
fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "TCSS 445 Project".into(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
    ); // fallback to nearest sampling

    // Local Plugins
    app.add_plugins(DatabasePlugin).add_plugins(MenuPlugin);

    app.run();
}
