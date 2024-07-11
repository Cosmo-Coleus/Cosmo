/// Gestion de l'ensemble de **Cosmo**
pub mod core;
/// Gestion de l'editeur **Cosmo**
mod editor;



pub mod commands;
mod editor_modes;
mod editor_view;
mod utils;

/// Liste les différents modes d'interaction de l'IDE.
#[derive(PartialEq, Eq, Debug)]
pub enum InputMode {
    Normal,
    Insert,
    Command,
    Exit,
}
