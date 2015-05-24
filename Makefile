# Toolchain
OBJCOPY=arm-none-eabi-objcopy
OBJDUMP=arm-none-eabi-objdump

# Target
TARGET=thumbv7m-none-eabi

# Files
OUT_DIR = target/$(TARGET)/release
BINARIES = \
	$(OUT_DIR)/blink \
	$(OUT_DIR)/rgb

.PHONY: clean listing $(BINARIES)

all: build listing
build: $(patsubst %,%.bin,$(BINARIES))
listing: $(patsubst %,%.lst,$(BINARIES))

$(BINARIES):
	cargo build --release --target=$(TARGET) --verbose

%.bin: %
	$(OBJCOPY) -O binary $< $@

%.lst: %
	$(OBJDUMP) -D $< > $@

clean:
	cargo clean
