use launchers::{
    get_areca_environment,
    get_areca_execution_arguments,
    get_shell_arguments, run_command_line
};



/**
 * areca.exe
 * areca.sh
 */
fn main() {
    let gui = "com.application.areca.launcher.gui.Launcher";
    let shell_arguments = get_shell_arguments();
    let env = get_areca_environment();
    let arguments = get_areca_execution_arguments(gui, shell_arguments, env);
    let command = "java";
    let _ = run_command_line(command, arguments);
}