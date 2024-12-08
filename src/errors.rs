use miette::Diagnostic;
use thiserror::Error;
use vex_v5_serial::packets::cdc2::Cdc2Ack;

#[derive(Error, Diagnostic, Debug)]
pub enum CliError {
    #[error(transparent)]
    #[diagnostic(code(cargo_v5::io_error))]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(code(cargo_v5::serial_error))]
    SerialError(#[from] vex_v5_serial::connection::serial::SerialError),

    #[error(transparent)]
    #[diagnostic(code(cargo_v5::cdc2_nack))]
    Nack(#[from] Cdc2Ack),

    // TODO: Add source spans.
    #[error("Incorrect type for field `{field}` (expected {expected}, found {found}).")]
    #[diagnostic(
        code(cargo_v5::bad_field_type),
        help("The `{field}` field should be of type {expected}.")
    )]
    BadFieldType {
        /// Field name
        field: String,

        /// Expected type
        expected: String,

        /// Actual type
        found: String,
    },

    // TODO: Add optional source spans.
    #[error("The provided slot should be in the range [1, 8] inclusive.")]
    #[diagnostic(
        code(cargo_v5::slot_out_of_range),
        help("The V5 brain only has eight program slots. Adjust the `slot` field or argument to be a number from 1-8."),
    )]
    SlotOutOfRange,

    // TODO: Add source spans.
    #[error("{0} is not a valid icon.")]
    #[diagnostic(
        code(cargo_v5::invalid_icon),
        help("See `cargo v5 upload --help` for a list of valid icon identifiers.")
    )]
    InvalidIcon(String),

    #[error("No slot number was provided.")]
    #[diagnostic(
        code(cargo_v5::no_slot),
        help("A slot number is required to upload programs. Try passing in a slot using the `--slot` argument, or setting the `package.v5.metadata.slot` field in your Cargo.toml.")
    )]
    NoSlot,

    #[error("ELF build artifact not found. Is this a binary crate?")]
    #[diagnostic(
        code(cargo_v5::no_artifact),
        help("`cargo v5 build` should generate an ELF file in your project's `target` folder unless this is a library crate. You can explicitly supply a file to upload with the `--file` (`-f`) argument.")
    )]
    NoArtifact,

    #[error("No V5 devices found.")]
    #[diagnostic(
        code(cargo_v5::no_device),
        help("Ensure that a V5 brain or controller is plugged in and powered on with a stable USB connection, then try again.")
    )]
    NoDevice,

    #[error("Output ELF file could not be parsed.")]
    #[diagnostic(code(cargo_v5::elf_parse_error))]
    ElfParseError(#[from] object::Error),

    #[error("Controller never reconnected after switching radio channels.")]
    #[diagnostic(
        code(cargo_v5::radio_channel_timeout),
        help("Try running `cargo v5 upload` again. If the problem persists, power cycle your controller and Brain.")
    )]
    RadioChannelTimeout,

    #[cfg(feature = "field-control")]
    #[error("Attempted to change the match mode over a direct Brain connection.")]
    #[diagnostic(
        code(cargo_v5::brain_connection_set_match_mode),
        help("Connect to the Brain over a controller connection to change the match mode.")
    )]
    BrainConnectionSetMatchMode,

    #[error("Attempted to create a new project at {0}, but the directory is not empty.")]
    #[diagnostic(
        code(cargo_v5::project_dir_full),
        help("Try creating the project in a different directory or with a different name.")
    )]
    ProjectDirFull(String),
    
}
