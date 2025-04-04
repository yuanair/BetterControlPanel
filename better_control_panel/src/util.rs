///
/// aplication auto install
///
pub mod auto_install;

///
/// std::process util
///
pub mod process;

///
/// command util
///
pub mod command;

#[cfg(feature = "rhai")]
pub fn registe_to_rhai(engine: &mut rhai::Engine) {
    auto_install::registe_to_rhai(engine);
}
