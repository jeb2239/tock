$(BUILD_DIR)/arch.o: $(SRC_DIR)arch/$(ARCH)/ctx_switch.S | $(BUILD_DIR)
	@$(TOOLCHAIN)as -mcpu=cortex-m4 -mthumb $^ -o $@

$(BUILD_DIR)/libcortexm4.rlib: $(call rwildcard,$(SRC_DIR)arch/cortex-m4,*.rs) $(BUILD_DIR)/libcore.rlib
	@echo "Building $@"
	@$(RUSTC) $(RUSTC_FLAGS) --out-dir $(BUILD_DIR) $(SRC_DIR)arch/cortex-m4/lib.rs

