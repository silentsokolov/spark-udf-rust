.PHONY: rust_test rust_build rust_build_linux copy_rust_to_java java_package java_compile copy_jar build build_cross_linux

UNAME := $(shell uname -s)
ifeq ($(UNAME),Darwin)
    LIBJVM_NAME := libjli
    LIB_EXT := dylib
    LIB_TYPE := darwin-amd64
    SED_OPT := -i ''
else
	LIBJVM_NAME := libjvm
	LIB_EXT := so
	LIB_TYPE := linux-amd64
	SED_OPT := 
endif

RUST_VERSION := 1.52.1
CURRECT_DIR := $(shell pwd)

JAVA_HOME := $(shell java -XshowSettings:properties -version 2>&1 > /dev/null | grep 'java.home' | awk '{print $$3}')
LIBJVM_PATH := $(shell find "$(JAVA_HOME)" -type f -name "${LIBJVM_NAME}.*" -print0 -quit | xargs -0 -n1 dirname)
JAVA_SIDE_PATH := $(shell find "$(CURRECT_DIR)" -type f -name "pom.xml" -print0 -quit | xargs -0 -n1 dirname)

RUST_SIDE_PATH := $(shell find "$(CURRECT_DIR)" -type f -name "Cargo.toml" -print0 -quit | xargs -0 -n1 dirname)
RUST_LIB_NAME := $(shell basename "$(RUST_SIDE_PATH)")
PROJECT_NAME := $(shell basename "$(CURRECT_DIR)")
DIST_DIR := $(CURRECT_DIR)/dist

rust_test:
	LD_LIBRARY_PATH=$$LD_LIBRARY_PATH:$(LIBJVM_PATH) cargo test --manifest-path=$(RUST_SIDE_PATH)/Cargo.toml

rust_build:
	LD_LIBRARY_PATH=$$LD_LIBRARY_PATH:$(LIBJVM_PATH) cargo build --release --manifest-path=$(RUST_SIDE_PATH)/Cargo.toml

rust_build_linux:
	sed $(SED_OPT) 's/\["invocation"\]/\[\]/g' $(RUST_SIDE_PATH)/Cargo.toml
	-docker run --rm --user "$(shell id -u)":"$(shell id -g)" -v "$(RUST_SIDE_PATH)":/usr/src/myapp -w /usr/src/myapp rust:$(RUST_VERSION) cargo build --release --target=x86_64-unknown-linux-gnu
	sed $(SED_OPT) 's/\[\]/\["invocation"\]/g' $(RUST_SIDE_PATH)/Cargo.toml

copy_rust_to_java:
	-mv -f "$(RUST_SIDE_PATH)/target/release/lib$(RUST_LIB_NAME).$(LIB_EXT)" "$(JAVA_SIDE_PATH)/src/main/resources/libs/$(LIB_TYPE).$(LIB_EXT)"
	-mv -f "$(RUST_SIDE_PATH)/target/x86_64-unknown-linux-gnu/release/lib$(RUST_LIB_NAME).so" "$(JAVA_SIDE_PATH)/src/main/resources/libs/linux-amd64.so"

java_package:
	mvn -f $(JAVA_SIDE_PATH)/pom.xml package

java_compile:
	mvn -f $(JAVA_SIDE_PATH)/pom.xml clean compile

copy_jar:
	$(eval jarfile := $(shell find "$(JAVA_SIDE_PATH)/target" -type f -name "*.jar" -print0 -quit | xargs -0 -n1 basename))
	mkdir -p "$(DIST_DIR)"
	mv -f "$(JAVA_SIDE_PATH)/target/$(jarfile)" "$(DIST_DIR)/$(PROJECT_NAME).jar"

build: rust_build copy_rust_to_java java_package copy_jar

build_cross_linux: rust_build_linux copy_rust_to_java java_package copy_jar
