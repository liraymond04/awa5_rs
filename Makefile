# Directory containing source files
SRCDIR = examples/lib

# Output directory for shared libraries
OUTDIR = examples/lib

# Find all C source files in the source directory
SRCS = $(wildcard $(SRCDIR)/*.c)

# Generate the corresponding object files
OBJS = $(SRCS:.c=.o)

# Generate the corresponding shared library names
LIBS = $(patsubst $(SRCDIR)/%.c,$(OUTDIR)/lib%.so,$(SRCS))

# Compiler and flags
CC = gcc
CFLAGS = -Wall -fPIC
LDFLAGS = -shared

# Default target to build all shared libraries
all: $(LIBS)

# Rule to build each shared library
$(OUTDIR)/lib%.so: $(SRCDIR)/%.c
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $<

# Clean target to remove generated files
clean:
	rm -f $(OUTDIR)/lib*.so $(OBJS)

.PHONY: all clean
