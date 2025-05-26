mod lens_commands;

use lens_commands::LensCommand;

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Lens(LensCommand),
}
