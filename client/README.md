# Client

# Installation :

Installer rust :

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Téléchargez les target :

``` sh
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android
rustup target add aarch64-linux-android
```

Ce projet a été testé avec Rust 1.74.0. Toute version antérieur est susceptible de ne pas fonctionner.

Il est également nécessaire d'avoir java d'installer. 
> Note : cela fonctionne avec une version de java >= 17, les versions antérieur n'ont pas été testé

# Installation du ndk et sdk

## Installation automatique (Linux uniquement)
Allez dans le dossier `client/` (Dossier où ce trouve ce README).
Puis effectuez :

``` sh
make init
```
Enfin exporter la variable $NDK_HOME$ qui pointe vers `android/ndk` : 

``` sh
export NDK_HOME=$PWD/android/ndk
```

> Note : Cette installation effectue également un patch spécifique à ce projet au ndk.

## Installation manuelle (explication pour linux, adaptable pour MacOs)
Étapes a effectuer dans le dossier `client/` (Dossier où ce trouve ce README).

- Téléchargez le `sdk` android et le mettre dans le dossier `android/sdk`
- Téléchargez le `ndk` android (version r26b) et le mettre dans le dossier `android/ndk` (tel que l'on est un dossier `android/ndk/toolchains`)
> Note : les versions r21e et avant ne fonctionnes pas.
- Téléchargez le sdk :

``` sh
cd android/sdk/cmdline-tools/bin
./sdkmanager --update --sdk_root=../..
./sdkmanager --install "build-tools;29.0.3" --sdk_root=../..
./sdkmanager --install "platform-tools" --sdk_root=../..
./sdkmanager --install "platforms;android-29" --sdk_root=../..
cd ../../../.. \
```

- Exportez la variable $NDK_HOME$ qui pointe vers `android/ndk` : 

``` sh
export NDK_HOME=$PWD/android/ndk
```

- Appliquer le patch nécessaire au fonctionnement de ce projet sur le `ndk` :
> Note : (si vous avez le bon `ndk`, cette commande fonctionne également sous MacOs)
``` sh
make init
```

# Build app
Maintenant aller dans le dossier `app` et effectuez :
- `make app` pour créer l'apk
- `make run` pour installer l'app directement sur votre téléphone si vous l'avez connectez avec `adb`

# Generate Doc
Pour générer la doc vous pouvez effectuer les commandes suivante dans le dossier `app` :
- `make doc` génère la doc dans le dossier `target/armv7-linux-androideabi/doc/`
- `make doc-open` génère la doc et l'ouvre dans votre navigateur par défaut

