use launchers::{ get_shell_arguments, run_areca, TUI_INITIAL_CLASS };



/// `areca_cl` opens the Areca's CLI (Command Line Interface).
fn main() {
    let shell_arguments = get_shell_arguments();
    let _ = run_areca(TUI_INITIAL_CLASS, shell_arguments);
}
