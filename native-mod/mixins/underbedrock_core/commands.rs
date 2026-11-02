// The mod's commands in the server's tree: one usage each, merged into one
// command by the word they share.
fn underbedrock_core_commands(commands: &mut Vec<crate::game::native_hooks::NativeCommand>) {
    use crate::game::native_hooks::NativeCommand;
    for usage in [
        "abyss",
        "abyss give",
        "abyss depth",
        "abyss dig",
        "abyss drop",
    ] {
        commands.push(NativeCommand::new(usage));
    }
}
