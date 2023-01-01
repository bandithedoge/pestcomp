#include "DistrhoPluginInfo.h"
#include "DistrhoUI.hpp"

#include "generated/OldeEnglish.h"
#include "imgui.h"
#include "imgui_toggle.h"

START_NAMESPACE_DISTRHO

class PestCompUI : public UI {
  public:
    PestCompUI() : UI(DISTRHO_UI_DEFAULT_WIDTH, DISTRHO_UI_DEFAULT_HEIGHT) {
        setGeometryConstraints(DISTRHO_UI_DEFAULT_WIDTH,
                               DISTRHO_UI_DEFAULT_HEIGHT, true);
    }

  protected:
    void parameterChanged(uint32_t index, float value) override {
        switch (index) {
        case paramEngage:
            engage = value == 1.0f;
        }
        repaint();
    }

    void onImGuiDisplay() override {
        const float width = getWidth();
        const float height = getHeight();

        ImGui::PushStyleVar(ImGuiStyleVar_WindowBorderSize, 0);

        ImGui::SetNextWindowPos(ImVec2(0, 0));
        ImGui::SetNextWindowSize(ImVec2(width, height));

        if (ImGui::Begin(DISTRHO_PLUGIN_NAME, nullptr,
                         ImGuiWindowFlags_NoResize |
                             ImGuiWindowFlags_NoDecoration |
                             ImGuiWindowFlags_NoScrollWithMouse)) {
            // plugin logo
            ImGui::PushFont(font_big);
            ImGui::Text(DISTRHO_PLUGIN_NAME);
            ImGui::PopFont();

            // engage switch
            ImGui::SameLine(width - 250);
            ImGui::PushFont(font);
            ImGui::SetCursorPosY((height * 0.5) - ImGui::GetTextLineHeight());
            if (ImGui::Toggle("Engage", &engage, ImGuiToggleFlags_Animated)) {
                if (ImGui::IsItemActivated()) {
                    editParameter(paramEngage, true);
                };
                setParameterValue(paramEngage, engage);
            }
            if (ImGui::IsItemDeactivated())
                editParameter(paramEngage, false);
            ImGui::PopFont();

            // copyright
            ImGui::NewLine();
            ImGui::SetCursorPosY(height - 60);
            ImGui::PushFont(font_small);
            ImGui::TextDisabled("%s \u00A9 2023", DISTRHO_PLUGIN_BRAND);
            ImGui::PopFont();
        }
        ImGui::End();
    };

  private:
    ImGuiIO &io = ImGui::GetIO();
    ImFont *font = io.Fonts->AddFontFromMemoryCompressedTTF(
        OldeEnglish_compressed_data, OldeEnglish_compressed_size, 40.0f);
    ImFont *font_big = io.Fonts->AddFontFromMemoryCompressedTTF(
        OldeEnglish_compressed_data, OldeEnglish_compressed_size, 150.0f);
    ImFont *font_small = io.Fonts->AddFontFromMemoryCompressedTTF(
        OldeEnglish_compressed_data, OldeEnglish_compressed_size, 30.0f);

    bool engage = false;

    DISTRHO_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(PestCompUI)
};

UI *createUI() { return new PestCompUI(); }

END_NAMESPACE_DISTRHO
