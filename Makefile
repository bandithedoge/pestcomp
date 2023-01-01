include dpf/Makefile.base.mk

all: libs plugins

libs:
	$(MAKE) -C dpf/dgl

plugins: libs
	$(MAKE) -C plugins/PestComp

clean:
	rm -rf bin build
	$(MAKE) clean -C dpf/dgl
	rm -f \
		dpf-widgets/opengl/*.d \
		dpf-widgets/opengl/*.o \
		dpf-widgets/opengl/DearImGui/*.d \
		dpf-widgets/opengl/DearImGui/*.o \
		imgui_toggle/*.d \
		imgui_toggle/*.o

.PHONY: plugins
