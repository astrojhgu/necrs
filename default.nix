# default.nix
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "mpi_rust"; # Probably put a more meaningful name here
    buildInputs = [clang
    llvmPackages.libclang.lib
    pkg-config
    libtool
    lapack
    gcc
    blas
    autoconf
    automake
    gfortran
    (lib.getLib gfortran.cc)
    #necpp
    ];
    hardeningDisable = [ "all" ];
    #buildInputs = [gcc-unwrapped gcc-unwrapped.out gcc-unwrapped.lib];
    LIBCLANG_PATH = llvmPackages.libclang.lib+"/lib";
}
