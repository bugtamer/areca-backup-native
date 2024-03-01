use launchers::{
    get_areca_environment,
    get_areca_execution_arguments,
    get_shell_arguments, run_command_line
};



/**
 * areca_cl.exe
 * areca_cl.sh
 */
fn main() {
    let tui = "com.application.areca.launcher.tui.Launcher";
    let shell_arguments = get_shell_arguments();
    let env = get_areca_environment();
    let arguments = get_areca_execution_arguments(tui, shell_arguments, env);
    let command = "java";
    let _ = run_command_line(command, arguments);
}
