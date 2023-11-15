# Client



# Setup of this folder :

Install [tools for raylib](https://github.com/raysan5/raylib/wiki/Working-on-GNU-Linux)



Download [Android sdk command-line tool](https://developer.android.com/studio/#command-tools)

Unzip, and put in `android/sdk`

Execute :
```bash
cd android/sdk/cmdline-tools/bin
./sdkmanager --update --sdk_root=../..
./sdkmanager --install "build-tools;29.0.3" --sdk_root=../..
./sdkmanager --install "platform-tools" --sdk_root=../..
./sdkmanager --install "platforms;android-29" --sdk_root=../..
cd ../../../..
```
Download [Android NDK, version r21e](https://dl.google.com/android/repository/android-ndk-r21e-linux-x86_64.zip)

Unzip and put in `android/ndk`


Run : 
```bash
cd raylib/src
cp raylib.h ../../include
make clean
make PLATFORM=PLATFORM_ANDROID ANDROID_NDK=../../android/ndk ANDROID_ARCH=arm ANDROID_API_VERSION=29
mv libraylib.a ../../lib/armeabi-v7a
make clean
make PLATFORM=PLATFORM_ANDROID ANDROID_NDK=../../android/ndk ANDROID_ARCH=arm64 ANDROID_API_VERSION=29
mv libraylib.a ../../lib/arm64-v8a
make clean
make PLATFORM=PLATFORM_ANDROID ANDROID_NDK=../../android/ndk ANDROID_ARCH=x86 ANDROID_API_VERSION=29
mv libraylib.a ../../lib/x86
make clean
make PLATFORM=PLATFORM_ANDROID ANDROID_NDK=../../android/ndk ANDROID_ARCH=x86_64 ANDROID_API_VERSION=29
mv libraylib.a ../../lib/x86_64
make clean
cd ../..
```

Run : 
```bash cd android
keytool -genkeypair -validity 1000 -dname "CN=raylib,O=Android,C=ES" -keystore raylib.keystore -storepass 'raylib' -keypass 'raylib' -alias projectKey -keyalg RSA
cd ..
```


Now execute `./build.sh`


See [raylib doc](https://github.com/raysan5/raylib/wiki/Working-for-Android-(on-Linux))

