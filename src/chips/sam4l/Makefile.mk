ARCH = cortex-m4

RUST_TARGET ?= $(SRC_DIR)chips/sam4l/target.json

RUSTC_FLAGS += -C opt-level=3 -Z no-landing-pads
RUSTC_FLAGS += --target $(RUST_TARGET)
RUSTC_FLAGS += -Ctarget-cpu=cortex-m4 -C relocation_model=static
RUSTC_FLAGS += -g -C no-stack-check

CFLAGS += -g -O3 -std=gnu99 -mcpu=cortex-m4 -mthumb -nostdlib -T$(SRC_DIR)chips/sam4l/loader.ld
LDFLAGS += -mcpu=cortex-m4 -mthumb
LOADER = $(SRC_DIR)chips/sam4l/loader.ld

ARCH = cortex-m4

$(BUILD_DIR)/libsam4l.rlib: $(call rwildcard,$(SRC_DIR)chips/sam4l,*.rs) $(BUILD_DIR)/libcore.rlib $(BUILD_DIR)/libhil.rlib $(BUILD_DIR)/libcommon.rlib $(BUILD_DIR)/libcortexm4.rlib
	@echo "Building $@"
	@$(RUSTC) $(RUSTC_FLAGS) --out-dir $(BUILD_DIR) $(SRC_DIR)chips/sam4l/lib.rs

$(BUILD_DIR)/crt1.o: $(SRC_DIR)chips/sam4l/crt1.c | $(BUILD_DIR)
	@echo "Building $@"
	@$(CC) $(CFLAGS) -c $< -o $@ -lc -lgcc

