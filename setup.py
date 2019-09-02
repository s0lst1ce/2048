import sys
from cx_Freeze import setup, Executable

# Dependencies are automatically detected, but it might need fine tuning.
build_exe_options = {
    "packages": [
        "os", "pygame", "os", "random"
    ],
    "excludes": ["tkinter"],
}
# GUI applications require a different base on Windows (the default is for a
# console application).
base = None
if sys.platform.startswith("win"):
    base = "Win32GUI"

options = {
    "build_exe": build_exe_options
}
executables = [Executable("main.py", base=base, targetName="2048")]
#print("options, executables, base", options, executables, base)

setup(
    name="2048",
    license='GPL3',
    author='s0lst1ce',
    author_email='thithib.cohergne@gmail.com',
    maintainer='s0lst1ce',
    maintainer_email='thithib.cohergne@gmail.com',
    description='A 2048 clone with Python & Pygame',
    options=options,
    executables=executables,
    #packages=find_packages(),
    url='https://github.com/s0lst1ce/2048',
    version='0.2',
)