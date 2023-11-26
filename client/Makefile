##
# Setup PhoneTile
#
# @file
# @version 0.1

ifndef NDK_HOME
	NDK_HOME_NOT_DEFINE=true
endif

NDK_HOME?=$(PWD)/android/ndk
NDK_VERSION=r26b
SYS = $(shell uname | tr A-Z a-z)

check_init:
ifdef NDK_HOME_NOT_DEFINE
	@echo -e "\e[1;33mWarning : NDK_HOME is not set, execute export NDK_HOME=\$$PWD/android/ndk \e[0m"
	@echo "Attention vous êtes sur le point de modifier le ndk"
else
	@echo "Attention vous êtes sur le point de modifier le ndk"
	@echo "Vérifier bien que la variable NDK_HOME pointe sur le bon dossier, NDK_HOME=$$NDK_HOME"
	@echo "Et non sur un NDK installé globalement sur votre système"
endif
	@echo -n "Êtes vous sur de vouloir modifier le ndk ? [y/N] " && read ans && [ $${ans:-N} = y ]

ifdef NDK_HOME_NOT_DEFINE
init: android/sdk android/ndk check_init
else
init: android/sdk check_init
endif
	$(info Début de la modification du ndk)
	grep "ANativeActivity_onCreate_C" $(NDK_HOME)/sources/android/native_app_glue/android_native_app_glue.c \
	|| sed -i s/ANativeActivity_onCreate/ANativeActivity_onCreate_C/g $(NDK_HOME)/sources/android/native_app_glue/android_native_app_glue.c
	grep "ANativeActivity_onCreate_C" $(NDK_HOME)/toolchains/llvm/prebuilt/$(SYS)-x86_64/sysroot/usr/include/android/native_activity.h \
	|| sed -i s/ANativeActivity_onCreate/ANativeActivity_onCreate_C/g $(NDK_HOME)/toolchains/llvm/prebuilt/$(SYS)-x86_64/sysroot/usr/include/android/native_activity.h
	@echo ""
	@echo "PATCH fini !"


android/sdk:
	@echo "Le sdk n'a pas été détecté dans le dossier client/android/sdk"
	@echo "Ce dernier va être téléchargé automatiquement"
	@echo -n "Acceptez-vous ? [y/N] " && read ans && [ $${ans:-N} = y ]
	mkdir -p android
	wget "https://dl.google.com/android/repository/commandlinetools-linux-10406996_latest.zip"
	unzip commandlinetools-linux-10406996_latest.zip -d android/sdk
	rm commandlinetools-linux-10406996_latest.zip
	cd android/sdk/cmdline-tools/bin; \
		./sdkmanager --update --sdk_root=../.. ; \
		./sdkmanager --install "build-tools;29.0.3" --sdk_root=../.. ; \
		./sdkmanager --install "platform-tools" --sdk_root=../.. ; \
		./sdkmanager --install "platforms;android-29" --sdk_root=../.. ; \
	cd ../../../.. \


android/ndk:
	@echo "La variable NDK_HOME n'a pas été set et le ndk ne semble pas être dans $(PWD)/android/ndk"
	@echo "ce qui laisse penser que le ndk n'est pas installé"
	@echo "Ce dernier va être téléchargé automatiquement"
	@echo -n "Acceptez-vous ? [y/N] " && read ans && [ $${ans:-N} = y ]
	wget "https://dl.google.com/android/repository/android-ndk-$(NDK_VERSION)-linux.zip"
	mkdir -p android
	unzip android-ndk-$(NDK_VERSION)-linux.zip
	mv android-ndk-$(NDK_VERSION) android/ndk
	rm android-ndk-$(NDK_VERSION)-linux.zip



# end
