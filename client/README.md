# Rusted Wheel Client

  * Get the reflex-platform
  * `ln -s ____ rusted-client`
  * In `extendHaskellPackages`, add this:
    ```
    bizzleludio = self.callCabal2nix "bizzleludio" (fetchFromGitHub {
      owner = "TheBizzle";
      repo = "bizzlelude";
      rev = "c423ef473e95a7ba335397aa931fb0168426255d";
      sha256 = "02n7n8z2cxaj9m7zn5i2ldrhfzfvsxv8663zn9a3m5yp07fcy8n9";
    }) {};
    ```
  * Modify `packages.nix` to add an entry for `bizzleludio` in the "general packages" section
  * `./try-reflex`
  * `ghcjs --make ./rusted-client/src/*.hs && rm -rf rusted-client/target/* && mv rusted-client/src/*.js* rusted-client/target/`
