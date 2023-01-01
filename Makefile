include dpf/Makefile.base.mk

PREFIX ?= /usr/local

all: libs plugins

libs:
	$(MAKE) -C dpf/dgl

plugins: libs
	$(MAKE) -C plugins/PestComp

install: all

	install -d $(PREFIX)/lib/vst
	install -m755 bin/PestComp-vst.so $(PREFIX)/lib/vst/PestComp.so

	install -d $(PREFIX)/lib/clap
	install -m755 bin/PestComp.clap $(PREFIX)/lib/clap/PestComp.clap

	install -d $(PREFIX)/lib/vst3
	cp -rL bin/PestComp.vst3 $(PREFIX)/lib/vst3/

ifeq ($(LINUX),true)
	install -d $(PREFIX)/bin
	install -m755 bin/PestComp $(PREFIX)/bin/PestComp

	install -d $(PREFIX)/lib/ladspa
	install -m755 bin/PestComp-ladspa.so $(PREFIX)/lib/ladspa/PestComp.so

	install -d $(PREFIX)/lib/lv2
	install -m755 bin/PestComp.lv2/PestComp_dsp.so $(PREFIX)/lib/lv2/PestComp.so
endif

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
