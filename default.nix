{ pkgs ? import <nixpkgs> { } }:

with pkgs; rustPlatform.buildRustPackage rec {
  pname = "coq-imports";
  name = "${pname}-${version}";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "0sjjj9z1dhilhpc8pq4154czrb79z9cm044jvn75kxcjv6v5l2m5";

  meta = with stdenv.lib; {
    description = "TODO";
    homepage = "https://github.com/siddharthist/${pname}";
    license = licenses.mpl2;
    maintainers = [ maintainers.siddharthist ];
    platforms = platforms.unix;
  };
}
