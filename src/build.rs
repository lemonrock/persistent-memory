// This file is part of persistent-memory. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT. No part of persistent-memory, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of persistent-memory. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/persistent-memory/master/COPYRIGHT.


#![allow(non_upper_case_globals)]


extern crate cc;


use ::cc::Build;
use ::std::env::current_dir;
use ::std::env::var;
use ::std::env::var_os;
use ::std::ffi::OsString;
use ::std::fs::create_dir_all;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::process::Command;


const UnsupportedTargets: [&'static str; 8] =
[
	"bitrig", // Although BitRig fiddles are implemented below.
	"emscripten",
	"fuchsia",
	"msvc",
	"openbsd", // Although OpenBSD fiddles are implemented below.
	"redox",
	"rumprun", // Although NetBSD is supported below.
	"wasm32",
];

const JemallocNamespacePrefix: &'static str = "_persistent_memory_";

fn main()
{
	compile_specialized_jemalloc()
}

fn compile_specialized_jemalloc()
{
	if use_external_library_rather_than_compiling()
	{
		return;
	}
	
	let (host, target, out_folder_path, build_folder_path, jemalloc_source_folder_path) = host_target_and_folder_paths();
	
	create_build_folder_structure(&build_folder_path);
	
	autogen_sh(&jemalloc_source_folder_path);
	
	configure(&host, &target, &out_folder_path, &build_folder_path, &jemalloc_source_folder_path);
	
	make(&host, &build_folder_path);
	
	print_variables_to_cargo(&target, &out_folder_path, &build_folder_path)
}

fn use_external_library_rather_than_compiling() -> bool
{
	let jemalloc_library_path = match var_os("JEMALLOC_OVERRIDE")
	{
		None => return false,
		Some(jemalloc_library_path) => PathBuf::from(jemalloc_library_path),
	};
	
	if !jemalloc_library_path.is_file()
	{
		panic!("JEMALLOC_OVERRIDE points to a path '{:?}' which is not an extant file path", jemalloc_library_path)
	}
	
	println!("cargo:rustc-link-search=native={}", jemalloc_library_path.parent().expect("JEMALLOC_OVERRIDE points to a file system root").display());
	
	let name = jemalloc_library_path.file_name().expect("JEMALLOC_OVERRIDE points to . or ..").to_str().expect("JEMALLOC_OVERRIDE contains a non-UTF-8 character");
	let kind = if name.ends_with(".a")
	{
		"static"
	}
	else
	{
		"dylib"
	};
	
	let stem = jemalloc_library_path.file_stem().unwrap().to_str().unwrap();
	
	println!("cargo:rustc-link-lib={}={}", kind, &stem[3..]);
	
	true
}

fn host_target_and_folder_paths() -> (String, String, PathBuf, PathBuf, PathBuf)
{
	let host = mandatory_environment_variable("HOST");
	
	let target = is_target_unsupported(mandatory_environment_variable("TARGET"));
	
	let out_folder_path = PathBuf::from(mandatory_os_string_environment_variable("OUT_DIR"));
	
	let build_folder_path = out_folder_path.join("build");
	
	let jemalloc_source_folder_path =
	{
		let crate_source_folder_path = current_dir().unwrap();
		crate_source_folder_path.join("lib").join("jemalloc")
	};
	
	(host, target, out_folder_path, build_folder_path, jemalloc_source_folder_path)
}

fn create_build_folder_structure(build_folder_path: &Path)
{
	create_dir_all(build_folder_path).expect("Could not create build directory folder structure");
}

fn autogen_sh(jemalloc_source_folder_path: &Path)
{
	let mut autoconf_sh_command = Command::new("sh");
	autoconf_sh_command.arg(shell_program("autogen.sh", jemalloc_source_folder_path)).current_dir(jemalloc_source_folder_path);
	
	execute(&mut autoconf_sh_command);
}

fn configure(target: &str, host: &str, out_folder_path: &Path, build_folder_path: &Path, jemalloc_source_folder_path: &Path)
{
	let (compiler_path, c_flags) = compiler_path_and_c_flags();
	
	let mut configure_command = Command::new("sh");
	configure_command.arg(shell_program("configure", jemalloc_source_folder_path)).current_dir(build_folder_path);
	
	configure_command.env("CC", compiler_path);
	
	configure_command.env("CFLAGS", &c_flags);
	
	configure_command.env("CPPFLAGS", &c_flags);
	
	configure_command.arg(format!("--host={}", adjust_rust_target_triple_for_windows(target)));
	
	configure_command.arg(format!("--build={}", adjust_rust_target_triple_for_windows(host)));
	
	configure_command.arg(format!("--prefix={}", out_folder_path.display()));
	
	configure_command.arg("--without-export");
	
	// Causes compilation of jemalloc to fail due to an implicit function definition.
	// configure_command.arg(format!("--with-jemalloc-prefix={}", JemallocNamespacePrefix));
	
	configure_command.arg(format!("--with-private-namespace={}", JemallocNamespacePrefix));
	
	configure_command.arg("--disable-cxx");
	
	configure_command.arg("--disable-xmalloc");
	
	if environment_variable_exists("CARGO_FEATURE_DEBUG")
	{
		configure_command.arg("--enable-debug");
	}
	else
	{
		configure_command.arg("--disable-stats");
	}
	
	if environment_variable_exists("CARGO_FEATURE_PROFILING")
	{
		configure_command.arg("--enable-prof");
	}
	
	// jemalloc's configure doesn't detect this value automatically for this target.
	if target == "sparc64-unknown-linux-gnu"
	{
		configure_command.arg("--with-lg-quantum=4");
	}
	
	if contains(&target, &["android", "ios"])
	{
		configure_command.arg("--disable-tls");
	}
	
	execute(&mut configure_command);
}

fn make(host: &str, build_folder_path: &Path)
{
	let make = if contains(host, &["bitrig", "dragonfly", "freebsd", "netbsd", "openbsd"])
	{
		"gmake"
	}
	else
	{
		"make"
	};
	
	execute
	(
		Command::new(make)
		.current_dir(build_folder_path)
		.arg("install_lib_static")
		.arg("install_include")
		.arg("-j").arg(mandatory_environment_variable("NUM_JOBS"))
	);
}

fn print_variables_to_cargo(target: &str, out_folder_path: &Path, build_folder_path: &Path)
{
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-env-changed=JEMALLOC_OVERRIDE");
	
	println!("cargo:root={}", out_folder_path.display());
	
	println!("cargo:rustc-link-search=native={}/lib", build_folder_path.display());
	
	if target.contains("windows")
	{
		println!("cargo:rustc-link-lib=static-nobundle=jemalloc");
	}
	else
	{
		println!("cargo:rustc-link-lib=static-nobundle=jemalloc_pic");
	}
	
	// Specifically on android we need to also link to `libgcc`, as it uses intrinsics implemented in `libgcc`.
	// On all other POSIX platforms, we need to link to `libpthread`.
	if target.contains("android")
	{
		println!("cargo:rustc-link-lib=gcc");
	}
	else if !target.contains("windows")
	{
		println!("cargo:rustc-link-lib=pthread");
	}
}

fn is_target_unsupported(target: String) -> String
{
	for unsupported_target in &UnsupportedTargets
	{
		if target.contains(unsupported_target)
		{
			panic!("jemalloc does not support target: {}", target)
		}
	}
	target
}

fn execute(command: &mut Command)
{
	let output = command.output();
	
	match output
	{
		Err(error) => panic!("Failed to execute command: '{}'", error),
		Ok(output) =>
		{
			let exit_status = output.status;
			if !exit_status.success()
			{
				eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
				eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
				panic!("Command '{:?}' failed; expected success, got exit_status '{}'.", command, exit_status);
			}
		}
	}
}

fn adjust_rust_target_triple_for_windows(target: &str) -> String
{
	match target
	{
		"i686-pc-windows-msvc" => "i686-pc-win32".to_string(),
		"x86_64-pc-windows-msvc" => "x86_64-pc-win32".to_string(),
		"i686-pc-windows-gnu" => "i686-w64-mingw32".to_string(),
		"x86_64-pc-windows-gnu" => "x86_64-w64-mingw32".to_string(),
		other => other.to_string(),
	}
}

fn mandatory_environment_variable(environment_variable_name: &str) -> String
{
	var(environment_variable_name).expect(&format!("Environment variable '{}' was not set", environment_variable_name))
}

fn mandatory_os_string_environment_variable(environment_variable_name: &str) -> OsString
{
	var_os(environment_variable_name).expect(&format!("Environment variable '{}' was not set", environment_variable_name))
}

fn environment_variable_exists(environment_variable_name: &str) -> bool
{
	var_os(environment_variable_name).is_some()
}

fn contains(string: &str, contains_to_check: &[&'static str]) -> bool
{
	for contains in contains_to_check
	{
		if string.contains(contains)
		{
			return true
		}
	}
	false
}

fn shell_program(shell_program_name: &str, jemalloc_source_folder_path: &Path) -> String
{
	let program = jemalloc_source_folder_path.join(shell_program_name);
	program.to_str().unwrap().replace("C:\\", "/c/").replace("\\", "/").to_string()
}

fn compiler_path_and_c_flags() -> (PathBuf, String)
{
	let build = Build::new();
	
	// This commands dumps environment variables to standard out for some reason.
	let compiler = build.get_compiler();
	
	let compiler_path = compiler.path().to_owned();
	let c_flags = compiler.args().iter().map(|c_flag| c_flag.to_str().expect("C Flag contained non-UTF-8 character")).collect::<Vec<_>>().join(" ");
	(compiler_path, c_flags)
}
