from conans import ConanFile

class MyConanFile(ConanFile):
    requires = "cli11/2.6.0", "fmt/12.1.0", "prometheus-cpp/1.3.0"
    generators = "cmake", "BazelDeps", "BazelToolchain"
