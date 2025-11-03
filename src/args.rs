use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
pub enum Command {
    #[bpaf(command("tui"))]
    /// open the TUI
    Tui,

    #[bpaf(command("web"))]
    /// open the webui in a browser
    Web,
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Opts {
    #[bpaf(short, long)]
    pub verbose: bool,

    #[bpaf(external(command))]
    pub command: Command,
}
