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


Télécharger le ndk et sdk android.
Mettre le ndk dans le dossier `android/ndk`
Mettre le sdk dans le dossier `android/sdk`

Quand vous êtes dans le doosier `client` (dossier où se trouve ce `README.md`),
exporter une variable global nommé `NDK_HOME` qui pointe vers `android/ndk` :

``` sh
export NDK_HOME=$PWD/android/ndk
```


Et maintenant la partie un peu drôle qui permet de fonctionner.


# Modification ANativeActivity_onCreate
## avec la ligne de commande
Effectuez un `make init`

## A la main
Il faut modifier le fichier `android/ndk/sources/android/native_app_glue/android_native_app_glue.c` `android/ndk/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/android/native_activity.h` 
et modifier le nom de la fonction `ANativeActivity_onCreate` par `ANativeActivity_onCreate_C`.

# Modification du compilateur
Ah et j'oubliais, une dernière partie un peu drole c'est que les scripts pour compiler sont faux donc il faut modifier le fichier 
`android/ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/clang`, afin d'avoir `clang-17 $@`


# Build app
Maintenant aller dans le dossier `app` et effectuez :
- `make app` pour créer l'apk
- `make run` pour installer l'app directement sur votre téléphone si vous l'avez connectez avec `adb`


