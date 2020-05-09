use crate::errors::*;

#[derive(Debug, Fail)]
pub enum CLIError {
    #[fail(display = "{}", _0)]
    BuildError(BuildError),

    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "{}", _0)]
    ChecksumFileError(ChecksumFileError),

    #[fail(display = "{}", _0)]
    InitError(InitError),

    #[fail(display = "{}", _0)]
    InputsDirectoryError(InputsDirectoryError),

    #[fail(display = "{}", _0)]
    MainFileError(MainFileError),

    #[fail(display = "{}", _0)]
    ManifestError(ManifestError),

    #[fail(display = "{}", _0)]
    NewError(NewError),

    #[fail(display = "{}", _0)]
    OutputsDirectoryError(OutputsDirectoryError),

    #[fail(display = "{}", _0)]
    ProofFileError(ProofFileError),

    #[fail(display = "{}", _0)]
    ProvingKeyFileError(ProvingKeyFileError),

    #[fail(display = "{}", _0)]
    RunError(RunError),

    #[fail(display = "{}", _0)]
    SourceDirectoryError(SourceDirectoryError),

    #[fail(display = "{}", _0)]
    VerificationKeyFileError(VerificationKeyFileError),
}

impl From<BuildError> for CLIError {
    fn from(error: BuildError) -> Self {
        CLIError::BuildError(error)
    }
}

impl From<ChecksumFileError> for CLIError {
    fn from(error: ChecksumFileError) -> Self {
        CLIError::ChecksumFileError(error)
    }
}

impl From<InitError> for CLIError {
    fn from(error: InitError) -> Self {
        CLIError::InitError(error)
    }
}

impl From<InputsDirectoryError> for CLIError {
    fn from(error: InputsDirectoryError) -> Self {
        CLIError::InputsDirectoryError(error)
    }
}

impl From<MainFileError> for CLIError {
    fn from(error: MainFileError) -> Self {
        CLIError::MainFileError(error)
    }
}

impl From<ManifestError> for CLIError {
    fn from(error: ManifestError) -> Self {
        CLIError::ManifestError(error)
    }
}

impl From<NewError> for CLIError {
    fn from(error: NewError) -> Self {
        CLIError::NewError(error)
    }
}

impl From<OutputsDirectoryError> for CLIError {
    fn from(error: OutputsDirectoryError) -> Self {
        CLIError::OutputsDirectoryError(error)
    }
}

impl From<ProofFileError> for CLIError {
    fn from(error: ProofFileError) -> Self {
        CLIError::ProofFileError(error)
    }
}

impl From<ProvingKeyFileError> for CLIError {
    fn from(error: ProvingKeyFileError) -> Self {
        CLIError::ProvingKeyFileError(error)
    }
}

impl From<RunError> for CLIError {
    fn from(error: RunError) -> Self {
        CLIError::RunError(error)
    }
}

impl From<SourceDirectoryError> for CLIError {
    fn from(error: SourceDirectoryError) -> Self {
        CLIError::SourceDirectoryError(error)
    }
}

impl From<VerificationKeyFileError> for CLIError {
    fn from(error: VerificationKeyFileError) -> Self {
        CLIError::VerificationKeyFileError(error)
    }
}

impl From<leo_compiler::errors::CompilerError> for CLIError {
    fn from(error: leo_compiler::errors::CompilerError) -> Self {
        CLIError::Crate("leo_compiler", format!("{}", error))
    }
}

impl From<serde_json::error::Error> for CLIError {
    fn from(error: serde_json::error::Error) -> Self {
        CLIError::Crate("serde_json", format!("{}", error))
    }
}

impl From<std::io::Error> for CLIError {
    fn from(error: std::io::Error) -> Self {
        CLIError::Crate("std::io", format!("{}", error))
    }
}