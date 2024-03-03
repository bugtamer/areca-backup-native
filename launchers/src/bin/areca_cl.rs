use launchers::{
    get_environmental_data_for_areca,
    get_arguments_for_areca_execution,
    get_shell_arguments, run_command_line
};



/// `areca_cl` opens the Areca's CLI (Command Line Interface).
fn main() {
    let tui = "com.application.areca.launcher.tui.Launcher";
    let shell_arguments = get_shell_arguments();
    let env = get_environmental_data_for_areca();
    let arguments = get_arguments_for_areca_execution(tui, shell_arguments, env);
    let command = "java";
    let _ = run_command_line(command, arguments);
}
