# Exports rules to build `kernel.o`, to be linked with platform specific code to
# compile a binary image. For convenience,Also exposes rules to 

CORE_SOURCES=$(call rwildcard,$(SRC_DIR)main/,*.rs)
MAIN_DEPS=$(BUILD_PLATFORM_DIR)/libcore.rlib $(BUILD_PLATFORM_DIR)/libsupport.rlib $(CORE_SOURCES)
MAIN_DEPS+=$(BUILD_PLATFORM_DIR)/libplatform.rlib $(BUILD_PLATFORM_DIR)/libprocess.rlib

$(BUILD_PLATFORM_DIR)/kernel.o: $(MAIN_DEPS) | $(BUILD_PLATFORM_DIR)
	@echo "Building $@"
	@$(RUSTC) $(RUSTC_FLAGS) -C lto --emit obj -o $@ $(SRC_DIR)main/main.rs
	@$(OBJDUMP) $(OBJDUMP_FLAGS) $@ > $(BUILD_PLATFORM_DIR)/kernel.lst

$(BUILD_PLATFORM_DIR)/kernel.S: $(MAIN_DEPS) | $(BUILD_PLATFORM_DIR)
	@echo "Building $@"
	@$(RUSTC) $(RUSTC_FLAGS) -C lto --emit asm -o $@ $(SRC_DIR)main/main.rs

$(BUILD_PLATFORM_DIR)/kernel.ir: $(MAIN_DEPS) | $(BUILD_PLATFORM_DIR)
	@echo "Building $@"
	@$(RUSTC) $(RUSTC_FLAGS) -C lto --emit llvm-ir -o $@ $(SRC_DIR)main/main.rs

$(BUILD_PLATFORM_DIR)/kernel.elf: $(BUILD_PLATFORM_DIR)/kernel.o $(BUILD_PLATFORM_DIR)/ctx_switch.o
	@echo "Building $@"
	@$(CC) $^ $(CFLAGS) $(LDFLAGS) -o $@

SLOAD=sload
SDB_MAINTAINER=$(shell whoami)
SDB_VERSION=$(shell git show-ref -s HEAD)
SDB_NAME=storm.rs
SDB_DESCRIPTION="An OS for the storm"

$(BUILD_PLATFORM_DIR)/kernel.sdb: $(BUILD_PLATFORM_DIR)/kernel.elf
	@echo "Building $@"
	@$(SLOAD) pack -m "$(SDB_MAINTAINER)" -v "$(SDB_VERSION)" -n "$(SDB_NAME)" -d $(SDB_DESCRIPTION) -o $@ $<