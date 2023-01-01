NAME = PestComp

CXXFLAGS += -I../../dpf-widgets/opengl -I../../dpf-widgets/generic -I../../dpf-widgets/opengl/DearImGui -I../../imgui_toggle

ifeq ($(MACOS),true)
BUILD_CXX_FLAGS += -ObjC++
endif

FILES_DSP = \
	Plugin.cpp

FILES_UI = \
	../../dpf-widgets/opengl/DearImGui.cpp \
	../../imgui_toggle/imgui_toggle.cpp \
	../../imgui_toggle/imgui_toggle_palette.cpp \
	../../imgui_toggle/imgui_toggle_presets.cpp \
	../../imgui_toggle/imgui_toggle_renderer.cpp \
	UI.cpp

include ../../dpf/Makefile.plugins.mk

TARGETS += vst2 vst3 clap

ifeq ($(LINUX),true)
	TARGETS += ladspa lv2_dsp jack
endif

all: $(TARGETS)
