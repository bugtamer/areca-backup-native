use std::env;
use std::io::{BufRead, BufReader};
use std::path::MAIN_SEPARATOR;
use std::process::{Command, Stdio};



pub fn run_command_line<'a>(command: &str, arguments: [String; 20]) -> std::io::Result<()> {
    let mut cmd = Command::new(command)
        .args(arguments)
        .stdout(Stdio::piped())
        .spawn()?;
    
    if let Some(stdout) = cmd.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    cmd.wait()?;

    Ok(())
}



pub fn get_areca_execution_arguments<'a>(init_class: &str, shell_arguments: [String; 12], env: ArecaEnvironment) -> [String; 20] {
    let program_dir = if env.areca_home.len() > 0 {
        env.areca_home
    } else {
        env.current_working_dir
    };

    let license_path = "";
    let i18n_path    = "translations";
    let config_path  = "config";
    let lib_path     = "lib";
    
    let separator_dir = if env.is_windows {
        "\\"
    } else {
        "/"
    };

    let classpath_items: [String; 14] = [
        // Areca paths
        program_dir.to_owned() + &separator_dir + &license_path,
        program_dir.to_owned() + &separator_dir + &config_path,
        program_dir.to_owned() + &separator_dir + &i18n_path,
        // Java dependencies
        program_dir.to_owned() + &separator_dir + &lib_path + &separator_dir + "areca.jar",
        program_dir.to_owned() + &separator_dir + &lib_path + &separator_dir + "mail.jar",
        program_dir.to_owned() + &separator_dir + &lib_path + &separator_dir + "activation.jar",
        program_dir.to_owned() + &separator_dir + &lib_path + &separator_dir + "commons-net-1.4.1.jar",
        program_dir.to_owned() + &separator_dir + &lib_path + &separator_dir + "commons-codec-1.4.jar",
        program_dir.to_owned() + &separator_dir + &lib_path + &separator_dir + "jakarta-oro-2.0.8.jar",
        program_dir.to_owned() + &separator_dir + &lib_path + &separator_dir + "jsch.jar",
        program_dir.to_owned() + &separator_dir + &lib_path + &separator_dir + "org.eclipse.core.commands_3.2.0.I20060605-1400.jar",
        program_dir.to_owned() + &separator_dir + &lib_path + &separator_dir + "org.eclipse.equinox.common_3.2.0.v20060603.jar",
        program_dir.to_owned() + &separator_dir + &lib_path + &separator_dir + "org.eclipse.jface_3.2.0.I20060605-1400.jar",
        program_dir.to_owned() + &separator_dir + &lib_path + &separator_dir + "swt.jar",
    ];

    let separator_path: &str = if env.is_windows {
        ";"
    } else {
        ":"
    };

    let classpath = classpath_items.join(separator_path);

    let library_path = if env.is_windows {
        env.java_home + &separator_dir + &lib_path
    } else {
        lib_path.to_string() + ":/lib64:/lib:/usr/lib64:/usr/lib:/usr/lib64/java:/usr/lib/java:/usr/lib64/jni:/usr/lib/jni:/usr/share/java"
    };

    let java_library_path = String::from("-Djava.library.path=") + &library_path;
    let user_dir = String::from("-Duser.dir=") + &program_dir;

    let areca_execution_arguments: [String; 20] = [
        "-Xmx1024m".to_string(),
        "-Xms64m".to_string(),
        "-cp".to_string(),
        classpath,
        user_dir,
        java_library_path,
        "-Djava.system.class.loader=com.application.areca.impl.tools.ArecaClassLoader".to_string(),
        init_class.to_string(),
        shell_arguments[ 0].to_string(),
        shell_arguments[ 1].to_string(),
        shell_arguments[ 2].to_string(),
        shell_arguments[ 3].to_string(),
        shell_arguments[ 4].to_string(),
        shell_arguments[ 5].to_string(),
        shell_arguments[ 6].to_string(),
        shell_arguments[ 7].to_string(),
        shell_arguments[ 8].to_string(),
        shell_arguments[ 9].to_string(),
        shell_arguments[10].to_string(),
        shell_arguments[11].to_string(),
    ];

    return areca_execution_arguments;
}



pub fn get_shell_arguments<'a>() -> [String; 12] {
    let mut areca_arguments: [String; 12] = [
        String::new(), String::new(), String::new(),
        String::new(), String::new(), String::new(),
        String::new(), String::new(), String::new(),
        String::new(), String::new(), String::new(),
    ];
    
    let all_arguments: Vec<String> = env::args().collect();

    let num_areca_args = areca_arguments.len();
    for (i, arg) in all_arguments.iter().enumerate().take(num_areca_args) {
        areca_arguments[i] = arg.to_owned();
    }

    return areca_arguments;
}



pub fn get_areca_environment() -> ArecaEnvironment {
    let is_windows = MAIN_SEPARATOR == '\\';

    let current_working_dir: String = match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_e) => String::new()
    };

    let areca_home = match env::var("ARECA_HOME") {
        Ok(v) => v,
        Err(_e) => String::new()
    };

    let java_home = match env::var("JAVA_HOME") {
        Ok(v) => v,
        Err(_e) => String::new()
    };

    return ArecaEnvironment {
        is_windows,
        areca_home,
        java_home,
        current_working_dir,
    };
}



pub struct ArecaEnvironment {
    is_windows: bool,
    areca_home: String,
    java_home: String,
    current_working_dir: String,
}
