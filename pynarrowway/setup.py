from setuptools import setup, find_packages
import platform

if platform.system() == "Darwin":
    dll_name = "libnarrowway.dylib"
elif platform.system() == "Linux":
    dll_name = "libnarrowway.so"
elif platform.system() == "Windows":
    dll_name = "narrowway.dll"

setup(
    name="pynarrowway",
    version="0.1.0",
    author="31core",
    author_email="31core@tutanota.com",
    description="NarrowWay symmetric cipher.",

    url="https://github.com/31core/NarrowWay", 

    packages=find_packages(),
    include_package_data=True,
    package_data={"narrowway": [dll_name]}
)
