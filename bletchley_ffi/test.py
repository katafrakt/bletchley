from ctypes import cdll
import platform

extensions_by_system = {
    "Linux": "so",
    "Darwin": "dylib"
}
extension = extensions_by_system[platform.system()]
lib = cdll.LoadLibrary("target/debug/libbletchley_ffi." + extension)
lib.encrypt("../xyz.pub", "../why-rust.pdf")
