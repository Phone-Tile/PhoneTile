##
# PhoneTile
#
# @file
# @version 0.1

# A part of this Makefile retake the script from :
# https://github.com/raysan5/raylib/wiki/Working-for-Android-(on-Linux)
# for compile a apk


ABIS=armeabi-v7a x86_64

PREFIX=..

SDK_VERSION?=29.0.3
BUILD_TOOLS=$(PREFIX)/android/sdk/build-tools/$(SDK_VERSION)
TOOLCHAIN=$(PREFIX)/android/ndk/toolchains/llvm/prebuilt/linux-x86_64
NATIVE_APP_GLUE=$(PREFIX)/android/ndk/sources/android/native_app_glue_modified

FLAGS="-ffunction-sections -funwind-tables -fstack-protector-strong -fPIC -Wall \
	-Wformat -Werror=format-security -no-canonical-prefixes \
	-DANDROID -DPLATFORM_ANDROID -D__ANDROID_API__=29"

INCLUDES="-I. -Iinclude -I../include -I$NATIVE_APP_GLUE -I$TOOLCHAIN/sysroot/usr/include"

CC="$TOOLCHAIN/bin/${CCTYPE}29-clang"

APP_NAME=PhoneTile

RUST_APP_NAME=phone_tile

MODE=debug

NDK_HOME ?=../android/ndk 


.PHONY: clean

all: app

app: $(RUST_APP_NAME).apk


binaries:
	#cargo build --target=aarch64-linux-android
	cargo build --target=armv7-linux-androideabi
	#cargo build --target=i686-linux-android
	cargo build	--target=x86_64-linux-android


android/$(RUST_APP_NAME).keystore:
	cd android ; \
	keytool -genkeypair -validity 1000 -dname "CN=$(RUST_APP_NAME),O=Android,C=ES" -keystore $(RUST_APP_NAME).keystore -storepass '$(RUST_APP_NAME)' -keypass '$(RUST_APP_NAME)' -alias projectKey -keyalg RSA ; \
	cd ..


$(RUST_APP_NAME).apk: binaries android/$(RUST_APP_NAME).keystore

	cp assets/icon_ldpi.png android/build/res/drawable-ldpi/icon.png
	cp assets/icon_mdpi.png android/build/res/drawable-mdpi/icon.png
	cp assets/icon_hdpi.png android/build/res/drawable-hdpi/icon.png
	cp assets/icon_xhdpi.png android/build/res/drawable-xhdpi/icon.png

	cp assets/* android/build/assets

	$(BUILD_TOOLS)/aapt package -f -m \
		-S android/build/res -J android/build/src -M android/build/AndroidManifest.xml \
		-I $(PREFIX)/android/sdk/platforms/android-29/android.jar

# Compile NativeLoader.java
	javac -verbose -source 1.8 -target 1.8 -d android/build/obj \
		-bootclasspath jre/lib/rt.jar \
		-classpath $(PREFIX)/android/sdk/platforms/android-29/android.jar:android/build/obj \
		-sourcepath src android/build/src/com/$(RUST_APP_NAME)/game/R.java \
		android/build/src/com/$(RUST_APP_NAME)/game/NativeLoader.java

	$(BUILD_TOOLS)/dx --verbose --dex --output=android/build/dex/classes.dex android/build/obj

# Add resources and assets to APK
	$(BUILD_TOOLS)/aapt package -f \
		-M android/build/AndroidManifest.xml -S android/build/res -A assets \
		-I $(PREFIX)/android/sdk/platforms/android-29/android.jar -F $(RUST_APP_NAME).apk android/build/dex
	mkdir -p android
	cp target/x86_64-linux-android/$(MODE)/lib$(RUST_APP_NAME).so android/build/lib/x86_64/lib$(RUST_APP_NAME).so
	#cp target/x86-linux-android/$(MODE)/lib$(RUST_APP_NAME).so android/build/lib/x86/lib$(RUST_APP_NAME).so
	cp target/armv7-linux-androideabi/$(MODE)/lib$(RUST_APP_NAME).so android/build/lib/armeabi-v7a/lib$(RUST_APP_NAME).so

# Add libraries to APK
	cd android/build ; \
	../../$(BUILD_TOOLS)/aapt add ../../$(RUST_APP_NAME).apk lib/armeabi-v7a/lib$(RUST_APP_NAME).so ; \
	../../$(BUILD_TOOLS)/aapt add ../../$(RUST_APP_NAME).apk lib/x86_64/lib$(RUST_APP_NAME).so ; \
	cd ../..

	jarsigner -keystore android/$(RUST_APP_NAME).keystore -storepass $(RUST_APP_NAME) -keypass $(RUST_APP_NAME) \
		-signedjar $(RUST_APP_NAME).apk $(RUST_APP_NAME).apk projectKey

	$(BUILD_TOOLS)/zipalign -f 4 $(RUST_APP_NAME).apk $(RUST_APP_NAME).final.apk
	mv -f $(RUST_APP_NAME).final.apk $(RUST_APP_NAME).apk

run: $(RUST_APP_NAME).apk
	$(PREFIX)/android/sdk/platform-tools/adb install -r $(RUST_APP_NAME).apk

clean:
	cargo clean

# end
