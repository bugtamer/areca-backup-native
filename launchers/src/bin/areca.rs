use launchers::{ get_shell_arguments, run_areca, GUI_INITIAL_CLASS };



/// `areca` opens the Areca's GUI (Graphical User Interface).
fn main() {
    let shell_arguments = get_shell_arguments();
    let _ = run_areca(GUI_INITIAL_CLASS, shell_arguments);
}