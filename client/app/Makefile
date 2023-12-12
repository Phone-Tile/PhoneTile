##
# PhoneTile
#
# @file
# @version 0.1

# A part of this Makefile retake the script from :
# https://github.com/raysan5/raylib/wiki/Working-for-Android-(on-Linux)
# for compile a apk


ABIS=armeabi-v7a x86_64

PREFIX=$(PWD)/..
SYS = $(shell uname | tr A-Z a-z)
NDK_HOME ?=../android/ndk

SDK_VERSION?=29.0.3
BUILD_TOOLS=$(PREFIX)/android/sdk/build-tools/$(SDK_VERSION)
TOOLCHAIN=$(NDK_HOME)/toolchains/llvm/prebuilt/$(SYS)-x86_64
NATIVE_APP_GLUE=$(NDK_HOME)/sources/android/native_app_glue


APP_NAME=PhoneTile

RUST_APP_NAME=phone_tile

PACKAGE_NAME=com.$(RUST_APP_NAME).game

MODE?=debug

ifeq ($(MODE),release)
CARGO_FLAGS += --release
endif


FLAGS="-ffunction-sections -funwind-tables -fstack-protector-strong -fPIC -Wall \
	-Wformat -Werror=format-security -no-canonical-prefixes \
	-DANDROID -DPLATFORM_ANDROID -D__ANDROID_API__=29"

INCLUDES="-I. -Iinclude -I../include -I$NATIVE_APP_GLUE -I$TOOLCHAIN/sysroot/usr/include"


.PHONY: clean

all: app

app: $(RUST_APP_NAME).apk


binaries:
	cargo build --target=aarch64-linux-android   $(CARGO_FLAGS)
	cargo build --target=armv7-linux-androideabi $(CARGO_FLAGS)
	cargo build --target=i686-linux-android      $(CARGO_FLAGS)
	cargo build --target=x86_64-linux-android    $(CARGO_FLAGS)


android/$(RUST_APP_NAME).keystore:
	mkdir -p android
	cd android ; \
	keytool -genkeypair -validity 1000 -dname "CN=$(RUST_APP_NAME),O=Android,C=ES" -keystore $(RUST_APP_NAME).keystore -storepass '$(RUST_APP_NAME)' -keypass '$(RUST_APP_NAME)' -alias projectKey -keyalg RSA ; \
	cd ..


$(RUST_APP_NAME).apk: binaries android/$(RUST_APP_NAME).keystore
	rm -rf apk

	mkdir -p apk/assets

	mkdir -p apk/res/drawable-ldpi
	mkdir -p apk/res/drawable-mdpi
	mkdir -p apk/res/drawable-hdpi
	mkdir -p apk/res/drawable-xhdpi

	cp assets/icon_ldpi.png apk/res/drawable-ldpi/icon.png
	cp assets/icon_mdpi.png apk/res/drawable-mdpi/icon.png
	cp assets/icon_hdpi.png apk/res/drawable-hdpi/icon.png
	cp assets/icon_xhdpi.png apk/res/drawable-xhdpi/icon.png

	cp assets/* apk/assets

# Add resources and assets to APK
	mkdir -p android
	mkdir -p apk/lib/x86_64
	mkdir -p apk/lib/x86
	mkdir -p apk/lib/armeabi-v7a
	mkdir -p apk/lib/arm64-v8a
	cp target/x86_64-linux-android/$(MODE)/lib$(RUST_APP_NAME).so apk/lib/x86_64/lib$(RUST_APP_NAME).so
	cp target/i686-linux-android/$(MODE)/lib$(RUST_APP_NAME).so apk/lib/x86/lib$(RUST_APP_NAME).so
	cp target/aarch64-linux-android/$(MODE)/lib$(RUST_APP_NAME).so apk/lib/arm64-v8a/lib$(RUST_APP_NAME).so
	cp target/armv7-linux-androideabi/$(MODE)/lib$(RUST_APP_NAME).so apk/lib/armeabi-v7a/lib$(RUST_APP_NAME).so

	$(BUILD_TOOLS)/aapt package -f -F temp.apk \
		-S apk/res -M AndroidManifest.xml \
		-A apk/assets \
		-I $(PREFIX)/android/sdk/platforms/android-29/android.jar -v

	unzip -o temp.apk -d apk
	rm -rf apk.apk

	cd apk && zip -D9r ../apk.apk . && zip -D0r ../apk.apk ./resources.arsc ./AndroidManifest.xml

	rm -rf $(RUST_APP_NAME).apk
	#jarsigner -keystore android/$(RUST_APP_NAME).keystore -storepass $(RUST_APP_NAME) -keypass $(RUST_APP_NAME) \
	#	-signedjar $(RUST_APP_NAME).apk $(RUST_APP_NAME).apk projectKey

	$(BUILD_TOOLS)/zipalign -v -f 4 apk.apk $(RUST_APP_NAME).apk
	$(BUILD_TOOLS)/apksigner sign --key-pass pass:$(RUST_APP_NAME) --ks-pass pass:$(RUST_APP_NAME) --ks android/$(RUST_APP_NAME).keystore $(RUST_APP_NAME).apk

	rm -rf temp.apk
	rm -rf apk.apk
	rm -rf apk


ADB=$(PREFIX)/android/sdk/platform-tools/adb

push: $(RUST_APP_NAME).apk
	$(ADB) install -r $(RUST_APP_NAME).apk

run: push
	$(eval ACTIVITYNAME:=$(shell $(BUILD_TOOLS)/aapt dump badging $(RUST_APP_NAME).apk | grep "launchable-activity" | cut -f 2 -d"'"))
	@echo App name $(ACTIVITYNAME)
	$(ADB) shell am start -n com.$(RUST_APP_NAME).game/$(ACTIVITYNAME)

doc:
	cargo doc --target=armv7-linux-androideabi --document-private-items

doc-open:
	cargo doc --target=armv7-linux-androideabi --document-private-items --open

clean:
	cargo clean
	rm -rf $(RUST_APP_NAME).apk

# end
