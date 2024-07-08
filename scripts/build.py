import os
import re
import sys
import shutil
from datetime import datetime
import pathlib
import platform

PROJECT_PATH = pathlib.Path(__file__).parent.parent.resolve()

UNITY_PATH = PROJECT_PATH / "unity"
UNITY_PATH.mkdir(exist_ok=True)

CLAM_FFI_PATH = PROJECT_PATH / "clam_ffi" / "clam_ffi"
assert CLAM_FFI_PATH.exists(), "clam_ffi not found at " + str(CLAM_FFI_PATH)


def replace_word_in_file(filename, new_libname):
    with open(filename,'r') as file:
        data = file.readlines()
        found = False
        for (i,line) in enumerate(data):
            if "const string __DllName" in line:
                data[i] = "\tpublic const string __DllName = " + '"' + new_libname + '"' + ";" + "\n"
                found = True
            # else:
                # data[i] = data[i].replace("unsafe ","")
        if not found:
            print("error libname not updated")
            
        return data


def update_unity(new_libname):

    filename = UNITY_PATH / "Assets" / "Scripts" / "FFI" / "NativeDLLName.cs"
    data = replace_word_in_file(filename, new_libname)
    with open(filename, 'w') as file:
        file.writelines(data)

    return


def get_platform() -> str:
    pf = platform.system().lower()
    if pf == "windows":
        return "windows"
    elif pf == "darwin":
        return "macos"
    elif pf == "linux":
        return "linux"
    else:
        print(f"Unknown platform: {pf}")
        sys.exit(1)

def get_lib_ext() -> str:
    platform = get_platform()
    if platform == "windows":
        return ".dll"
    elif platform == "macos":
        return ".dylib"
    elif platform == "linux":
        return ".so"
    else:
        print("Unknown platform: " + platform)
        sys.exit(1)


def copy_lib(libname, new_libname, is_release):
    lib_path = UNITY_PATH / "Assets" / "Plugins" / "lib"
    print(lib_path)

    lib_path.mkdir(exist_ok=True, parents=True)

    build_mode = "release" if is_release else "debug"

    ext = get_lib_ext()

    new_libname += ext
    libname += ext
    src = CLAM_FFI_PATH / "target" / build_mode / libname
    dst = lib_path / new_libname

    print("copying ", new_libname, " from ", src," to ", dst)
    shutil.copyfile(src, dst)
    return


def build_lib(build_is_release):
    os.chdir(CLAM_FFI_PATH)
    command = "cargo build"
    if build_is_release:
        command += " --release"
    os.system(command)
    os.chdir(PROJECT_PATH)
    return


def should_build_release():
    if len(sys.argv) != 2:
        return False

    build_mode = sys.argv[1].lower()
    return build_mode == "--release"


def get_lib_prefix() -> str:
    platform = get_platform()
    if platform == "windows":
        return ""
    elif platform == "macos":
        return "lib"
    elif platform == "linux":
        return "lib"
    else:
        print("Unknown platform: " + platform)
        sys.exit(1)


def main():

    timestamp = f'{datetime.now():%Y-%m-%d%H-%M-%S%z}'

    prefix = get_lib_prefix()
    libname = prefix + "clam_ffi"

    new_libname = libname + "_" + timestamp
    is_release = should_build_release()
    build_lib(is_release)
    copy_lib(libname, new_libname, is_release)
    # gen_cs_binding()
    update_unity(new_libname)


main()