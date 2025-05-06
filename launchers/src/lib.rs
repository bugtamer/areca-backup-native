use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::process::Command;
use walkdir::WalkDir;


pub const GUI_INITIAL_CLASS: &str = "com.application.areca.launcher.gui.Launcher";
pub const TUI_INITIAL_CLASS: &str = "com.application.areca.launcher.tui.Launcher";

pub const DEFAULT_MAXIMUM_JAVA_HEAP_MEMORY: &str = "-Xmx1024m";


pub fn run_areca<'a>(init_class: &str, shell_arguments: [String; 13]) -> std::io::Result<()> {
    let env = get_environmental_data_for_areca();

    let java_cmd = if env.is_windows {
        if init_class.eq(GUI_INITIAL_CLASS) {
            "javaw.exe"
        } else {
            "java.exe"
        }
    } else {
        "java"
    };
    let java_path = find_java_path();
    let command = match java_path.to_str() {
        Some(path) => OsString::from(String::from(path) + env.dir_separator.as_str() + java_cmd),
        None => OsString::from(java_cmd.to_string()),
    };

    let areca_arguments = build_arguments_for_areca_execution(init_class, shell_arguments, &env);

    let mut cmd = Command::new(command)
        .args(areca_arguments)
        .spawn()?;
    
    cmd.wait()?;

    Ok(())
}



pub fn get_shell_arguments<'a>() -> [String; 13] {
    let mut areca_arguments: [String; 13] = Default::default();
    
    let all_arguments: Vec<String> = env::args().collect();

    let num_areca_args = areca_arguments.len();
    for (i, arg) in all_arguments.iter().enumerate().take(num_areca_args) {
        areca_arguments[i] = arg.to_owned();
    }

    return areca_arguments;
}



fn build_arguments_for_areca_execution<'a>(init_class: &str, shell_arguments: [String; 13], env: &ArecaEnvironment) -> Vec<String> {
    let license_path = "";
    let i18n_path    = "translations";
    let config_path  = "config";
    let lib_path     = "lib";
    
    let classpath_items: [String; 14] = [
        // Areca paths
        env.program_dir.to_owned() /* + &env.dir_separator */ + &license_path,
        env.program_dir.to_owned() + &env.dir_separator + &config_path,
        env.program_dir.to_owned() + &env.dir_separator + &i18n_path,
        // Java dependencies
        env.program_dir.to_owned() + &env.dir_separator + &lib_path + &env.dir_separator + "areca.jar",
        env.program_dir.to_owned() + &env.dir_separator + &lib_path + &env.dir_separator + "mail.jar",
        env.program_dir.to_owned() + &env.dir_separator + &lib_path + &env.dir_separator + "activation.jar",
        env.program_dir.to_owned() + &env.dir_separator + &lib_path + &env.dir_separator + "commons-net-1.4.1.jar",
        env.program_dir.to_owned() + &env.dir_separator + &lib_path + &env.dir_separator + "commons-codec-1.4.jar",
        env.program_dir.to_owned() + &env.dir_separator + &lib_path + &env.dir_separator + "jakarta-oro-2.0.8.jar",
        env.program_dir.to_owned() + &env.dir_separator + &lib_path + &env.dir_separator + "jsch.jar",
        env.program_dir.to_owned() + &env.dir_separator + &lib_path + &env.dir_separator + "org.eclipse.core.commands_3.2.0.I20060605-1400.jar",
        env.program_dir.to_owned() + &env.dir_separator + &lib_path + &env.dir_separator + "org.eclipse.equinox.common_3.2.0.v20060603.jar",
        env.program_dir.to_owned() + &env.dir_separator + &lib_path + &env.dir_separator + "org.eclipse.jface_3.2.0.I20060605-1400.jar",
        env.program_dir.to_owned() + &env.dir_separator + &lib_path + &env.dir_separator + "swt.jar",
    ];

    let classpath = classpath_items.join(&env.path_separator);

    let library_path = if env.is_windows {
        String::from(env.java_path.as_os_str().to_str().unwrap()) + &env.dir_separator + &lib_path
    } else {
        lib_path.to_string() + ":/lib64:/lib:/usr/lib64:/usr/lib:/usr/lib64/java:/usr/lib/java:/usr/lib64/jni:/usr/lib/jni:/usr/share/java"
    };

    let java_library_path = String::from("-Djava.library.path=") + &library_path;
    let user_dir = String::from("-Duser.dir=") + &env.program_dir;

    let areca_execution_arguments: Vec<String> = Vec::<String>::from([
        java_heap_memory(init_class, "-Xmx"),
        "-Xms64m".to_string(),
        "-cp".to_string(),
        classpath,
        user_dir,
        java_library_path,
        "-Djava.system.class.loader=com.application.areca.impl.tools.ArecaClassLoader".to_string(),
        init_class.to_string(),
        shell_arguments[ 1].to_string(), // shell_arguments[0] --> program name
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
        shell_arguments[12].to_string(),
    ]);

    return areca_execution_arguments;
}



struct ArecaEnvironment {
    pub is_windows: bool,
    pub dir_separator: String,
    pub path_separator: String,
    pub program_dir: String,
    pub java_path: OsString,
}



fn get_environmental_data_for_areca() -> ArecaEnvironment {
    let dir_separator = get_system_directory_separator();
    let launcher_dir = get_launcher_dir();
    let areca_home = get_environment_variable("ARECA_HOME");

    ArecaEnvironment {
        is_windows: is_windows(),
        path_separator: get_system_path_separator(),
        java_path: find_java_path(),
        program_dir: if is_areca_present(&areca_home, &dir_separator) {
            String::clone(&areca_home)
        } else {
            String::clone(&launcher_dir)
        },
        dir_separator: dir_separator,
    }
}



fn is_windows() -> bool {
    return get_system_directory_separator().eq("\\");
}



fn is_areca_present(path: &String, dir_separator: &str) -> bool {
    let mut areca_jar = String::clone(&path);
    areca_jar.push_str(dir_separator);
    areca_jar.push_str("lib");
    areca_jar.push_str(dir_separator);
    areca_jar.push_str("areca.jar");
    return (path.len() > 0) && Path::new(&areca_jar).exists()
}



fn get_system_directory_separator() -> String {
    return String::from(MAIN_SEPARATOR);
}



fn get_system_path_separator() -> String {
    if is_windows() {
        return String::from(";");
    };
    return String::from(":");
}



fn get_launcher_dir() -> String {
    match env::current_exe() {
        Ok(exe_path) => {
            let exe_dir: PathBuf = exe_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."));
            String::from(exe_dir.to_str().unwrap())
        },
        Err(why) => panic!("Unknown launcher directory. {}", why),
    }
}



fn get_environment_variable(var: &str) -> String {
    return match env::var(var) {
        Ok(v) => v,
        Err(_e) => String::new()
    };
}



fn find_java_path() -> OsString {
    let path: OsString;
    if is_windows() {
        path = find_java_installation_on_windows();
    } else {
        path = find_java_installation_on_linux();
    }
    return parent_dir(path.as_os_str());
}



fn parent_dir(path: &OsStr) -> OsString {
    match Path::new(path).parent() {
        Some(parent) => OsString::from(parent.as_os_str()),
        None => OsString::from(path),
    }
}



/**
 * Help to keep compatibility with 'areca.exe' and 'areca_cl.exe' version 7.5
 * -Xms<size> set initial Java heap size
 * -Xmx<size> set maximum Java heap size
 */
fn java_heap_memory(init_class: &str, memory: &str) -> String {
    let bin_conf_file_name = if init_class.eq(GUI_INITIAL_CLASS) {
        String::from("areca.l4j.ini")
    } else {
        String::from("areca_cl.l4j.ini")
    };

    let env = get_environmental_data_for_areca();
    let script_dir = get_launcher_dir();
    let absolute_path = [script_dir, bin_conf_file_name].join(env.dir_separator.as_str());

    let path = Path::new(absolute_path.as_str());
    let display = path.display();

    match File::open(&path) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Err(_) => continue,
                    Ok(line) => {
                        if line.starts_with(memory) {
                            return String::from(line);
                        }
                    },
                };
            }
        },
        Err(_) => {
            let mut missing_file = match File::create(&path) {
                Ok(file) => file,
                Err(why) => panic!("Could not create {}: {}", display, why),
            };
            let comment = "# Launch4j runtime config";
            let new_line = "\n";
            let default_conf_file = [comment, DEFAULT_MAXIMUM_JAVA_HEAP_MEMORY].join(new_line);
            match missing_file.write_all(default_conf_file.as_bytes()) {
                Ok(_) => println!("Create missing {}", display),
                Err(why) => print!("Could not write to {}: {}", display, why),
            }
        },
    };
    return String::from(DEFAULT_MAXIMUM_JAVA_HEAP_MEMORY);
}



fn find_java_installation_on_windows() -> OsString {
    let default = OsString::from("java.exe");

    // Direct search in arbitrary local user directories.
    let bin_java = "\\bin\\java.exe";
    let embedded_jdk_in_cwd = OsString::from(get_launcher_dir() + "\\jdk" + bin_java);
    let embedded_jre_in_cwd = OsString::from(get_launcher_dir() + "\\jre" + bin_java);
    let embedded_jdk_in_areca_home = OsString::from(get_environment_variable("ARECA_HOME") + "\\jdk" + bin_java);
    let embedded_jre_in_areca_home = OsString::from(get_environment_variable("ARECA_HOME") + "\\jre" + bin_java);
    let java_home = OsString::from(get_environment_variable("JAVA_HOME"));
    let list_of_common_java_installations = [
        embedded_jdk_in_cwd,
        embedded_jre_in_cwd,
        embedded_jdk_in_areca_home,
        embedded_jre_in_areca_home,
        java_home,
    ];
    for path in list_of_common_java_installations {
        if Path::new(path.as_os_str()).exists() {
            return path.as_os_str().to_os_string();
        }
    }
    
    // Recursive searches in system installations
    let dir_list_to_search_recursive = [
        "C:\\Java",
        "C:\\Program Files (X86)\\Java",
        "C:\\Program Files\\Java",
    ];
    for path in dir_list_to_search_recursive {
        let common_installation_paths = find_file(path, "java.exe");
        let first_match = if common_installation_paths.len() == 0 {
            OsString::new()
        } else {
            match common_installation_paths.get(0) {
                Some(p) => p.as_os_str().to_os_string(),
                None => OsString::new(),
            }
        };
        if first_match.len() > 0 {
            return first_match;
        }
    }

    // Oracle classpaths. These directories only contain 3 executable files (java.exe | javaw.exe | javaws.exe)
    let oracle_javapath_locations = [
        OsString::from("C:\\Program Files (X86)\\Common Files\\Oracle\\Java\\javapath\\java.exe"),
        OsString::from("C:\\Program Files\\Common Files\\Oracle\\Java\\javapath\\java.exe")
    ]; 
    for path in oracle_javapath_locations {
        if Path::new(path.as_os_str()).exists() {
            return path;
        }
    }
    
    // Any other known Java installations
    let where_outcome = run_where_command_to_find_java();
    if where_outcome.len() > 0 {
        return match where_outcome.get(0) {
            Some(p) => p.to_os_string(),
            None => default
        };
    }

    // Java not found on Windows system
    return default;
}



fn find_file(parent_dir: &str, target_file: &str) -> Vec<PathBuf> {
    WalkDir::new(parent_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|folder_and_file_entries| folder_and_file_entries.file_type().is_file())
        .filter(|file_entries| file_entries.file_name() == target_file)
        .map(|file_entry| file_entry.path().to_path_buf())
        .collect::<Vec<PathBuf>>()
}



fn run_where_command_to_find_java() -> Vec<OsString> {
    match Command::new("where")
        .arg("java")
        .output() {
            Ok(output) => {
                if output.status.success() {
                    let mut matches = Vec::<OsString>::new();
                    let o = String::from_utf8(output.stdout).unwrap();
                    for p in o.split("\r\n") {
                        let path = OsString::from(p);
                        if path.is_empty() == false {
                            matches.push(path);
                        }
                    }
                    matches
                } else {
                    Vec::<OsString>::new()
                }
            },
            Err(_) => Vec::<OsString>::new(),
        }
}



fn find_java_installation_on_linux() -> OsString {
    let default = OsString::from("java");
    // not implemented yet
    return default;
}
