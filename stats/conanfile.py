from conans import ConanFile

class MyConanFile(ConanFile):
    requires = "cli11/1.9.1", "fmt/12.1.0", "prometheus-cpp/1.1.0"
    generators = "cmake", "BazelDeps", "BazelToolchain"
