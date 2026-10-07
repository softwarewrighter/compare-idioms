/// A language column: one language as run by one runtime.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Column {
    Xetal,
    GnuApl,
    J,
    Bqn,
    /// ngn/k
    K,
    /// Kona
    K3,
    /// k edu on BareMetal-OS, under QEMU
    Kbm,
    Uiua,
}

impl Column {
    pub const ALL: [Column; 8] = [
        Column::Xetal,
        Column::GnuApl,
        Column::J,
        Column::Bqn,
        Column::K,
        Column::K3,
        Column::Kbm,
        Column::Uiua,
    ];

    /// The column's name in reports, test names and the command line.
    pub fn name(self) -> &'static str {
        match self {
            Column::Xetal => "xetal",
            Column::GnuApl => "gnu-apl",
            Column::J => "j",
            Column::Bqn => "bqn",
            Column::K => "k",
            Column::K3 => "k3",
            Column::Kbm => "kbm",
            Column::Uiua => "uiua",
        }
    }

    pub fn from_name(name: &str) -> Option<Column> {
        Column::ALL.into_iter().find(|c| c.name() == name)
    }

    /// The runtime's name in `scripts/runtimes.sh`.
    pub fn runtime(self) -> &'static str {
        match self {
            Column::Bqn => "cbqn",
            Column::K => "ngn-k",
            Column::K3 => "kona",
            other => other.name(),
        }
    }

    /// The column of X_eTaL's data the expression comes from. Kona and kbm
    /// have no cells of their own yet: they try the ngn/k cell unchanged.
    pub fn source_key(self) -> &'static str {
        match self {
            Column::GnuApl => "apl2",
            Column::K | Column::K3 | Column::Kbm => "k",
            other => other.name(),
        }
    }

    /// Indices count from 0 (X_eTaL and APL count from 1).
    pub fn zero_origin(self) -> bool {
        !matches!(self, Column::Xetal | Column::GnuApl)
    }

    /// The program prints its own `kind|shape|items` line; the others are
    /// read from their native display.
    pub fn self_serializes(self) -> bool {
        !matches!(self, Column::Xetal | Column::Kbm | Column::Uiua)
    }
}
