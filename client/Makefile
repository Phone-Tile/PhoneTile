##
# Setup PhoneTile
#
# @file
# @version 0.1

NDK_HOME?=$(PWD)/android/ndk
SYS = $(shell uname | tr A-Z a-z)

check_init:
	@echo "Attention vous êtes sur le point de modifier le ndk"
	@echo "Vérifier bien que la variable NDK_HOME pointe sur le bon dossier"
	@echo "Et non sur un NDK installé globalement sur votre système"
	@echo -n "Êtes vous sur de vouloir modifier le ndk ? [y/N] " && read ans && [ $${ans:-N} = y ]

init: check_init
	grep "ANativeActivity_onCreate_C" $(NDK_HOME)/sources/android/native_app_glue/android_native_app_glue.c \
	|| sed -i s/ANativeActivity_onCreate/ANativeActivity_onCreate_C/g $(NDK_HOME)/sources/android/native_app_glue/android_native_app_glue.c
	grep "ANativeActivity_onCreate_C" $(NDK_HOME)/toolchains/llvm/prebuilt/$(SYS)-x86_64/sysroot/usr/include/android/native_activity.h \
	|| sed -i s/ANativeActivity_onCreate/ANativeActivity_onCreate_C/g $(NDK_HOME)/toolchains/llvm/prebuilt/$(SYS)-x86_64/sysroot/usr/include/android/native_activity.h
	@echo ""
	@echo "PATCH fini !"


# end
